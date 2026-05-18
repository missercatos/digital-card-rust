use macroquad::prelude::*;
use std::f32::consts::PI;

const SPRING_K: f32 = 0.07;
const DAMPING: f32 = 0.86;
const REPULSION_RADIUS: f32 = 180.0;
const REPULSION_FORCE: f32 = 1100.0;
const CHAR_SIZE: f32 = 21.0;

#[derive(Clone)]
struct Particle {
    x: f32,
    y: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
    ch: char,
    brightness: f32,
}

struct Skill {
    text: String,
    x: f32,
    y: f32,
    bx: f32,
    by: f32,
    phase: f32,
    speed: f32,
    amp_x: f32,
    amp_y: f32,
}

fn load_ascii_art_from_file(path: &str) -> Option<Vec<Vec<(f32, char)>>> {
    let content = std::fs::read_to_string(path).ok()?;
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return None;
    }

    let h = lines.len();
    let w = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
    if w == 0 || h > 300 || w > 300 {
        return None;
    }

    let mut density_grid = vec![vec![0.0f32; w]; h];
    let mut char_grid = vec![vec![' '; w]; h];

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let density = match ch {
                '@' | '$' | '&' | '#' | '%' => 1.0,
                '8' | 'B' | 'M' | 'W' | 'N' => 0.90,
                '0' | 'Q' | 'O' | 'G' | 'D' => 0.80,
                '*' | '=' | '+' | 'X' | 'Z' => 0.70,
                'o' | 'a' | 'h' | 'k' | 'b' => 0.55,
                'd' | 'p' | 'q' | 'w' | 'm' => 0.45,
                '?' | '!' | ';' | '|' | '/' => 0.35,
                '-' | '~' | ':' | '^' | ',' => 0.25,
                '`' | '\'' | '.' | ' ' => 0.10,
                _ => 0.8,
            };
            if ch != ' ' {
                density_grid[y][x] = density;
                char_grid[y][x] = ch;
            }
        }
    }

    let mut blurred = vec![vec![0.0f32; w]; h];
    for y in 0..h {
        for x in 0..w {
            let mut sum = 0.0f32;
            let mut cnt = 0i32;
            for dy in -1isize..=1 {
                for dx in -1isize..=1 {
                    let ny = y as isize + dy;
                    let nx = x as isize + dx;
                    if ny >= 0 && ny < h as isize && nx >= 0 && nx < w as isize {
                        sum += density_grid[ny as usize][nx as usize];
                        cnt += 1;
                    }
                }
            }
            blurred[y][x] = (sum / cnt as f32).max(density_grid[y][x]);
        }
    }

    let mut result = vec![vec![(0.0f32, ' '); w]; h];
    for y in 0..h {
        for x in 0..w {
            result[y][x] = (blurred[y][x], char_grid[y][x]);
        }
    }

    Some(result)
}

fn density_to_dot_brightness(d: f32) -> f32 {
    d.clamp(0.0, 1.0)
}

fn dot_color(brightness: f32) -> Color {
    let t = brightness.clamp(0.0, 1.0);
    let g = 0.30 + t * 0.70;
    Color::new(0.06 + t * 0.30, g, 0.06 + t * 0.50, 1.0)
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Digital Card".to_owned(),
        window_width: 1300,
        window_height: 850,
        window_resizable: true,
        high_dpi: true,
        ..Default::default()
    }
}

async fn load_cjk_font() -> Option<Font> {
    let paths = [
        "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/noto-cjk/NotoSansCJK-Light.ttc",
        "/usr/share/fonts/noto-cjk/NotoSansCJK-Bold.ttc",
        "/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc",
        "/usr/share/fonts/truetype/droid/DroidSansFallbackFull.ttf",
        "/System/Library/Fonts/PingFang.ttc",
        "C:\\Windows\\Fonts\\msyh.ttc",
        "C:\\Windows\\Fonts\\simsun.ttc",
    ];
    for p in &paths {
        if let Ok(font) = load_ttf_font(p).await {
            return Some(font);
        }
    }
    None
}

#[macroquad::main(window_conf)]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = if args.len() > 1 {
        args[1].clone()
    } else {
        "art.txt".to_string()
    };

    let cjk_font = load_cjk_font().await;

    let (art_grid, art_label) = match load_ascii_art_from_file(&path) {
        Some(grid) => (grid, path),
        None => (
            vec![vec![(0.0f32, ' '); 0]; 0],
            String::new(),
        ),
    };

    let grid_h = art_grid.len();
    let grid_w = if grid_h > 0 { art_grid[0].len() } else { 0 };

    let sx = CHAR_SIZE * 0.33;
    let sy = CHAR_SIZE * 0.85;
    let art_w = grid_w as f32 * sx;
    let art_h = grid_h as f32 * sy;
    let art_start_x = screen_width() / 2.0 - art_w / 2.0;
    let art_start_y = screen_height() / 2.0 - art_h / 2.0;

    let mut particles: Vec<Particle> = Vec::new();

    for gy in 0..grid_h {
        for gx in 0..grid_w {
            let (d, ch) = art_grid[gy][gx];
            if d > 0.02 && ch != ' ' {
                let tx = art_start_x + gx as f32 * sx;
                let ty = art_start_y + gy as f32 * sy;
                let rx = macroquad::rand::gen_range(0.0, screen_width());
                let ry = macroquad::rand::gen_range(0.0, screen_height());

                particles.push(Particle {
                    x: rx,
                    y: ry,
                    tx,
                    ty,
                    vx: macroquad::rand::gen_range(-100.0, 100.0),
                    vy: macroquad::rand::gen_range(-100.0, 100.0),
                    ch,
                    brightness: density_to_dot_brightness(d),
                });
            }
        }
    }

    let skill_names: &[&str] = &[
        "Rust", "Python", "ML / AI", "Web Dev",
        "Linux", "Docker", "Git", "SQL",
        "Cloud", "OSS", "Go", "React",
        "C++", "K8s", "CI/CD", "API",
        "", "", "", "",
    ];

    let mut skills: Vec<Skill> = Vec::new();
    let n = skill_names.len() as f32;

    for (i, name) in skill_names.iter().enumerate() {
        let angle = (i as f32 / n) * 2.0 * PI - PI / 2.0;
        let r = (screen_width().min(screen_height()) * 0.38) as f32;
        let bx = screen_width() / 2.0 + angle.cos() * r;
        let by = screen_height() / 2.0 + angle.sin() * r;

        skills.push(Skill {
            text: name.to_string(),
            x: bx,
            y: by,
            bx,
            by,
            phase: macroquad::rand::gen_range(0.0, 2.0 * PI),
            speed: macroquad::rand::gen_range(0.18, 0.50),
            amp_x: macroquad::rand::gen_range(8.0, 24.0),
            amp_y: macroquad::rand::gen_range(5.0, 16.0),
        });
    }

    loop {
        let dt = get_frame_time().min(0.05);
        let time = get_time() as f32;
        let (mx, my) = mouse_position();
        let mouse_down = is_mouse_button_down(MouseButton::Left);

        for p in particles.iter_mut() {
            let fx = (p.tx - p.x) * SPRING_K;
            let fy = (p.ty - p.y) * SPRING_K;

            if mouse_down {
                let dx = p.x - mx;
                let dy = p.y - my;
                let dist_sq = dx * dx + dy * dy;
                if dist_sq < REPULSION_RADIUS * REPULSION_RADIUS && dist_sq > 0.001 {
                    let dist = dist_sq.sqrt();
                    let t = 1.0 - dist / REPULSION_RADIUS;
                    let force = t * t * REPULSION_FORCE;
                    let nx = dx / dist;
                    let ny = dy / dist;
                    p.vx += nx * force * dt;
                    p.vy += ny * force * dt;
                }
            }

            p.vx += fx * dt * 60.0;
            p.vy += fy * dt * 60.0;
            p.vx *= DAMPING;
            p.vy *= DAMPING;
            p.x += p.vx * dt;
            p.y += p.vy * dt;
        }

        for s in skills.iter_mut() {
            s.x = s.bx + (time * s.speed + s.phase).sin() * s.amp_x;
            s.y = s.by + (time * s.speed * 0.7 + s.phase + 1.2).cos() * s.amp_y;
        }

        clear_background(Color::new(0.03, 0.03, 0.09, 1.0));

        let grid_spacing = 45.0;
        let grid_color = Color::new(0.05, 0.05, 0.13, 0.28);
        let grid_thickness = 0.35;
        let w = screen_width();
        let h = screen_height();
        let cols = (w / grid_spacing) as i32 + 1;
        let rows = (h / grid_spacing) as i32 + 1;

        for i in 0..cols {
            let x = i as f32 * grid_spacing;
            draw_line(x, 0.0, x, h, grid_thickness, grid_color);
        }
        for i in 0..rows {
            let y = i as f32 * grid_spacing;
            draw_line(0.0, y, w, y, grid_thickness, grid_color);
        }

        for p in particles.iter() {
            let base = dot_color(p.brightness);

            let dx = p.x - mx;
            let dy = p.y - my;
            let dist = (dx * dx + dy * dy).sqrt();
            let glow = if mouse_down && dist < REPULSION_RADIUS * 1.3 {
                1.0 + (1.0 - dist / (REPULSION_RADIUS * 1.3)) * 0.8
            } else {
                1.0
            };

            let color = Color::new(
                (base.r * glow).min(1.0),
                (base.g * glow).min(1.0),
                (base.b * glow).min(1.0),
                base.a,
            );

            draw_text(&p.ch.to_string(), p.x, p.y, CHAR_SIZE, color);
        }

        for s in skills.iter() {
            if s.text.is_empty() {
                continue;
            }
            let alpha = 0.38 + 0.14 * (time * s.speed * 1.2).sin();
            let color = Color::new(0.10, 0.72, 0.85, alpha);
            let params = TextParams {
                font: cjk_font.as_ref(),
                font_size: 21,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
                rotation: 0.0,
                color,
            };
            let dims = measure_text(&s.text, cjk_font.as_ref(), 21, 1.0);
            draw_text_ex(
                &s.text,
                s.x - dims.width / 2.0,
                s.y - dims.height / 2.0,
                params,
            );
        }

        let hint = if cfg!(target_os = "android") {
            "Touch & hold to interact"
        } else {
            "Click & hold to interact"
        };
        let hdims = measure_text(hint, None, 18, 1.0);
        draw_text(
            hint,
            w / 2.0 - hdims.width / 2.0,
            h - 33.0,
            18.0,
            Color::new(0.28, 0.28, 0.48, 0.48),
        );

        let label = art_label.as_str();
        let sdims = measure_text(label, None, 18, 1.0);
        draw_text(
            label,
            w / 2.0 - sdims.width / 2.0,
            art_start_y + art_h + 45.0,
            18.0,
            Color::new(0.22, 0.52, 0.62, 0.45),
        );

        next_frame().await;
    }
}

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
    text: &'static str,
    x: f32,
    y: f32,
    bx: f32,
    by: f32,
    phase: f32,
    speed: f32,
    amp_x: f32,
    amp_y: f32,
}

fn letter_pattern(c: char) -> Vec<&'static str> {
    match c {
        'M' => vec![
            "1..............1",
            "11............11",
            "1.11........11.1",
            "1..11......11..1",
            "1...11....11...1",
            "1....11..11....1",
            "1.....1111.....1",
            "1......11......1",
            "1..............1",
            "1..............1",
            "1..............1",
            "1..............1",
            "1..............1",
            "1..............1",
            "1..............1",
            "1..............1",
            "1..............1",
            "1..............1",
            "1..............1",
            "1..............1",
        ],
        'I' => vec![
            ".....111111.....",
            ".......11.......",
            ".......11.......",
            ".......11.......",
            ".......11.......",
            ".......11.......",
            ".......11.......",
            ".......11.......",
            ".......11.......",
            ".......11.......",
            ".......11.......",
            ".......11.......",
            ".......11.......",
            ".......11.......",
            ".......11.......",
            ".......11.......",
            ".......11.......",
            ".......11.......",
            ".......11.......",
            ".....111111.....",
        ],
        'S' => vec![
            "....11111111....",
            "...11......11...",
            "..11............",
            "..11............",
            "...11...........",
            "....1111111.....",
            ".........1111...",
            "..........111...",
            "..........111...",
            "............11..",
            "..11........11..",
            "...11......11...",
            "....11111111....",
            "................",
            "................",
            "................",
            "................",
            "................",
            "................",
            "................",
        ],
        'E' => vec![
            "..111111111111..",
            "..11............",
            "..11............",
            "..11............",
            "..111111111111..",
            "..11............",
            "..11............",
            "..11............",
            "..11............",
            "..11............",
            "..11............",
            "..11............",
            "..11............",
            "..11............",
            "..11............",
            "..11............",
            "..11............",
            "..11............",
            "..11............",
            "..111111111111..",
        ],
        'R' => vec![
            "..11111111111...",
            "..11.......11...",
            "..11........11..",
            "..11........11..",
            "..11........11..",
            "..11.......11...",
            "..11111111111...",
            "..11...11.......",
            "..11....11......",
            "..11.....11.....",
            "..11......11....",
            "..11.......11...",
            "..11........11..",
            "..11.........11.",
            "................",
            "................",
            "................",
            "................",
            "................",
            "................",
        ],
        ' ' => vec!["................"],
        _ => vec![" "],
    }
}

fn generate_ascii_art_from_letters(text: &str) -> Vec<Vec<f32>> {
    let scale: usize = 4;
    let blur_passes: usize = 2;
    let spacer_w: usize = 4;

    let text_upper: String = text.to_uppercase();
    let letters: Vec<Vec<&str>> = text_upper
        .chars()
        .map(|c| letter_pattern(c))
        .collect();

    let letter_h = letters[0].len();
    let letter_w = letters[0][0].len();

    let mut total_w = 0usize;
    for (i, _) in letters.iter().enumerate() {
        if i > 0 {
            total_w += spacer_w;
        }
        total_w += letter_w;
    }

    let mut raw_grid = vec![vec![false; total_w]; letter_h];

    let mut cx = 0usize;
    for (i, letter) in letters.iter().enumerate() {
        if i > 0 {
            cx += spacer_w;
        }
        for (y, row) in letter.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                if x < letter_w {
                    raw_grid[y][cx + x] = ch != '.' && ch != ' ';
                }
            }
        }
        cx += letter_w;
    }

    let up_h = letter_h * scale;
    let up_w = total_w * scale;
    let mut current = vec![vec![0.0f32; up_w]; up_h];
    let mut next = vec![vec![0.0f32; up_w]; up_h];

    for y in 0..letter_h {
        for x in 0..total_w {
            let val = if raw_grid[y][x] { 1.0f32 } else { 0.0f32 };
            for dy in 0..scale {
                for dx in 0..scale {
                    current[y * scale + dy][x * scale + dx] = val;
                }
            }
        }
    }

    for _ in 0..blur_passes {
        for y in 0..up_h {
            for x in 0..up_w {
                let mut sum = 0.0f32;
                let mut count = 0i32;
                let r: isize = 1;
                for dy in -r..=r {
                    for dx in -r..=r {
                        let ny = y as isize + dy;
                        let nx = x as isize + dx;
                        if ny >= 0 && ny < up_h as isize && nx >= 0 && nx < up_w as isize {
                            sum += current[ny as usize][nx as usize];
                            count += 1;
                        }
                    }
                }
                next[y][x] = sum / count as f32;
            }
        }
        std::mem::swap(&mut current, &mut next);
    }

    let mut result = vec![vec![0.0f32; total_w]; letter_h];
    for y in 0..letter_h {
        for x in 0..total_w {
            let mut sum = 0.0f32;
            for dy in 0..scale {
                for dx in 0..scale {
                    let py = y * scale + dy;
                    let px = x * scale + dx;
                    sum += current[py][px];
                }
            }
            result[y][x] = sum / (scale * scale) as f32;
        }
    }

    result
}

fn load_ascii_art_from_file(path: &str) -> Option<Vec<Vec<f32>>> {
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

    let mut grid = vec![vec![0.0f32; w]; h];

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
                grid[y][x] = density;
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
                        sum += grid[ny as usize][nx as usize];
                        cnt += 1;
                    }
                }
            }
            blurred[y][x] = (sum / cnt as f32).max(grid[y][x]);
        }
    }

    Some(blurred)
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
        window_title: "Misser - Digital Card".to_owned(),
        window_width: 1300,
        window_height: 850,
        window_resizable: true,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    let (art_grid, art_label) = if args.len() > 1 {
        match load_ascii_art_from_file(&args[1]) {
            Some(grid) => (grid, args[1].clone()),
            None => {
                eprintln!(
                    "Could not load '{}', falling back to default art.",
                    args[1]
                );
                (generate_ascii_art_from_letters("MISSER"), "MISSER".to_string())
            }
        }
    } else {
        (generate_ascii_art_from_letters("MISSER"), "MISSER".to_string())
    };

    let grid_h = art_grid.len();
    let grid_w = if grid_h > 0 { art_grid[0].len() } else { 0 };

    if grid_w == 0 || grid_h == 0 {
        panic!("Failed to generate any art");
    }

    let sx = CHAR_SIZE * 0.33;
    let sy = CHAR_SIZE * 0.85;
    let art_w = grid_w as f32 * sx;
    let art_h = grid_h as f32 * sy;
    let art_start_x = screen_width() / 2.0 - art_w / 2.0;
    let art_start_y = screen_height() / 2.0 - art_h / 2.0;

    let mut particles: Vec<Particle> = Vec::new();

    for gy in 0..grid_h {
        for gx in 0..grid_w {
            let d = art_grid[gy][gx];
            if d > 0.02 {
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
                    ch: '.',
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
            text: name,
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
            let dims = measure_text(s.text, None, 21, 1.0);
            draw_text(
                s.text,
                s.x - dims.width / 2.0,
                s.y - dims.height / 2.0,
                21.0,
                color,
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

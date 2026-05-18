use macroquad::prelude::*;
use std::f32::consts::PI;

const SPRING_K: f32 = 0.08;
const DAMPING: f32 = 0.88;
const REPULSION_RADIUS: f32 = 120.0;
const REPULSION_FORCE: f32 = 800.0;
const CHAR_SIZE: f32 = 13.0;

const DENSITY_CHARS: &[char] = &['@', '%', '#', '*', '+', '=', '-', ':', '.', ' '];

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
            "1........1",
            "11......11",
            "1.1....1.1",
            "1..1..1..1",
            "1...11...1",
            "1........1",
            "1........1",
            "1........1",
            "1........1",
            "1........1",
        ],
        'I' => vec![
            "..111111..",
            "....11....",
            "....11....",
            "....11....",
            "....11....",
            "....11....",
            "....11....",
            "....11....",
            "....11....",
            "..111111..",
        ],
        'S' => vec![
            "..111111..",
            ".11....11.",
            "11........",
            ".11.......",
            "..1111....",
            ".....111..",
            ".......11.",
            "11....11..",
            ".111111...",
            "..........",
        ],
        'E' => vec![
            ".11111111.",
            ".11.......",
            ".11.......",
            ".1111111..",
            ".11.......",
            ".11.......",
            ".11.......",
            ".11.......",
            ".11.......",
            ".11111111.",
        ],
        'R' => vec![
            ".1111111..",
            ".11....11.",
            ".11....11.",
            ".11....11.",
            ".1111111..",
            ".11..11...",
            ".11...11..",
            ".11....11.",
            ".11....11.",
            ".11....11.",
        ],
        ' ' => vec!["........"],
        _ => vec![" "],
    }
}

fn generate_ascii_art(text: &str) -> Vec<Vec<f32>> {
    let scale: usize = 3;
    let blur_passes: usize = 2;
    let letter_h: usize = 10;
    let spacer_w: usize = 3;

    let text_upper: String = text.to_uppercase();
    let letters: Vec<Vec<&str>> = text_upper
        .chars()
        .map(|c| letter_pattern(c))
        .collect();

    let mut total_w = 0usize;
    for (i, letter) in letters.iter().enumerate() {
        if i > 0 {
            total_w += spacer_w;
        }
        total_w += letter[0].len();
    }

    let mut raw_grid = vec![vec![false; total_w]; letter_h];

    let mut cx = 0usize;
    for (i, letter) in letters.iter().enumerate() {
        if i > 0 {
            cx += spacer_w;
        }
        for (y, row) in letter.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                if x < letter[0].len() {
                    raw_grid[y][cx + x] = ch != '.' && ch != ' ';
                }
            }
        }
        cx += letter[0].len();
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

fn brightness_to_char(b: f32) -> char {
    let idx = ((1.0 - b) * (DENSITY_CHARS.len() - 1) as f32) as usize;
    DENSITY_CHARS[idx.min(DENSITY_CHARS.len() - 1)]
}

fn brightness_to_color(b: f32) -> Color {
    let t = b.clamp(0.0, 1.0);
    let g = 0.15 + t * 0.85;
    Color::new(0.05 + t * 0.15, g, 0.05 + t * 0.25, 0.92)
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Misser - Digital Card".to_owned(),
        window_width: 1000,
        window_height: 750,
        window_resizable: true,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let art_text = "MISSER";
    let art_grid = generate_ascii_art(art_text);

    let grid_h = art_grid.len();
    let grid_w = if grid_h > 0 { art_grid[0].len() } else { 0 };

    let scale_x = CHAR_SIZE * 0.58;
    let scale_y = CHAR_SIZE * 1.15;
    let art_w = grid_w as f32 * scale_x;
    let art_h = grid_h as f32 * scale_y;
    let art_start_x = screen_width() / 2.0 - art_w / 2.0;
    let art_start_y = screen_height() / 2.0 - art_h / 2.0;

    let mut particles: Vec<Particle> = Vec::new();

    for gy in 0..grid_h {
        for gx in 0..grid_w {
            let brightness = art_grid[gy][gx];
            if brightness > 0.06 {
                let tx = art_start_x + gx as f32 * scale_x;
                let ty = art_start_y + gy as f32 * scale_y;
                let sx = macroquad::rand::gen_range(0.0, screen_width());
                let sy = macroquad::rand::gen_range(0.0, screen_height());

                particles.push(Particle {
                    x: sx,
                    y: sy,
                    tx,
                    ty,
                    vx: macroquad::rand::gen_range(-80.0, 80.0),
                    vy: macroquad::rand::gen_range(-80.0, 80.0),
                    ch: brightness_to_char(brightness),
                    brightness,
                });
            }
        }
    }

    let skill_names: &[&str] = &[
        "Rust", "Python", "ML / AI",
        "Web Dev", "Linux", "Docker",
        "Git", "SQL", "Cloud", "OSS",
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
            speed: macroquad::rand::gen_range(0.25, 0.60),
            amp_x: macroquad::rand::gen_range(12.0, 30.0),
            amp_y: macroquad::rand::gen_range(8.0, 22.0),
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

        clear_background(Color::new(0.03, 0.03, 0.08, 1.0));

        let grid_spacing = 40.0;
        let grid_color = Color::new(0.06, 0.06, 0.13, 0.35);
        let grid_thickness = 0.4;
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
            let base_color = brightness_to_color(p.brightness);

            let dx = p.x - mx;
            let dy = p.y - my;
            let dist = (dx * dx + dy * dy).sqrt();
            let glow = if mouse_down && dist < REPULSION_RADIUS * 1.5 {
                1.0 + (1.0 - dist / (REPULSION_RADIUS * 1.5)) * 0.6
            } else {
                1.0
            };

            let color = Color::new(
                (base_color.r * glow).min(1.0),
                (base_color.g * glow).min(1.0),
                (base_color.b * glow).min(1.0),
                base_color.a,
            );

            draw_text(
                &p.ch.to_string(),
                p.x,
                p.y,
                CHAR_SIZE,
                color,
            );
        }

        for s in skills.iter() {
            let alpha = 0.4 + 0.1 * (time * s.speed * 1.3).sin();
            let color = Color::new(0.15, 0.75, 0.85, alpha);
            let dims = measure_text(s.text, None, 20, 1.0);
            draw_text(
                s.text,
                s.x - dims.width / 2.0,
                s.y - dims.height / 2.0,
                20.0,
                color,
            );
        }

        let hint = if cfg!(target_os = "android") {
            "Touch & hold to interact"
        } else {
            "Click & hold to interact"
        };
        let hdims = measure_text(hint, None, 13, 1.0);
        draw_text(
            hint,
            w / 2.0 - hdims.width / 2.0,
            h - 30.0,
            13.0,
            Color::new(0.35, 0.35, 0.50, 0.55),
        );

        let sub = "Digital Business Card";
        let sdims = measure_text(sub, None, 15, 1.0);
        draw_text(
            sub,
            w / 2.0 - sdims.width / 2.0,
            art_start_y + art_h + 50.0,
            15.0,
            Color::new(0.25, 0.55, 0.65, 0.5),
        );

        next_frame().await;
    }
}

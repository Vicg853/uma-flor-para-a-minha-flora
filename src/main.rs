use macroquad::prelude::*;
use std::f32::consts::TAU;
use macroquad::window::Conf;
use macroquad::rand::gen_range;

// Configura a janela com anti‐aliasing (MSAA 4×)
fn window_conf() -> Conf {
    Conf {
        window_title: "Uma Rosa para a minha Flor(a)".to_owned(),
        sample_count: 4,
        ..Default::default()
    }
}

// Interpola linearmente entre duas cores
fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    Color {
        r: a.r * (1.0 - t) + b.r * t,
        g: a.g * (1.0 - t) + b.g * t,
        b: a.b * (1.0 - t) + b.b * t,
        a: 1.0,
    }
}

// Desenha uma nuvem decorativa com três círculos
fn draw_cloud(x: f32, y: f32, scale: f32) {
    let radii = [20.0, 25.0, 20.0];
    let offsets = [(-25.0, 0.0), (0.0, -10.0), (25.0, 0.0)];
    for i in 0..3 {
        let r = radii[i] * scale;
        let (dx, dy) = offsets[i];
        let cx = x + dx * scale;
        let cy = y + dy * scale;
        draw_circle(cx, cy, r, WHITE);
        draw_circle_lines(cx, cy, r, 2.0, BLACK);
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let a = 150.0;     // escala da rosa
    let k = 4.0;       // número de pétalas
    let steps = 3600;  // resolução da curva

    // Obtém dimensões da tela
    let w = screen_width();
    let h = screen_height();
    let sky_height = h * 0.8;
    let grass_y = h * 0.8;

    // Gera padrões fixos do céu
    let sky_blue = Color::new(0.529 * 0.5, 0.808 * 0.5, 0.922 * 0.5, 1.0);
    let mut sky_patterns: Vec<(f32, f32, f32, Color)> = Vec::new();
    for _ in 0..50 {
        let x = gen_range(0.0, w);
        let y = gen_range(0.0, sky_height);
        let r = gen_range(5.0, 15.0);
        let variation = gen_range(-0.05, 0.05);
        let c = Color::new(
            (sky_blue.r + variation).clamp(0.0, 1.0),
            (sky_blue.g + variation).clamp(0.0, 1.0),
            (sky_blue.b + variation).clamp(0.0, 1.0),
            1.0,
        );
        sky_patterns.push((x, y, r, c));
    }

    // Gera padrões fixos do solo
    let grass_green = Color::new(0.0 * 0.5, 1.0 * 0.5, 0.0 * 0.5, 1.0);
    let mut ground_patterns: Vec<(f32, f32, f32, Color)> = Vec::new();
    for _ in 0..50 {
        let x = gen_range(0.0, w);
        let y = gen_range(grass_y, h);
        let r = gen_range(5.0, 20.0);
        let variation = gen_range(-0.05, 0.05);
        let c = Color::new(
            (grass_green.r + variation).clamp(0.0, 1.0),
            (grass_green.g + variation).clamp(0.0, 1.0),
            (grass_green.b + variation).clamp(0.0, 1.0),
            1.0,
        );
        ground_patterns.push((x, y, r, c));
    }

    loop {
        // --- Desenho de cena ---
        // Céu
        draw_rectangle(0.0, 0.0, w, sky_height, sky_blue);
        for &(x, y, r, c) in &sky_patterns {
            draw_circle(x, y, r, c);
        }
        // Nuvens
        draw_cloud(w * 0.2, h * 0.2, 1.0);
        draw_cloud(w * 0.5, h * 0.15, 1.2);
        draw_cloud(w * 0.8, h * 0.25, 0.9);

        // Solo
        draw_rectangle(0.0, grass_y, w, h - grass_y, grass_green);
        for &(x, y, r, c) in &ground_patterns {
            draw_circle(x, y, r, c);
        }

        // Caule da rosa
        let cx = w * 0.5;
        let cy = h * 0.5;
        let stem_color = Color::new(0.0, 0.3, 0.0, 1.0); // mais escuro
        let stem_top = cy + 10.0;
        draw_line(cx, h, cx, stem_top, 10.0, BLACK);
        draw_line(cx, h, cx, stem_top, 6.0, stem_color);

        // Rosa giratória
        let t = get_time() as f32;
        let rotation = 0.1 * (t * 0.5).sin();
        let inner = Color::new(
            1.0,
            0.4 + 0.2 * t.sin(),
            0.3 + 0.2 * (t + 1.0).sin(),
            1.0,
        );
        let outer = Color::new(
            0.9,
            0.3 + 0.2 * (t + 2.0).sin(),
            0.3 + 0.2 * (t + 3.0).sin(),
            1.0,
        );
        let pontos: Vec<(Vec2, f32)> = (0..=steps)
            .map(|i| {
                let theta0 = i as f32 / steps as f32 * TAU;
                let r = a * (k * theta0).sin();
                let theta = theta0 + rotation;
                let x = r * theta.cos() + cx;
                let y = r * theta.sin() + cy;
                (vec2(x, y), r.abs())
            })
            .collect();
        for i in 0..pontos.len() - 1 {
            let (p0, r0) = pontos[i];
            let (p1, r1) = pontos[i + 1];
            let t_mid = (r0 / a + r1 / a) * 0.5;
            let color = lerp_color(inner, outer, t_mid);
            draw_triangle(vec2(cx, cy), p0, p1, color);
        }
        for window in pontos.windows(2) {
            let p0 = window[0].0;
            let p1 = window[1].0;
            draw_line(p0.x, p0.y, p1.x, p1.y, 4.0, BLACK);
            draw_line(p0.x, p0.y, p1.x, p1.y, 2.0, RED);
        }

        next_frame().await;
    }
}


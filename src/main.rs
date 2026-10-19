mod framebuffer;
mod line;
mod obj;
mod triangle;

use framebuffer::Framebuffer;
use glam::Vec3;
use minifb::{Key, Window, WindowOptions};
use obj::Model;
use triangle::{triangle, triangle_wireframe};

const SCREEN_WIDTH: usize = 800;
const SCREEN_HEIGHT: usize = 600;

fn rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | b as u32
}

fn rotate_y(v: Vec3, angle: f32) -> Vec3 {
    let (s, c) = angle.sin_cos();
    Vec3::new(v.x * c + v.z * s, v.y, -v.x * s + v.z * c)
}

fn fit(model: &Model) -> (Vec3, f32) {
    let mut min = Vec3::splat(f32::MAX);
    let mut max = Vec3::splat(f32::MIN);
    for v in &model.vertices {
        min = min.min(*v);
        max = max.max(*v);
    }
    let extent = (max - min).max_element().max(1e-6);
    let scale = SCREEN_HEIGHT.min(SCREEN_WIDTH) as f32 * 0.8 / extent;
    ((min + max) * 0.5, scale)
}

fn render(fb: &mut Framebuffer, model: &Model, angle: f32, wireframe: bool) {
    let (center, scale) = fit(model);
    let light = Vec3::new(0.3, 0.5, 1.0).normalize();

    // Transformar vértices: centrar, rotar, escalar y pasar a coordenadas de pantalla.
    let world: Vec<Vec3> = model
        .vertices
        .iter()
        .map(|v| rotate_y((*v - center) * scale, angle))
        .collect();
    let screen: Vec<Vec3> = world
        .iter()
        .map(|v| {
            Vec3::new(
                v.x + SCREEN_WIDTH as f32 / 2.0,
                SCREEN_HEIGHT as f32 / 2.0 - v.y,
                v.z,
            )
        })
        .collect();

    // Dibujar cada cara.
    for &[i, j, k] in &model.faces {
        let normal = (world[j] - world[i]).cross(world[k] - world[i]).normalize_or_zero();

        if wireframe {
            fb.set_current_color(rgb(255, 255, 0));
            triangle_wireframe(fb, screen[i], screen[j], screen[k]);
        } else {
            let l = normal.dot(light).max(0.0);
            let shade = (40.0 + 215.0 * l) as u8;
            fb.set_current_color(rgb(shade, shade, shade));
            triangle(fb, screen[i], screen[j], screen[k]);
        }
    }
}

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| "assets/xwing.obj".to_string());
    let model = obj::load(&path).unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });
    println!("{path}: {} vértices, {} triángulos", model.vertices.len(), model.faces.len());

    let mut window = Window::new(
        "X-wing",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions::default(),
    )
    .expect("no se pudo crear la ventana");
    window.set_target_fps(60);

    let mut fb = Framebuffer::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut angle = 0.0_f32;
    let mut wireframe = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::W, minifb::KeyRepeat::No) {
            wireframe = !wireframe;
        }
        angle += 0.01;

        fb.clear();
        render(&mut fb, &model, angle, wireframe);

        window
            .update_with_buffer(&fb.buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }
}

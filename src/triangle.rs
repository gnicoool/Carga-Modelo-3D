use crate::framebuffer::Framebuffer;
use crate::line::line;
use glam::Vec3;

// Contorno del triángulo
pub fn triangle_wireframe(fb: &mut Framebuffer, a: Vec3, b: Vec3, c: Vec3) {
    line(fb, a, b);
    line(fb, b, c);
    line(fb, c, a);
}

// Funcion del borde para saber si es positivo o negativo
fn edge(a: Vec3, b: Vec3, p: Vec3) -> f32 {
    (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x)
}

// Se recorre el bounding box del triángulo y calculo por pixel de las coordenadas baricéntricas para saber si esta dentro
pub fn triangle(fb: &mut Framebuffer, a: Vec3, b: Vec3, c: Vec3) {
    let area = edge(a, b, c);
    if area.abs() < 1e-6 {
        return;
    }

    let min_x = a.x.min(b.x).min(c.x).floor().max(0.0) as usize;
    let max_x = a.x.max(b.x).max(c.x).ceil().min(fb.width as f32 - 1.0) as usize;
    let min_y = a.y.min(b.y).min(c.y).floor().max(0.0) as usize;
    let max_y = a.y.max(b.y).max(c.y).ceil().min(fb.height as f32 - 1.0) as usize;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);
            let w0 = edge(b, c, p);
            let w1 = edge(c, a, p);
            let w2 = edge(a, b, p);

            if w0 * area >= 0.0 && w1 * area >= 0.0 && w2 * area >= 0.0 {
                let z = (w0 * a.z + w1 * b.z + w2 * c.z) / area;
                fb.point_depth(x, y, z);
            }
        }
    }
}

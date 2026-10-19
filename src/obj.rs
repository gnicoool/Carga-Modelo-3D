use glam::Vec3;
use std::fs;

pub struct Model {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<[usize; 3]>,
}

/// Carga un archivo .obj triangulado. Solo lee posiciones (`v`) y caras (`f`)
pub fn load(path: &str) -> Result<Model, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("no se pudo leer {path}: {e}"))?;

    let mut vertices: Vec<Vec3> = Vec::new();
    let mut faces: Vec<[usize; 3]> = Vec::new();

    for (n, line) in text.lines().enumerate() {
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("v") => {
                let c: Vec<f32> = parts.take(3).map(|s| s.parse().unwrap_or(0.0)).collect();
                if c.len() < 3 {
                    return Err(format!("línea {}: vértice incompleto", n + 1));
                }
                vertices.push(Vec3::new(c[0], c[1], c[2]));
            }
            Some("f") => {
                // Cada elemento puede ser "v", "v/vt", "v//vn" o "v/vt/vn"                let mut idx: Vec<usize> = Vec::new();
                for token in parts {
                    let raw = token.split('/').next().unwrap_or("");
                    let i: i64 = raw
                        .parse()
                        .map_err(|_| format!("línea {}: índice inválido '{token}'", n + 1))?;
                    let i = if i > 0 { i - 1 } else { vertices.len() as i64 + i };
                    if i < 0 || i as usize >= vertices.len() {
                        return Err(format!("línea {}: índice fuera de rango", n + 1));
                    }
                    idx.push(i as usize);
                }
                if idx.len() != 3 {
                    return Err(format!(
                        "línea {}: la cara tiene {} vértices, el modelo debe estar triangulado",
                        n + 1,
                        idx.len()
                    ));
                }
                faces.push([idx[0], idx[1], idx[2]]);
            }
            _ => {}
        }
    }

    Ok(Model { vertices, faces })
}

use crate::{math::clamp, vec3::Color};

pub fn as_ppm(color: Color, samples_per_pixel: u32) -> String {
    // Get components of the color
    let r = color.x();
    let g = color.y();
    let b = color.z();

    // Divide color by number of params and perform gamma correction (gamma=2.0)
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (scale * r).sqrt();
    let g = (scale * g).sqrt();
    let b = (scale * b).sqrt();

    // Normalize colors from [0.0, 1.0] to [0, 255]
    let r = (256.0 * clamp(r, 0.0..0.999)) as u32;
    let g = (256.0 * clamp(g, 0.0..0.999)) as u32;
    let b = (256.0 * clamp(b, 0.0..0.999)) as u32;

    format!("{} {} {}", r, g, b)
}

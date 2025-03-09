use crate::interval::Interval;

use glam::Vec3;

#[inline(always)]
fn linear_to_gamma(linear: f32) -> f32 {
    if linear > 0.0 {
        return linear.sqrt();
    }
    0.0
}

#[inline]
pub fn vec3_to_rgb8(color_vec: Vec3) -> image::Rgb<u8> {
    let r = linear_to_gamma(color_vec.x);
    let g = linear_to_gamma(color_vec.y);
    let b = linear_to_gamma(color_vec.z);

    const INTENSITY_INTERVAL: Interval = Interval::new(0.0, 0.999);
    let r_byte = (256.0 * INTENSITY_INTERVAL.clamp(r)) as u8;
    let g_byte = (256.0 * INTENSITY_INTERVAL.clamp(g)) as u8;
    let b_byte = (256.0 * INTENSITY_INTERVAL.clamp(b)) as u8;

    image::Rgb([r_byte, g_byte, b_byte])
}

use crate::interval::Interval;

use glam::Vec3;

fn linear_to_gamma(linear: f32) -> f32 {
    if linear > 0.0 {
        return linear.sqrt();
    }
    0.0
}

#[inline]
pub fn vec3_to_rgb8(color_vec: Vec3) -> image::Rgb<u8> {
    let mut r = linear_to_gamma(color_vec.x);
    let mut g = linear_to_gamma(color_vec.y);
    let mut b = linear_to_gamma(color_vec.z);

    if r != r {
        r = 0.0;
    }
    if g != g {
        g = 0.0;
    }
    if b != b {
        b = 0.0;
    }

    const INTENSITY_INTERVAL: Interval = Interval::new(0.0, 0.999);
    let r_byte = (256.0 * INTENSITY_INTERVAL.clamp(r)) as u8;
    let g_byte = (256.0 * INTENSITY_INTERVAL.clamp(g)) as u8;
    let b_byte = (256.0 * INTENSITY_INTERVAL.clamp(b)) as u8;

    image::Rgb([r_byte, g_byte, b_byte])
}

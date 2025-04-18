use glam::{Vec2, Vec3};

#[inline(always)]
pub fn deg_to_rad(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180.0
}

#[inline(always)]
pub fn rad_to_deg(rad: f32) -> f32 {
    rad * 180.0 / std::f32::consts::PI
}

#[inline(always)]
pub fn random_vec3(min: Vec3, max: Vec3, rng: &mut impl rand::Rng) -> Vec3 {
    Vec3::new(
        rng.random_range(min.x..=max.x),
        rng.random_range(min.y..=max.y),
        rng.random_range(min.z..=max.z),
    )
}

#[inline(always)]
pub fn random_unit_vec3(rng: &mut impl rand::Rng) -> Vec3 {
    let theta = rng.random_range(0.0..(2.0 * std::f32::consts::PI));
    let z: f32 = rng.random_range(-1.0..1.0);
    Vec3::new(
        (1.0 - z * z).sqrt() * theta.cos(),
        (1.0 - z * z).sqrt() * theta.sin(),
        z,
    )
}

#[inline(always)]
pub fn random_on_hemisphere(normal: Vec3, rng: &mut impl rand::Rng) -> Vec3 {
    let unit = random_unit_vec3(rng);
    if unit.dot(normal) > 0.0 { unit } else { -unit }
}

#[inline(always)]
pub fn random_in_unit_disk(rng: &mut impl rand::Rng) -> Vec2 {
    let theta = rng.random_range(0.0..(2.0 * std::f32::consts::PI));
    let r = rng.random_range(0.0 as f32..=1.0 as f32).sqrt();
    Vec2::new(r * theta.cos(), r * theta.sin())
}

#[inline(always)]
pub fn vec3_near_zero(v: Vec3) -> bool {
    const S: f32 = 1e-8;
    (v.x.abs() < S) && (v.y.abs() < S) && (v.z.abs() < S)
}

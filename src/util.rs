use std::f32::consts::PI;

use glam::{Vec2, Vec3};
use rand::{Rng, RngCore};

pub fn random_vec3(min: Vec3, max: Vec3, rng: &mut dyn RngCore) -> Vec3 {
    Vec3::new(
        rng.random_range(min.x..=max.x),
        rng.random_range(min.y..=max.y),
        rng.random_range(min.z..=max.z),
    )
}

pub fn random_unit_vec3(rng: &mut dyn RngCore) -> Vec3 {
    let theta = rng.random_range(0.0..(2.0 * PI));
    let z: f32 = rng.random_range(-1.0..1.0);
    Vec3::new(
        (1.0 - z * z).sqrt() * theta.cos(),
        (1.0 - z * z).sqrt() * theta.sin(),
        z,
    )
}

pub fn random_on_hemisphere(normal: Vec3, rng: &mut dyn RngCore) -> Vec3 {
    let unit = random_unit_vec3(rng);
    if unit.dot(normal) > 0.0 { unit } else { -unit }
}

pub fn random_cosine_direction(rng: &mut dyn RngCore) -> Vec3 {
    let r1: f32 = rng.random();
    let r2: f32 = rng.random();

    let theta = 2.0 * PI * r1;
    let x = theta.cos() * r2.sqrt();
    let y = theta.sin() * r2.sqrt();
    let z = (1.0 - r2).sqrt();

    Vec3::new(x, y, z)
}

pub fn random_in_unit_disk(rng: &mut dyn RngCore) -> Vec2 {
    let theta = rng.random_range(0.0..(2.0 * PI));
    let r = rng.random_range(0.0 as f32..=1.0 as f32).sqrt();
    Vec2::new(r * theta.cos(), r * theta.sin())
}

pub const fn vec3_near_zero(v: Vec3) -> bool {
    const S: f32 = 1e-8;
    (v.x.abs() < S) && (v.y.abs() < S) && (v.z.abs() < S)
}

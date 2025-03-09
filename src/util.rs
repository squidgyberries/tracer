use glam::{Vec2, Vec3, vec2, vec3};

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
    vec3(
        rng.random_range(min.x..=max.x),
        rng.random_range(min.y..=max.y),
        rng.random_range(min.z..=max.z),
    )
}

#[inline(always)]
pub fn random_unit_vec3(rng: &mut impl rand::Rng) -> Vec3 {
    // loop {
    //     let v = random_vec3(Vec3::NEG_ONE, Vec3::ONE, rng);
    //     let length_squared = v.length_squared();
    //     if length_squared > 1e-35 && length_squared <= 1.0 {
    //         return v / length_squared.sqrt();
    //     }
    // }

    // loop {
    //     let v = vec3(
    //         rng.sample(rand_distr::StandardNormal),
    //         rng.sample(rand_distr::StandardNormal),
    //         rng.sample(rand_distr::StandardNormal),
    //     );
    //     let length_squared = v.length_squared();
    //     if length_squared > 1e-35 {
    //         return v / length_squared.sqrt();
    //     }
    // }

    let theta = rng.random_range(0.0..(2.0 * std::f32::consts::PI));
    let z: f32 = rng.random_range(-1.0..1.0);
    vec3(
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
    vec2(r * theta.cos(), r * theta.sin())
}

#[inline(always)]
pub fn vec3_near_zero(v: Vec3) -> bool {
    const S: f32 = 1e-8;
    (v.x.abs() < S) && (v.y.abs() < S) && (v.z.abs() < S)
}

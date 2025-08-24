use std::{fmt::Debug, sync::Arc};

use crate::{
    hit::HitRecord,
    ray::Ray,
    texture::{SolidColor, Texture},
    util::{random_unit_vec3, vec3_near_zero},
};

use glam::{Vec2, Vec3};
use rand::{Rng, RngCore};

// is there a better way to do this?
pub static DEFAULT_MATERIAL: std::sync::LazyLock<Arc<dyn Material>> =
    std::sync::LazyLock::new(|| {
        Arc::new(LambertianMaterial::new(
            Arc::new(SolidColor::from_rgb(1.0, 0.0, 1.0)),
            Vec3::ONE,
        ))
    });

pub trait Material: Send + Sync + Debug {
    fn scatter(
        &self,
        ray_in: Ray,
        hit_record: &HitRecord,
        rng: &mut dyn RngCore,
    ) -> (bool, Vec3, Ray);

    fn emitted(&self, uv: Vec2, point: Vec3) -> Vec3;
}

#[derive(Clone, Debug)]
pub struct LambertianMaterial {
    pub texture: Arc<dyn Texture>,
    pub diffuse_p: Vec3,
}

impl LambertianMaterial {
    #[inline(always)]
    pub const fn new(texture: Arc<dyn Texture>, diffuse_p: Vec3) -> Self {
        Self { texture, diffuse_p }
    }
}

impl Material for LambertianMaterial {
    fn scatter(
        &self,
        _ray_in: Ray,
        hit_record: &HitRecord,
        rng: &mut dyn RngCore,
    ) -> (bool, Vec3, Ray) {
        let mut scatter_direction = hit_record.normal + random_unit_vec3(rng);

        if vec3_near_zero(scatter_direction) {
            scatter_direction = hit_record.normal;
        }

        let mut attenuation = Vec3::ZERO;
        let random_scatter: f32 = rng.random();
        let scatter_r = random_scatter < self.diffuse_p.x;
        let scatter_g = random_scatter < self.diffuse_p.y;
        let scatter_b = random_scatter < self.diffuse_p.z;
        let texture_value = self.texture.value(hit_record.uv, hit_record.point);

        if scatter_r {
            attenuation.x = texture_value.x / self.diffuse_p.x;
        }
        if scatter_g {
            attenuation.y = texture_value.y / self.diffuse_p.y;
        }
        if scatter_b {
            attenuation.z = texture_value.z / self.diffuse_p.z;
        }

        (
            scatter_r || scatter_g || scatter_b,
            attenuation,
            Ray::new(hit_record.point, scatter_direction),
        )
    }

    #[inline(always)]
    fn emitted(&self, _uv: Vec2, _point: Vec3) -> Vec3 {
        Vec3::ZERO
    }
}

#[derive(Clone, Debug)]
pub struct MetalMaterial {
    pub texture: Arc<dyn Texture>,
    pub fuzz: f32,
}

impl MetalMaterial {
    #[inline(always)]
    pub const fn new(texture: Arc<dyn Texture>, fuzz: f32) -> Self {
        Self { texture, fuzz }
    }
}

impl Material for MetalMaterial {
    fn scatter(
        &self,
        ray_in: Ray,
        hit_record: &HitRecord,
        rng: &mut dyn RngCore,
    ) -> (bool, Vec3, Ray) {
        let mut reflected = ray_in.direction.reflect(hit_record.normal);
        reflected = reflected.normalize() + (self.fuzz * random_unit_vec3(rng));
        (
            true,
            self.texture.value(hit_record.uv, hit_record.point),
            Ray::new(hit_record.point, reflected),
        )
    }

    #[inline(always)]
    fn emitted(&self, _uv: Vec2, _point: Vec3) -> Vec3 {
        Vec3::ZERO
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct DielectricMaterial {
    pub refraction_index: f32,
}

impl DielectricMaterial {
    #[inline(always)]
    pub const fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }

    #[inline(always)]
    fn dielectric_reflectance(cosine: f32, refraction_index: f32) -> f32 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for DielectricMaterial {
    fn scatter(
        &self,
        ray_in: Ray,
        hit_record: &HitRecord,
        rng: &mut dyn RngCore,
    ) -> (bool, Vec3, Ray) {
        let ri = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        // REIMPLEMENT
        let unit_direction = ray_in.direction.normalize();
        let cos_theta = (-unit_direction).dot(hit_record.normal).min(1.0);

        let mut direction = unit_direction.normalize().refract(hit_record.normal, ri);
        if direction == Vec3::ZERO
            || Self::dielectric_reflectance(cos_theta, ri) > rng.random::<f32>()
        {
            direction = ray_in.direction.reflect(hit_record.normal);
        }

        (true, Vec3::ONE, Ray::new(hit_record.point, direction))
    }

    #[inline(always)]
    fn emitted(&self, _uv: Vec2, _point: Vec3) -> Vec3 {
        Vec3::ZERO
    }
}

#[derive(Clone, Debug)]
pub struct DiffuseLightMaterial {
    pub texture: Arc<dyn Texture>,
    pub strength: f32,
}

impl DiffuseLightMaterial {
    #[inline(always)]
    pub const fn new(texture: Arc<dyn Texture>, strength: f32) -> Self {
        Self { texture, strength }
    }
}

impl Material for DiffuseLightMaterial {
    #[inline(always)]
    fn scatter(
        &self,
        _ray_in: Ray,
        _hit_record: &HitRecord,
        _rng: &mut dyn RngCore,
    ) -> (bool, Vec3, Ray) {
        (false, Vec3::ZERO, Ray::default())
    }

    #[inline(always)]
    fn emitted(&self, uv: Vec2, point: Vec3) -> Vec3 {
        self.texture.value(uv, point) * self.strength
    }
}

#[derive(Clone, Debug)]
pub struct IsotropicMaterial {
    texture: Arc<dyn Texture>,
}

impl IsotropicMaterial {
    #[inline(always)]
    pub const fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Material for IsotropicMaterial {
    #[inline(always)]
    fn scatter(
        &self,
        _ray_in: Ray,
        hit_record: &HitRecord,
        rng: &mut dyn RngCore,
    ) -> (bool, Vec3, Ray) {
        (
            true,
            self.texture.value(hit_record.uv, hit_record.point),
            Ray::new(hit_record.point, crate::util::random_unit_vec3(rng)),
        )
    }

    #[inline(always)]
    fn emitted(&self, _uv: Vec2, _point: Vec3) -> Vec3 {
        Vec3::ZERO
    }
}

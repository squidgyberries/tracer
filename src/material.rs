use std::sync::Arc;

use crate::{
    hit::HitRecord,
    ray::Ray,
    texture::{SolidColor, Texture},
    util::{random_unit_vec3, vec3_near_zero},
};

use glam::{Vec2, Vec3};

#[derive(Clone)]
pub enum Material {
    Lambertian(LambertianMaterial),
    Metal(MetalMaterial),
    Dielectric(DielectricMaterial),
    DiffuseLight(DiffuseLightMaterial),
}

impl Material {
    #[inline(always)]
    pub fn scatter(
        &self,
        ray_in: Ray,
        hit_record: &HitRecord,
        rng: &mut impl rand::Rng,
    ) -> (bool, Vec3, Ray) {
        match self {
            Material::Lambertian(m) => m.scatter(ray_in, hit_record, rng),
            Material::Metal(m) => m.scatter(ray_in, hit_record, rng),
            Material::Dielectric(m) => m.scatter(ray_in, hit_record, rng),
            Material::DiffuseLight(m) => m.scatter(ray_in, hit_record, rng),
        }
    }

    #[inline(always)]
    pub fn emitted(&self, uv: Vec2, point: Vec3) -> Vec3 {
        match self {
            Material::Lambertian(m) => m.emitted(uv, point),
            Material::Metal(m) => m.emitted(uv, point),
            Material::Dielectric(m) => m.emitted(uv, point),
            Material::DiffuseLight(m) => m.emitted(uv, point),
        }
    }

    #[inline(always)]
    pub fn new_lambertian(texture: Arc<dyn Texture + Send + Sync>, diffuse_p: Vec3) -> Self {
        Self::Lambertian(LambertianMaterial::new(texture, diffuse_p))
    }

    #[inline(always)]
    pub fn new_metal(texture: Arc<dyn Texture + Send + Sync>, fuzz: f32) -> Self {
        Self::Metal(MetalMaterial::new(texture, fuzz))
    }

    #[inline(always)]
    pub const fn new_dielectric(refraction_index: f32) -> Self {
        Self::Dielectric(DielectricMaterial::new(refraction_index))
    }

    #[inline(always)]
    pub const fn new_diffuse_light(texture: Arc<dyn Texture + Send + Sync>, strength: f32) -> Self {
        Self::DiffuseLight(DiffuseLightMaterial::new(texture, strength))
    }
}

impl Default for Material {
    #[inline(always)]
    fn default() -> Self {
        Self::new_lambertian(Arc::new(SolidColor::from_rgb(1.0, 0.0, 1.0)), Vec3::ONE)
    }
}

#[derive(Clone)]
pub struct LambertianMaterial {
    pub texture: Arc<dyn Texture + Send + Sync>,
    pub diffuse_p: Vec3,
}

impl LambertianMaterial {
    #[inline(always)]
    pub fn new(texture: Arc<dyn Texture + Send + Sync>, diffuse_p: Vec3) -> Self {
        Self { texture, diffuse_p }
    }

    pub fn scatter(
        &self,
        _ray_in: Ray,
        hit_record: &HitRecord,
        rng: &mut impl rand::Rng,
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
    pub fn emitted(&self, _uv: Vec2, _point: Vec3) -> Vec3 {
        Vec3::ZERO
    }
}

#[derive(Clone)]
pub struct MetalMaterial {
    pub texture: Arc<dyn Texture + Send + Sync>,
    pub fuzz: f32,
}

impl MetalMaterial {
    #[inline(always)]
    pub const fn new(texture: Arc<dyn Texture + Send + Sync>, fuzz: f32) -> Self {
        Self { texture, fuzz }
    }

    pub fn scatter(
        &self,
        ray_in: Ray,
        hit_record: &HitRecord,
        rng: &mut impl rand::Rng,
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
    pub fn emitted(&self, _uv: Vec2, _point: Vec3) -> Vec3 {
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

    pub fn scatter(
        &self,
        ray_in: Ray,
        hit_record: &HitRecord,
        rng: &mut impl rand::Rng,
    ) -> (bool, Vec3, Ray) {
        let ri = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        // REIMPLEMENT
        let unit_direction = ray_in.direction.normalize();
        let cos_theta = (-unit_direction).dot(hit_record.normal).min(1.0);

        let mut direction = unit_direction
            .normalize()
            .refract(hit_record.normal, ri);
        if direction == Vec3::ZERO
            || Self::dielectric_reflectance(cos_theta, ri) > rng.random::<f32>()
        {
            direction = ray_in.direction.reflect(hit_record.normal);
        }

        (true, Vec3::ONE, Ray::new(hit_record.point, direction))
    }

    #[inline(always)]
    pub fn emitted(&self, _uv: Vec2, _point: Vec3) -> Vec3 {
        Vec3::ZERO
    }
}

#[derive(Clone)]
pub struct DiffuseLightMaterial {
    pub texture: Arc<dyn Texture + Send + Sync>,
    pub strength: f32,
}

impl DiffuseLightMaterial {
    #[inline(always)]
    pub const fn new(texture: Arc<dyn Texture + Send + Sync>, strength: f32) -> Self {
        Self { texture, strength }
    }

    #[inline(always)]
    pub fn scatter(
        &self,
        _ray_in: Ray,
        _hit_record: &HitRecord,
        _rng: &mut impl rand::Rng,
    ) -> (bool, Vec3, Ray) {
        (false, Vec3::ZERO, Ray::default())
    }

    #[inline(always)]
    pub fn emitted(&self, uv: Vec2, point: Vec3) -> Vec3 {
        self.texture.value(uv, point) * self.strength
    }
}

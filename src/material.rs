use std::{fmt::Debug, sync::Arc};

use crate::{
    hit::HitRecord,
    onb::Onb,
    pdf::{CosinePdf, Pdf, SpherePdf},
    ray::Ray,
    texture::{SolidColor, Texture},
    util::{random_cosine_direction, random_unit_vec3},
};

use either::Either;
use glam::{Vec2, Vec3};
use rand::{Rng, RngCore};

#[derive(Clone, Debug)]
pub struct ScatterRecord {
    pub attenuation: Vec3,
    pub pdf_or_skip_ray: Either<Arc<dyn Pdf>, Ray>,
}

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
    ) -> Option<ScatterRecord>;

    fn emitted(&self, hit_record: &HitRecord, uv: Vec2, point: Vec3) -> Vec3;

    fn scattering_pdf(&self, ray_in: Ray, hit_record: &HitRecord, scattered: Ray) -> f32;
}

#[derive(Clone, Debug)]
pub struct LambertianMaterial {
    pub texture: Arc<dyn Texture>,
    pub diffuse_p: Vec3,
}

impl LambertianMaterial {
    pub const fn new(texture: Arc<dyn Texture>, diffuse_p: Vec3) -> Self {
        Self { texture, diffuse_p }
    }
}

impl Material for LambertianMaterial {
    fn scatter(
        &self,
        _ray_in: Ray,
        hit_record: &HitRecord,
        _rng: &mut dyn RngCore,
    ) -> Option<ScatterRecord> {
        let attenuation = self.texture.value(hit_record.uv, hit_record.point);
        let pdf = Arc::new(CosinePdf::new(hit_record.normal));

        Some(ScatterRecord {
            attenuation,
            pdf_or_skip_ray: Either::Left(pdf),
        })
    }

    fn emitted(&self, _hit_record: &HitRecord, _uv: Vec2, _point: Vec3) -> Vec3 {
        Vec3::ZERO
    }

    fn scattering_pdf(&self, ray_in: Ray, hit_record: &HitRecord, scattered: Ray) -> f32 {
        let cos_theta = hit_record.normal.dot(scattered.direction.normalize());
        if cos_theta < 0.0 {
            0.0
        } else {
            cos_theta * std::f32::consts::FRAC_1_PI
        }
    }
}

#[derive(Clone, Debug)]
pub struct MetalMaterial {
    pub texture: Arc<dyn Texture>,
    pub fuzz: f32,
}

impl MetalMaterial {
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
    ) -> Option<ScatterRecord> {
        let attenuation = self.texture.value(hit_record.uv, hit_record.point);

        let reflected = ray_in.direction.reflect(hit_record.normal).normalize();
        let reflected_fuzzed = reflected.normalize() + (self.fuzz * random_unit_vec3(rng));
        let ray = Ray::new(hit_record.point, reflected_fuzzed);

        Some(ScatterRecord {
            attenuation,
            pdf_or_skip_ray: Either::Right(ray),
        })
    }

    fn emitted(&self, _hit_record: &HitRecord, _uv: Vec2, _point: Vec3) -> Vec3 {
        Vec3::ZERO
    }

    fn scattering_pdf(&self, ray_in: Ray, hit_record: &HitRecord, scattered: Ray) -> f32 {
        0.0
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct DielectricMaterial {
    pub refraction_index: f32,
}

impl DielectricMaterial {
    pub const fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }

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
    ) -> Option<ScatterRecord> {
        let attenuation = Vec3::ONE;

        let ri = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.direction.normalize();
        let cos_theta = (-unit_direction).dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > rng.random::<f32>()
        {
            unit_direction.reflect(hit_record.normal)
        } else {
            unit_direction.refract(hit_record.normal, ri)
        };

        let ray = Ray::new(hit_record.point, direction);

        Some(ScatterRecord {
            attenuation,
            pdf_or_skip_ray: Either::Right(ray),
        })
    }

    fn emitted(&self, _hit_record: &HitRecord, _uv: Vec2, _point: Vec3) -> Vec3 {
        Vec3::ZERO
    }

    fn scattering_pdf(&self, ray_in: Ray, hit_record: &HitRecord, scattered: Ray) -> f32 {
        0.0
    }
}

impl DielectricMaterial {
    fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

#[derive(Clone, Debug)]
pub struct DiffuseLightMaterial {
    pub texture: Arc<dyn Texture>,
    pub strength: f32,
}

impl DiffuseLightMaterial {
    pub const fn new(texture: Arc<dyn Texture>, strength: f32) -> Self {
        Self { texture, strength }
    }
}

impl Material for DiffuseLightMaterial {
    fn scatter(
        &self,
        _ray_in: Ray,
        _hit_record: &HitRecord,
        _rng: &mut dyn RngCore,
    ) -> Option<ScatterRecord> {
        None
    }

    fn emitted(&self, hit_record: &HitRecord, uv: Vec2, point: Vec3) -> Vec3 {
        if !hit_record.front_face {
            return Vec3::ZERO;
        }
        self.texture.value(uv, point) * self.strength
    }

    fn scattering_pdf(&self, ray_in: Ray, hit_record: &HitRecord, scattered: Ray) -> f32 {
        0.0
    }
}

#[derive(Clone, Debug)]
pub struct IsotropicMaterial {
    texture: Arc<dyn Texture>,
}

impl IsotropicMaterial {
    pub const fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Material for IsotropicMaterial {
    fn scatter(
        &self,
        _ray_in: Ray,
        hit_record: &HitRecord,
        rng: &mut dyn RngCore,
    ) -> Option<ScatterRecord> {
        let attenuation = self.texture.value(hit_record.uv, hit_record.point);
        let pdf = Arc::new(SpherePdf);

        Some(ScatterRecord {
            attenuation,
            pdf_or_skip_ray: Either::Left(pdf),
        })
    }

    fn emitted(&self, _hit_record: &HitRecord, _uv: Vec2, _point: Vec3) -> Vec3 {
        Vec3::ZERO
    }

    fn scattering_pdf(&self, ray_in: Ray, hit_record: &HitRecord, scattered: Ray) -> f32 {
        std::f32::consts::FRAC_1_PI * 0.25 // 1 / 4pi
    }
}

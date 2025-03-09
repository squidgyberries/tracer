use std::rc::Rc;

use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::util::{random_unit_vec3, vec3_near_zero};

use glam::{Vec3, vec3};

const DEFAULT_MATERIAL: Material = Material::new_lambertian(vec3(1.0, 0.0, 1.0), Vec3::ONE);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MaterialType {
    Lambertian,
    Metal,
    Dielectric,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Material {
    material_type: MaterialType,
    albedo: Vec3,
    diffuse_p: Vec3,
    metal_fuzz: f32,
    refraction_index: f32, // refractive index material / refractive index surroundings
}

impl Material {
    #[inline(always)]
    pub const fn new_lambertian(albedo: Vec3, diffuse_p: Vec3) -> Self {
        Self {
            material_type: MaterialType::Lambertian,
            albedo,
            diffuse_p,
            metal_fuzz: 0.0,
            refraction_index: 1.0,
            // ..Default::default()
        }
    }

    #[inline(always)]
    pub const fn new_metal(albedo: Vec3, fuzz: f32) -> Self {
        Self {
            material_type: MaterialType::Metal,
            albedo,
            diffuse_p: Vec3::ONE,
            metal_fuzz: fuzz,
            refraction_index: 1.0,
            // ..Default::default()
        }
    }

    #[inline(always)]
    pub const fn new_dielectric(refraction_index: f32) -> Self {
        Self {
            material_type: MaterialType::Dielectric,
            albedo: Vec3::ONE,
            diffuse_p: Vec3::ONE,
            metal_fuzz: 0.0,
            refraction_index,
            // ..Default::default()
        }
    }

    #[inline(always)]
    pub fn scatter(
        &self,
        ray_in: Ray,
        hit_record: &HitRecord,
        rng: &mut impl rand::Rng,
    ) -> (bool, Vec3, Ray) {
        match self.material_type {
            MaterialType::Lambertian => self.scatter_lambertian(ray_in, hit_record, rng),
            MaterialType::Metal => self.scatter_metal(ray_in, hit_record, rng),
            MaterialType::Dielectric => self.scatter_dielectric(ray_in, hit_record, rng),
        }
    }

    fn scatter_lambertian(
        &self,
        ray_in: Ray,
        hit_record: &HitRecord,
        rng: &mut impl rand::Rng,
    ) -> (bool, Vec3, Ray) {
        assert_eq!(self.material_type, MaterialType::Lambertian);

        let mut scatter_direction = hit_record.get_normal() + random_unit_vec3(rng);

        if vec3_near_zero(scatter_direction) {
            scatter_direction = hit_record.get_normal();
        }

        let mut attenuation = Vec3::ZERO;
        let random_scatter: f32 = rng.random();
        let scatter_r = random_scatter < self.diffuse_p.x;
        let scatter_g = random_scatter < self.diffuse_p.y;
        let scatter_b = random_scatter < self.diffuse_p.z;

        if scatter_r {
            attenuation.x = self.albedo.x / self.diffuse_p.x;
        }
        if scatter_g {
            attenuation.y = self.albedo.y / self.diffuse_p.y;
        }
        if scatter_b {
            attenuation.z = self.albedo.z / self.diffuse_p.z;
        }

        (
            scatter_r || scatter_g || scatter_b,
            attenuation,
            Ray::new(hit_record.point, scatter_direction),
        )
    }

    fn scatter_metal(
        &self,
        ray_in: Ray,
        hit_record: &HitRecord,
        rng: &mut impl rand::Rng,
    ) -> (bool, Vec3, Ray) {
        assert_eq!(self.material_type, MaterialType::Metal);

        let mut reflected = ray_in.direction.reflect(hit_record.get_normal());
        reflected = reflected.normalize() + (self.metal_fuzz * random_unit_vec3(rng));
        (true, self.albedo, Ray::new(hit_record.point, reflected))
    }

    fn scatter_dielectric(
        &self,
        ray_in: Ray,
        hit_record: &HitRecord,
        rng: &mut impl rand::Rng,
    ) -> (bool, Vec3, Ray) {
        assert_eq!(self.material_type, MaterialType::Dielectric);

        let ri = if hit_record.get_front_face() {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        // REIMPLEMENT
        let unit_direction = ray_in.direction.normalize();
        let cos_theta = (-unit_direction).dot(hit_record.get_normal()).min(1.0);

        let mut direction = unit_direction
            .normalize()
            .refract(hit_record.get_normal(), ri);
        if direction == Vec3::ZERO
            || Self::dielectric_reflectance(cos_theta, ri) > rng.random::<f32>()
        {
            direction = ray_in.direction.reflect(hit_record.get_normal());
        }

        (true, Vec3::ONE, Ray::new(hit_record.point, direction))
    }

    #[inline(always)]
    fn dielectric_reflectance(cosine: f32, refraction_index: f32) -> f32 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Default for Material {
    #[inline(always)]
    fn default() -> Self {
        DEFAULT_MATERIAL
    }
}

#[derive(Clone, Debug)]
pub struct SharedMaterial {
    inner: Rc<Material>,
}

impl SharedMaterial {
    #[inline(always)]
    pub fn new(material: Material) -> Self {
        Self {
            inner: Rc::new(material),
        }
    }
}

impl std::ops::Deref for SharedMaterial {
    type Target = Material;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl Default for SharedMaterial {
    #[inline(always)]
    fn default() -> Self {
        Self {
            inner: Rc::new(DEFAULT_MATERIAL),
        }
    }
}

use std::{fmt::Debug, sync::Arc};

use glam::Vec3;
use rand::{Rng, RngCore};

use crate::{
    hit::Hittable,
    onb::Onb,
    util::{random_cosine_direction, random_unit_vec3},
};

pub trait Pdf: Debug {
    fn value(&self, direction: Vec3, rng: &mut dyn RngCore) -> f32;

    fn generate(&self, rng: &mut dyn RngCore) -> Vec3;
}

#[derive(Debug)]
pub struct SpherePdf;

impl Pdf for SpherePdf {
    fn value(&self, _direction: Vec3, _rng: &mut dyn RngCore) -> f32 {
        std::f32::consts::FRAC_1_PI * 0.25 // 1 / 4pi
    }

    fn generate(&self, rng: &mut dyn RngCore) -> Vec3 {
        random_unit_vec3(rng)
    }
}

#[derive(Debug)]
pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn new(normal: Vec3) -> Self {
        Self {
            uvw: Onb::new(normal),
        }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: Vec3, _rng: &mut dyn RngCore) -> f32 {
        let cosine_theta = direction.normalize().dot(self.uvw.w);
        (cosine_theta * std::f32::consts::FRAC_1_PI).max(0.0)
    }

    fn generate(&self, rng: &mut dyn RngCore) -> Vec3 {
        self.uvw.transform(random_cosine_direction(rng))
    }
}

#[derive(Debug)]
pub struct HittablePdf {
    objects: Arc<dyn Hittable>,
    origin: Vec3,
}

impl HittablePdf {
    pub fn new(objects: Arc<dyn Hittable>, origin: Vec3) -> Self {
        Self { objects, origin }
    }
}

impl Pdf for HittablePdf {
    fn value(&self, direction: Vec3, rng: &mut dyn RngCore) -> f32 {
        self.objects.pdf_value(self.origin, direction, rng)
    }

    fn generate(&self, rng: &mut dyn RngCore) -> Vec3 {
        self.objects.random(self.origin, rng)
    }
}

#[derive(Debug)]
pub struct MixturePdf {
    pdf0: Arc<dyn Pdf>,
    pdf1: Arc<dyn Pdf>,
}

impl MixturePdf {
    pub fn new(pdf0: Arc<dyn Pdf>, pdf1: Arc<dyn Pdf>) -> Self {
        Self { pdf0, pdf1 }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: Vec3, rng: &mut dyn RngCore) -> f32 {
        0.5 * self.pdf0.value(direction, rng) + 0.5 * self.pdf1.value(direction, rng)
    }

    fn generate(&self, rng: &mut dyn RngCore) -> Vec3 {
        if rng.random_bool(0.5) {
            self.pdf0.generate(rng)
        } else {
            self.pdf1.generate(rng)
        }
    }
}

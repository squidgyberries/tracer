use std::{fmt::Debug, sync::Arc};

use glam::{Vec2, Vec3};
use rand::RngCore;

use crate::{aabb::Aabb, interval::Interval, material::Material, ray::Ray};

#[derive(Clone)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f32,
    pub uv: Vec2,
    pub front_face: bool,
}

impl HitRecord {
    // TODO: replace with constructor
    #[inline]
    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable: Send + Sync + Debug {
    fn hit(&self, ray: Ray, ray_t: Interval, rng: &mut dyn RngCore) -> Option<HitRecord>;

    fn bounding_box(&self) -> Aabb;

    fn pdf_value(&self, origin: Vec3, direction: Vec3, rng: &mut dyn RngCore) -> f32;

    fn random(&self, origin: Vec3, rng: &mut dyn RngCore) -> Vec3;
}

#[derive(Debug)]
pub struct EmptyHittable;

impl Hittable for EmptyHittable {
    fn hit(&self, _ray: Ray, _ray_t: Interval, _rng: &mut dyn RngCore) -> Option<HitRecord> {
        None
    }

    fn bounding_box(&self) -> Aabb {
        Aabb::EMPTY
    }

    fn pdf_value(&self, _origin: Vec3, _direction: Vec3, _rng: &mut dyn RngCore) -> f32 {
        0.0
    }

    fn random(&self, _origin: Vec3, _rng: &mut dyn RngCore) -> Vec3 {
        Vec3::X
    }
}

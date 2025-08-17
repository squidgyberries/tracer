use std::sync::Arc;

use crate::{aabb::Aabb, interval::Interval, material::Material, ray::Ray};

use glam::{Vec2, Vec3};

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

impl Default for HitRecord {
    #[inline(always)]
    fn default() -> Self {
        Self {
            point: Default::default(),
            normal: Default::default(),
            material: (*crate::material::DEFAULT_MATERIAL).clone(),
            t: Default::default(),
            uv: Default::default(),
            front_face: Default::default(),
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> Aabb;
}

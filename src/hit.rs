use crate::interval::Interval;
use crate::material::SharedMaterial;
use crate::ray::Ray;

use glam::Vec3;

#[derive(Clone, Default, Debug)]
pub struct HitRecord {
    pub point: Vec3,
    normal: Vec3,
    pub material: SharedMaterial,
    pub t: f32,
    front_face: bool,
}

impl HitRecord {
    #[inline(always)]
    pub const fn get_normal(&self) -> Vec3 {
        self.normal
    }

    #[inline(always)]
    pub const fn get_front_face(&self) -> bool {
        self.front_face
    }

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

pub trait Hittable {
    fn hit(&self, ray: Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool;
}

use crate::aabb::Aabb;
use crate::hit::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::SharedMaterial;
use crate::ray::Ray;

use glam::Vec3;

#[derive(Clone, Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: SharedMaterial,
    bbox: Aabb,
}

impl Sphere {
    #[inline(always)]
    pub fn new(center: Vec3, radius: f32, material: SharedMaterial) -> Self {
        let r = radius.max(0.0);
        let rvec = Vec3::splat(r);
        Self {
            center,
            radius: r,
            material,
            bbox: Aabb::from_corners(center - rvec, center + rvec),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        let origin_center = self.center - ray.origin;
        let a = ray.direction.length_squared();
        // let b = -2.0 * ray.direction.dot(origin_center);
        let h = ray.direction.dot(origin_center); // b = -2h
        let c = origin_center.length_squared() - self.radius * self.radius;
        // let discriminant = b * b - 4.0 * a * c;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        // (-b - discriminant.sqrt()) / (2.0 * a)
        let mut root = (h - discriminant.sqrt()) / a;

        if !ray_t.surrounds(root) {
            root = (h + discriminant.sqrt()) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - self.center) / self.radius;
        hit_record.material = self.material.clone();
        hit_record.point = point;
        hit_record.t = root;
        hit_record.set_face_normal(ray, outward_normal);
        true
    }

    #[inline(always)]
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

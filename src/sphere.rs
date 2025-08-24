use std::sync::Arc;

use crate::{
    aabb::Aabb,
    hit::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
};

use glam::{Vec2, Vec3};
use rand::RngCore;

#[derive(Clone, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Arc<dyn Material>,
    bbox: Aabb,
}

impl Sphere {
    #[inline(always)]
    pub fn new(center: Vec3, radius: f32, material: Arc<dyn Material>) -> Self {
        let r = radius.max(0.0);
        let rvec = Vec3::splat(r);
        Self {
            center,
            radius: r,
            material,
            bbox: Aabb::from_corners(center - rvec, center + rvec),
        }
    }

    pub fn get_sphere_uv(point: Vec3) -> Vec2 {
        // point: point on sphere of radius one centered at origin
        // u: [0,1] of angle around y axis from x=-1.
        // v: [0,1] of angle from y=-1 to y=+1.
        let theta = (-point.y).acos();
        let phi = (-point.z).atan2(point.x) + std::f32::consts::PI;

        Vec2::new(
            phi / (2.0 * std::f32::consts::PI),
            theta / std::f32::consts::PI,
        )
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: Ray,
        ray_t: Interval,
        hit_record: &mut HitRecord,
        _rng: &mut dyn RngCore,
    ) -> bool {
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
        hit_record.uv = Self::get_sphere_uv(outward_normal);
        true
    }

    #[inline(always)]
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

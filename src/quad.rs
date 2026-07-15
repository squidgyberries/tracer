use std::sync::Arc;

use glam::{Vec2, Vec3};
use rand::{Rng, RngCore};

use crate::{
    aabb::Aabb,
    hit::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
};

#[derive(Clone, Debug)]
pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    uvs: [Vec2; 4], // q, q + u, q + v, q + u + v
    material: Arc<dyn Material>,
    bbox: Aabb,
    normal: Vec3,
    d: f32,
    w: Vec3,
    area: f32,
}

impl Quad {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, uvs: [Vec2; 4], material: Arc<dyn Material>) -> Self {
        let bbox_diagonal1 = Aabb::from_corners(q, q + u + v);
        let bbox_diagonal2 = Aabb::from_corners(q + u, q + v);
        let bbox = Aabb::merged(bbox_diagonal1, bbox_diagonal2);

        let n = u.cross(v);
        let normal = n.normalize();
        let d = normal.dot(q);
        let w = n / n.dot(n);

        let area = n.length();

        Self {
            q,
            u,
            v,
            uvs,
            material,
            bbox,
            normal,
            d,
            w,
            area,
        }
    }

    pub const fn is_interior(&self, a: f32, b: f32) -> bool {
        const UNIT_INTERVAL: Interval = Interval::new(0.0, 1.0);
        UNIT_INTERVAL.contains(a) && UNIT_INTERVAL.contains(b)
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: Ray, ray_t: Interval, _rng: &mut dyn RngCore) -> Option<HitRecord> {
        let denom = self.normal.dot(ray.direction);

        // ray is parallel to plane
        if denom.abs() < 1e-8 {
            return None;
        }
        // back-face culling
        // if denom > -1e-8 {
        //     return false;
        // }

        let t = (self.d - self.normal.dot(ray.origin)) / denom;
        if !ray_t.surrounds(t) {
            return None;
        }

        // lies in quad?
        let hit_point = ray.at(t);
        let planar_hitpt_vector = hit_point - self.q;
        let alpha = self.w.dot(planar_hitpt_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hitpt_vector));

        if !self.is_interior(alpha, beta) {
            return None;
        }

        let uv = (1.0 - alpha) * (1.0 - beta) * self.uvs[0]
            + alpha * (1.0 - beta) * self.uvs[1]
            + (1.0 - alpha) * beta * self.uvs[2]
            + alpha * beta * self.uvs[3];

        Some(HitRecord {
            point: hit_point,
            normal: self.normal,
            material: self.material.clone(),
            t,
            uv,
            front_face: denom <= 0.0,
        })
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn pdf_value(&self, origin: Vec3, direction: Vec3, rng: &mut dyn RngCore) -> f32 {
        let Some(hit_record) = self.hit(
            Ray::new(origin, direction),
            Interval::new(0.001, f32::INFINITY),
            rng,
        ) else {
            return 0.0;
        };

        let distance_squared = hit_record.t * hit_record.t * direction.length_squared();
        let cosine = (direction.dot(hit_record.normal) / direction.length()).abs();

        distance_squared / (cosine * self.area)
    }

    fn random(&self, origin: Vec3, rng: &mut dyn RngCore) -> Vec3 {
        let p = self.q + (rng.random::<f32>() * self.u) + (rng.random::<f32>() * self.v);
        p - origin
    }
}

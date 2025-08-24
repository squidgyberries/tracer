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
}

impl Quad {
    #[inline(always)]
    pub fn new(q: Vec3, u: Vec3, v: Vec3, uvs: [Vec2; 4], material: Arc<dyn Material>) -> Self {
        let bbox_diagonal1 = Aabb::from_corners(q, q + u + v);
        let bbox_diagonal2 = Aabb::from_corners(q + u, q + v);
        let bbox = Aabb::merged(bbox_diagonal1, bbox_diagonal2);

        let n = u.cross(v);
        let normal = n.normalize();
        let d = normal.dot(q);
        let w = n / n.dot(n);
        // let w = n / n.length_squared();

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
        }
    }

    #[inline(always)]
    pub const fn is_interior(&self, a: f32, b: f32) -> bool {
        const UNIT_INTERVAL: Interval = Interval::new(0.0, 1.0);
        UNIT_INTERVAL.contains(a) && UNIT_INTERVAL.contains(b)
    }
}

impl Hittable for Quad {
    fn hit(
        &self,
        ray: Ray,
        ray_t: Interval,
        hit_record: &mut HitRecord,
        _rng: &mut dyn RngCore,
    ) -> bool {
        let denom = self.normal.dot(ray.direction);

        // ray is parallel to plane
        if denom.abs() < 1e-8 {
            return false;
        }
        // back-face culling
        // if denom > -1e-8 {
        //     return false;
        // }

        let t = (self.d - self.normal.dot(ray.origin)) / denom;
        if !ray_t.surrounds(t) {
            return false;
        }

        // lies in quad?
        let hit_point = ray.at(t);
        let planar_hitpt_vector = hit_point - self.q;
        let alpha = self.w.dot(planar_hitpt_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hitpt_vector));

        if !self.is_interior(alpha, beta) {
            return false;
        }

        let uv = (1.0 - alpha) * (1.0 - beta) * self.uvs[0]
            + alpha * (1.0 - beta) * self.uvs[1]
            + (1.0 - alpha) * beta * self.uvs[2]
            + alpha * beta * self.uvs[3];

        hit_record.t = t;
        hit_record.point = hit_point;
        hit_record.material = self.material.clone();
        hit_record.normal = self.normal;
        hit_record.front_face = denom <= 0.0;
        hit_record.uv = uv;
        true
    }

    #[inline(always)]
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

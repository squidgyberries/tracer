use std::sync::Arc;

use crate::{
    aabb::Aabb,
    hit::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
};

use glam::{Vec2, Vec3};

pub struct Triangle {
    a: Vec3,
    ab: Vec3, // replace with b and c?
    ac: Vec3,
    uvs: [Vec2; 3], // a, b, c
    material: Arc<Material>,
    bbox: Aabb,
    normal: Vec3,
    d: f32,
    w: Vec3,
}

impl Triangle {
    #[inline(always)]
    pub fn new(a: Vec3, ab: Vec3, ac: Vec3, uvs: [Vec2; 3], material: Arc<Material>) -> Self {
        // is there a better way?
        let bbox_diagonal1 = Aabb::from_corners(a, a + ab);
        let bbox_diagonal2 = Aabb::from_corners(a, a + ac);
        let bbox = Aabb::merged(bbox_diagonal1, bbox_diagonal2);

        let n = ab.cross(ac);
        let normal = n.normalize();
        let d = normal.dot(a);
        let w = n / n.dot(n);
        // let w = n / n.length_squared();

        Self {
            a,
            ab,
            ac,
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
        if a < 0.0 || b < 0.0 || a + b > 1.0 {
            return false;
        }
        true
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        let denom = self.normal.dot(ray.direction);

        // ray is parallel to plane
        if denom.abs() < 1e-8 {
            return false;
        }

        let t = (self.d - self.normal.dot(ray.origin)) / denom;
        // surrounds? contains?
        if !ray_t.surrounds(t) {
            return false;
        }

        // lies in quad?
        let hit_point = ray.at(t);
        let planar_hitpt_vector = hit_point - self.a;
        let alpha = self.w.dot(planar_hitpt_vector.cross(self.ac));
        let beta = self.w.dot(self.ab.cross(planar_hitpt_vector));

        if !self.is_interior(alpha, beta) {
            return false;
        }

        let uv = (1.0 - alpha - beta) * self.uvs[0]
            + alpha * self.uvs[1]
            + beta * self.uvs[2];

        hit_record.t = t;
        hit_record.point = hit_point;
        hit_record.material = self.material.clone();
        hit_record.set_face_normal(ray, self.normal);
        hit_record.uv = uv;
        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

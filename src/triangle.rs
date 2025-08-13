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
}

impl Triangle {
    #[inline(always)]
    pub fn new(a: Vec3, ab: Vec3, ac: Vec3, uvs: [Vec2; 3], material: Arc<Material>) -> Self {
        let bbox_diagonal1 = Aabb::from_corners(a, a + ab);
        let bbox_diagonal2 = Aabb::from_corners(a, a + ac);
        let bbox = Aabb::merged(bbox_diagonal1, bbox_diagonal2);

        let n = ab.cross(ac);
        let normal = n.normalize();

        Self {
            a,
            ab,
            ac,
            uvs,
            material,
            bbox,
            normal,
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
    // moller trumbore from scratchapixel
    fn hit(&self, ray: Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        let pvec = ray.direction.cross(self.ac);
        let det = self.ab.dot(pvec);

        // ray is parallel to plane
        if det.abs() < 1e-8 {
            return false;
        }
        // back-face culling
        // if det < 1e-8 {
        //     return false;
        // }

        let tvec = ray.origin - self.a;
        let u = tvec.dot(pvec) / det;
        if u < 0.0 || u > 1.0 {
            return false;
        }

        let qvec = tvec.cross(self.ab);
        let v = ray.direction.dot(qvec) / det;
        if v < 0.0 || u + v > 1.0 {
            return false;
        }

        let t = self.ac.dot(qvec) / det;
        if !ray_t.surrounds(t) {
            return false;
        }

        let hit_point = ray.at(t);

        let uv = (1.0 - u - v) * self.uvs[0]
            + u * self.uvs[1]
            + v * self.uvs[2];

        hit_record.t = t;
        hit_record.point = hit_point;
        hit_record.material = self.material.clone();
        hit_record.set_face_normal(ray, self.normal);
        hit_record.uv = uv;
        true
    }

    #[inline(always)]
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

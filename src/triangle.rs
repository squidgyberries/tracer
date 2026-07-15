use std::sync::Arc;

use glam::{Vec2, Vec3};
use rand::RngCore;

use crate::{
    aabb::Aabb,
    hit::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
};

#[derive(Debug)]
pub struct Triangle {
    a: Vec3,
    ab: Vec3, // replace with b and c?
    ac: Vec3,
    uvs: [Vec2; 3], // a, b, c
    material: Arc<dyn Material>,
    bbox: Aabb,
    normal: Vec3,
}

impl Triangle {
    pub fn new(a: Vec3, ab: Vec3, ac: Vec3, uvs: [Vec2; 3], material: Arc<dyn Material>) -> Self {
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
}

impl Hittable for Triangle {
    // moller trumbore from scratchapixel
    fn hit(&self, ray: Ray, ray_t: Interval, _rng: &mut dyn RngCore) -> Option<HitRecord> {
        let pvec = ray.direction.cross(self.ac);
        let det = self.ab.dot(pvec);

        // ray is parallel to plane
        if det.abs() < 1e-8 {
            return None;
        }
        // TODO: add option for back-face culling
        // back-face culling
        // if det < 1e-8 {
        //     return false;
        // }

        let tvec = ray.origin - self.a;
        let u = tvec.dot(pvec) / det;
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let qvec = tvec.cross(self.ab);
        let v = ray.direction.dot(qvec) / det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = self.ac.dot(qvec) / det;
        if !ray_t.surrounds(t) {
            return None;
        }

        let hit_point = ray.at(t);

        let uv = (1.0 - u - v) * self.uvs[0] + u * self.uvs[1] + v * self.uvs[2];

        Some(HitRecord {
            point: hit_point,
            normal: self.normal,
            material: self.material.clone(),
            t,
            uv,
            front_face: det >= 0.0,
        })
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn pdf_value(&self, _origin: Vec3, _direction: Vec3, _rng: &mut dyn RngCore) -> f32 {
        0.0
    }

    fn random(&self, _origin: Vec3, _rng: &mut dyn RngCore) -> Vec3 {
        Vec3::X
    }
}

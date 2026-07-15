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

#[derive(Debug)]
pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f32,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(
        boundary: Arc<dyn Hittable>,
        density: f32,
        phase_function: Arc<dyn Material>,
    ) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: Ray, ray_t: Interval, rng: &mut dyn RngCore) -> Option<HitRecord> {
        // Entry
        let Some(mut hit_record1) = self.boundary.hit(ray, Interval::EVERYTHING, rng) else {
            return None;
        };

        // Exit
        let Some(mut hit_record2) =
            self.boundary
                .hit(ray, Interval::new(hit_record1.t, f32::INFINITY), rng)
        else {
            return None;
        };

        if hit_record1.t < ray_t.min {
            hit_record1.t = ray_t.min;
        }
        if hit_record2.t > ray_t.max {
            hit_record2.t = ray_t.max;
        }

        if hit_record1.t >= hit_record2.t {
            return None;
        }

        let ray_length = ray.direction.length();
        let path_length = (hit_record2.t - hit_record1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rng.random::<f32>().ln();

        if hit_distance > path_length {
            return None;
        }

        let t = hit_record1.t + hit_distance / ray_length;

        Some(HitRecord {
            point: ray.at(t),
            normal: Vec3::ONE, // arbitrary
            material: self.phase_function.clone(),
            t,
            uv: Vec2::ZERO,
            front_face: true,
        })
    }

    fn bounding_box(&self) -> Aabb {
        self.boundary.bounding_box()
    }

    fn pdf_value(&self, _origin: Vec3, _direction: Vec3, _rng: &mut dyn RngCore) -> f32 {
        0.0
    }

    fn random(&self, _origin: Vec3, _rng: &mut dyn RngCore) -> Vec3 {
        Vec3::X
    }
}

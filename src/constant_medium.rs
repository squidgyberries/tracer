use std::{f32, sync::Arc};

use glam::Vec3;
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
    #[inline(always)]
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
    fn hit(
        &self,
        ray: Ray,
        ray_t: Interval,
        hit_record: &mut HitRecord,
        rng: &mut dyn RngCore,
    ) -> bool {
        let mut hit_record1 = HitRecord::default();
        let mut hit_record2 = HitRecord::default();

        // Entry
        if !self
            .boundary
            .hit(ray, Interval::EVERYTHING, &mut hit_record1, rng)
        {
            return false;
        }

        // Exit
        if !self.boundary.hit(
            ray,
            Interval::new(hit_record1.t, f32::INFINITY),
            &mut hit_record2,
            rng,
        ) {
            return false;
        }

        if hit_record1.t < ray_t.min {
            hit_record1.t = ray_t.min;
        }
        if hit_record2.t > ray_t.max {
            hit_record2.t = ray_t.max;
        }

        if hit_record1.t >= hit_record2.t {
            return false;
        }

        let ray_length = ray.direction.length();
        let path_length = (hit_record2.t - hit_record1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rng.random::<f32>().ln();

        if hit_distance > path_length {
            return false;
        }

        hit_record.t = hit_record1.t + hit_distance / ray_length;
        hit_record.point = ray.at(hit_record.t);

        hit_record.normal = Vec3::ONE; // arbitrary
        hit_record.front_face = true;
        hit_record.material = self.phase_function.clone();

        true
    }

    #[inline(always)]
    fn bounding_box(&self) -> Aabb {
        self.boundary.bounding_box()
    }
}

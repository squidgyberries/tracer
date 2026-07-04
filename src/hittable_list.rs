use std::sync::Arc;

use glam::Vec3;
use rand::{Rng, RngCore};

use crate::{
    aabb::Aabb,
    hit::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

#[derive(Debug)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: Aabb,
}

impl HittableList {
    pub const fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: Aabb::EMPTY,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            objects: Vec::with_capacity(capacity),
            bbox: Aabb::EMPTY,
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object.clone());
        self.bbox.merge(object.bounding_box());
    }

    pub fn update_bounding_box(&mut self) {
        let mut min = Vec3::MAX;
        let mut max = Vec3::MIN;
        for object in &self.objects {
            let corners = object.bounding_box().get_corners();
            min = min.min(corners.0);
            max = max.max(corners.1);
        }
        self.bbox = Aabb::from_corners(min, max);
    }

    pub fn from_vec(objects: &mut Vec<Arc<dyn Hittable>>) -> Self {
        let mut list = Self::new();
        list.objects = objects.clone();
        list.update_bounding_box();
        list
    }
}

impl Hittable for HittableList {
    fn hit(
        &self,
        ray: Ray,
        ray_t: Interval,
        hit_record: &mut HitRecord,
        rng: &mut dyn RngCore,
    ) -> bool {
        let mut temp_hit_record = HitRecord::default();
        let mut temp_hit;
        let mut hit = false;
        let mut closest = ray_t.max;

        for object in &self.objects {
            temp_hit = object.hit(
                ray,
                Interval::new(ray_t.min, closest),
                &mut temp_hit_record,
                rng,
            );
            if temp_hit {
                hit = true;
                closest = temp_hit_record.t;
                *hit_record = temp_hit_record.clone();
            }
        }

        hit
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn pdf_value(&self, origin: Vec3, direction: Vec3, rng: &mut dyn RngCore) -> f32 {
        let weight = 1.0 / self.objects.len() as f32; // TODO: add specifiable weights
        let mut sum = 0.0;

        for object in &self.objects {
            sum += weight * object.pdf_value(origin, direction, rng);
        }

        sum
    }

    fn random(&self, origin: Vec3, rng: &mut dyn RngCore) -> Vec3 {
        self.objects[rng.random_range(0..self.objects.len())].random(origin, rng)
    }
}

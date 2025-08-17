use std::sync::Arc;

use crate::{
    aabb::Aabb,
    hit::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: Aabb,
}

impl HittableList {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: Aabb::EMPTY,
        }
    }

    #[inline(always)]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            objects: Vec::with_capacity(capacity),
            bbox: Aabb::EMPTY,
        }
    }

    #[inline(always)]
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object.clone());
        self.bbox.merge(object.bounding_box());
    }

    #[inline(always)]
    pub fn update_bounding_box(&mut self) {
        for object in &self.objects {
            self.bbox.merge(object.bounding_box());
        }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        let mut temp_hit_record = HitRecord::default();
        let mut temp_hit;
        let mut hit = false;
        let mut closest = ray_t.max;

        for object in &self.objects {
            temp_hit = object.hit(ray, Interval::new(ray_t.min, closest), &mut temp_hit_record);
            if temp_hit {
                hit = true;
                closest = temp_hit_record.t;
                *hit_record = temp_hit_record.clone();
            }
        }

        hit
    }

    #[inline(always)]
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

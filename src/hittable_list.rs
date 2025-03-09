use std::rc::Rc;

use crate::hit::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    #[inline(always)]
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
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
}

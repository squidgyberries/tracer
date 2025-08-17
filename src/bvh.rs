use crate::{
    aabb::Aabb,
    hit::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    ray::Ray,
};

use std::sync::Arc;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(
        objects: &mut Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
    ) -> Self {
        let mut bbox = Aabb::EMPTY;
        for object in &objects[start..end] {
            bbox.merge(object.bounding_box());
        }

        let axis = bbox.longest_axis();

        let cmp_fn = if axis == 0 {
            Self::box_x_compare
        } else if axis == 1 {
            Self::box_y_compare
        } else {
            Self::box_z_compare
        };

        let object_span = end - start;

        let left;
        let right;
        if object_span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        } else if object_span == 2 {
            left = objects[start].clone();
            right = objects[start + 1].clone();
        } else {
            objects[start..end].sort_by(cmp_fn);

            let mid = start + object_span / 2;
            left = Arc::new(Self::new(objects, start, mid));
            right = Arc::new(Self::new(objects, mid, end));
        }

        Self { left, right, bbox }
    }

    #[inline(always)]
    pub fn from_hittable_list(mut list: HittableList) -> Self {
        let len = list.objects.len();
        Self::new(&mut list.objects, 0, len)
    }

    #[inline]
    fn box_compare(
        a: &Arc<dyn Hittable>,
        b: &Arc<dyn Hittable>,
        axis_index: usize,
    ) -> std::cmp::Ordering {
        let a_axis_interval = a.bounding_box()[axis_index];
        let b_axis_interval = b.bounding_box()[axis_index];
        a_axis_interval.min.total_cmp(&b_axis_interval.min)
    }

    #[inline(always)]
    fn box_x_compare(
        a: &Arc<dyn Hittable>,
        b: &Arc<dyn Hittable>,
    ) -> std::cmp::Ordering {
        Self::box_compare(a, b, 0)
    }

    #[inline(always)]
    fn box_y_compare(
        a: &Arc<dyn Hittable>,
        b: &Arc<dyn Hittable>,
    ) -> std::cmp::Ordering {
        Self::box_compare(a, b, 1)
    }

    #[inline(always)]
    fn box_z_compare(
        a: &Arc<dyn Hittable>,
        b: &Arc<dyn Hittable>,
    ) -> std::cmp::Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BvhNode {
    #[inline]
    fn hit(&self, ray: Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        if !self.bbox.hit(ray, ray_t) {
            return false;
        }

        let hit_left = self.left.hit(ray, ray_t, hit_record);
        let hit_right = self.right.hit(
            ray,
            Interval::new(ray_t.min, if hit_left { hit_record.t } else { ray_t.max }),
            hit_record,
        );

        hit_left || hit_right
    }

    #[inline(always)]
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

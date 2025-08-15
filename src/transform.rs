use std::sync::Arc;

use crate::{
    aabb::Aabb,
    hit::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

use glam::{Mat4, Vec3};

pub struct Transform {
    object: Arc<dyn Hittable + Send + Sync>,
    transform: Mat4,
    transform_inv: Mat4,
    transform_inv_t: Mat4,
    bbox: Aabb,
}

impl Transform {
    pub fn new(object: Arc<dyn Hittable + Send + Sync>, transform: &Mat4) -> Self {
        let transform_inv = transform.inverse();
        let transform_inv_t = transform_inv.transpose();

        let mut bbox = object.bounding_box();

        let mut min = Vec3::INFINITY;
        let mut max = Vec3::NEG_INFINITY;

        // Transform bounding box corners
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let corner = Vec3::new(
                        if i == 1 { bbox.x.max } else { bbox.x.min },
                        if j == 1 { bbox.y.max } else { bbox.y.min },
                        if k == 1 { bbox.z.max } else { bbox.z.min },
                    );

                    let transformed = transform.transform_point3(corner);

                    min = min.min(transformed);
                    max = max.max(transformed);
                }
            }
        }

        bbox = Aabb::from_corners(min, max);

        Self {
            object,
            transform: *transform,
            transform_inv,
            transform_inv_t,
            bbox,
        }
    }
}

impl Hittable for Transform {
    fn hit(&self, ray: Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        let ray_transformed = Ray::new(
            self.transform_inv.transform_point3(ray.origin),
            self.transform_inv.transform_vector3(ray.direction),
        );

        if !self.object.hit(ray_transformed, ray_t, hit_record) {
            return false;
        }

        hit_record.point = self.transform.transform_point3(hit_record.point);
        hit_record.normal = self
            .transform_inv_t
            .transform_vector3(hit_record.normal)
            .normalize();

        true
    }

    #[inline(always)]
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

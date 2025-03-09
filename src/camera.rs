use crate::color::vec3_to_rgb8;
use crate::hit::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::util::{deg_to_rad, random_in_unit_disk};

use glam::{Vec3, vec3};
use rayon::iter::ParallelIterator;

#[derive(Debug)]
pub struct Camera {
    image_width: i32,
    image_height: i32,
    aspect_ratio: f32, // remove?
    vfov: f32,
    lookfrom: Vec3,
    lookat: Vec3,
    view_up: Vec3,
    center: Vec3,
    samples_per_pixel: i32,
    pixel_samples_scale: f32, // rename?
    max_depth: i32,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    // Camera frame basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_angle: f32,
    focus_dist: f32,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn render_threaded(
        &self,
        world: &(impl Hittable + Send + Sync),
        imgbuf: &mut image::RgbImage,
    ) {
        imgbuf
            .par_enumerate_pixels_mut()
            .for_each_init(rand::rng, |rng, (x, y, pixel)| {
                let mut pixel_color = Vec3::ZERO;
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x as i32, y as i32, rng);
                    pixel_color += Self::ray_color(ray, self.max_depth, world, rng);
                }

                *pixel = vec3_to_rgb8(self.pixel_samples_scale * pixel_color);
            });
    }

    pub fn render(
        &self,
        world: &impl Hittable,
        imgbuf: &mut image::RgbImage,
        rng: &mut impl rand::Rng,
    ) {
        let mut pixel_num = 1;
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            eprint!(
                "\rPixel {}/{}",
                pixel_num,
                self.image_width * self.image_height
            );

            let mut pixel_color = Vec3::ZERO;
            for _ in 0..self.samples_per_pixel {
                let ray = self.get_ray(x as i32, y as i32, rng);
                pixel_color += Self::ray_color(ray, self.max_depth, world, rng);
            }

            *pixel = vec3_to_rgb8(self.pixel_samples_scale * pixel_color);
            pixel_num += 1;
        }
        eprintln!("");
        eprintln!("Done.");
    }

    pub fn new(
        image_width: i32,
        image_height: i32,
        vfov: f32,
        lookfrom: Vec3,
        lookat: Vec3,
        view_up: Vec3,
        defocus_angle: f32,
        focus_dist: f32,
        samples_per_pixel: i32,
        max_depth: i32,
    ) -> Self {
        let aspect_ratio = image_width as f32 / image_height as f32;

        let center = lookfrom;

        // Determine viewport dimensions
        // let focal_length: f32 = (lookfrom - lookat).length();
        let theta = deg_to_rad(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height: f32 = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * aspect_ratio;

        // Calculate the u, v, w unit basis vectors for the camera coordinate frame
        let w = (lookfrom - lookat).normalize(); // into camera from lookat
        let u = view_up.cross(w).normalize(); // to camera right
        let v = w.cross(u); // to camera up

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u: Vec3 = viewport_width * u;
        let viewport_v: Vec3 = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u: Vec3 = viewport_u / image_width as f32;
        let pixel_delta_v: Vec3 = viewport_v / image_height as f32;

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            center - focus_dist * w - viewport_u / 2 as f32 - viewport_v / 2 as f32;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors
        let defocus_radius = focus_dist * deg_to_rad(defocus_angle / 2.0).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            image_width,
            image_height,
            aspect_ratio,
            vfov,
            lookfrom,
            lookat,
            view_up,
            center,
            samples_per_pixel,
            pixel_samples_scale: 1.0 / samples_per_pixel as f32,
            max_depth,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            u,
            v,
            w,
            defocus_angle,
            focus_dist,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    // Construct a camera ray originating from the defocus disk and directed at a randomly sampled point around the pixel location x, y
    fn get_ray(&self, x: i32, y: i32, rng: &mut impl rand::Rng) -> Ray {
        let offset = Self::sample_square(rng);
        let pixel_sample = self.pixel00_loc
            + (x as f32 + offset.x) * self.pixel_delta_u
            + (y as f32 + offset.y) * self.pixel_delta_v;

        // let ray_origin = self.center;
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample(rng)
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    // random point in [-0.5, -0.5]-[+0.5, +0.5] unit square
    #[inline(always)]
    fn sample_square(rng: &mut impl rand::Rng) -> Vec3 {
        vec3(
            rng.random_range(-0.5..=0.5),
            rng.random_range(-0.5..=0.5),
            0.0,
        )
    }

    // random point in the camera defocus disk
    #[inline(always)]
    fn defocus_disk_sample(&self, rng: &mut impl rand::Rng) -> Vec3 {
        let point = random_in_unit_disk(rng);
        self.center + (point.x * self.defocus_disk_u) + (point.y * self.defocus_disk_v)
    }

    fn ray_color(ray: Ray, depth: i32, world: &impl Hittable, rng: &mut impl rand::Rng) -> Vec3 {
        if depth <= 0 {
            return Vec3::ZERO;
        }

        let mut hit_record = HitRecord::default();
        let hit = world.hit(ray, Interval::new(0.001, f32::INFINITY), &mut hit_record);
        if hit {
            let (scatter, attenuation, scattered_ray) =
                hit_record.material.scatter(ray, &hit_record, rng);
            if scatter {
                return attenuation * Self::ray_color(scattered_ray, depth - 1, world, rng);
            }
            return Vec3::ZERO;
        }

        let unit_direction = ray.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Vec3::ONE + a * vec3(0.5, 0.7, 1.0)
        // (1.0 - a) * Vec3::ONE + a * vec3(1.0, 0.5, 0.5)
        // Vec3::ONE
    }
}

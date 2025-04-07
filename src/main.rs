mod aabb;
mod bvh;
mod camera;
mod color;
mod hit;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod util;

use std::sync::Arc;

use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::material::{Material, SharedMaterial};
use crate::sphere::Sphere;
use crate::util::random_vec3;

use glam::{Vec3, vec3};
use rand::Rng;

fn cover() {
    let mut world = HittableList::new();

    let ground_material =
        SharedMaterial::new(Material::new_lambertian(vec3(0.5, 0.5, 0.5), Vec3::ONE));
    world.add(Arc::new(Sphere::new(
        vec3(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let mut rng = rand::rng();

    for x in -10..=10 {
        for z in -10..=10 {
            let center = vec3(
                x as f32 + 0.5 * rng.random_range(-1.0..=1.0),
                0.2,
                z as f32 + 0.5 * rng.random_range(-1.0..=1.0),
            );

            let random_mat = rng.random::<f32>();
            let material = if random_mat < 0.5 {
                let albedo = random_vec3(Vec3::ZERO, Vec3::ONE, &mut rng)
                    * random_vec3(Vec3::ZERO, Vec3::ONE, &mut rng);
                SharedMaterial::new(Material::new_lambertian(albedo, Vec3::ONE))
            } else if random_mat < 0.75 {
                let albedo = random_vec3(Vec3::splat(0.5), Vec3::ONE, &mut rng);
                let fuzz = rng.random_range(0.0..0.5);
                SharedMaterial::new(Material::new_metal(albedo, fuzz))
            } else {
                SharedMaterial::new(Material::new_dielectric(1.5))
            };
            world.add(Arc::new(Sphere::new(center, 0.2, material)));
        }
    }

    let material1 = SharedMaterial::new(Material::new_dielectric(1.5));
    world.add(Arc::new(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = SharedMaterial::new(Material::new_lambertian(vec3(0.4, 0.2, 0.1), Vec3::ONE));
    world.add(Arc::new(Sphere::new(vec3(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = SharedMaterial::new(Material::new_metal(vec3(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(vec3(4.0, 1.0, 0.0), 1.0, material3)));

    let mut bvh_world = HittableList::new();
    bvh_world.add(Arc::new(BvhNode::from_hittable_list(world)));

    let image_width = 800;
    let image_height = 600;

    let camera = Camera::new(
        image_width,
        image_height,
        60.0,
        vec3(0.0, 1.2, -7.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        0.6,
        7.0,
        500,
        20,
    );

    let mut imgbuf = image::RgbImage::new(image_width as u32, image_height as u32);

    // rayon::ThreadPoolBuilder::new().num_threads(10).build_global().unwrap();

    // camera.render(&world, &mut imgbuf, &mut rng);
    // camera.render_threaded(&world, &mut imgbuf);
    camera.render_threaded(&bvh_world, &mut imgbuf);
    // camera.render_threaded(&BvhNode::from_hittable_list(world), &mut imgbuf);

    imgbuf.save("output.png").unwrap();
}

fn main() {
    cover();
    // thing();
}

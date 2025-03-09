mod camera;
mod color;
mod hit;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod util;

use std::rc::Rc;
use std::sync::Arc;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::material::{Material, SharedMaterial};
use crate::sphere::Sphere;
use crate::util::random_vec3;

use glam::{Vec3, vec3};
use rand::Rng;

// const IMAGE_WIDTH: i32 = 800;
// const IMAGE_HEIGHT: i32 = 600;

fn cover() {
    let mut world = HittableList::new();

    let ground_material =
        SharedMaterial::new(Material::new_lambertian(vec3(0.5, 0.5, 0.5), Vec3::ONE));
    world.add(Rc::new(Sphere::new(
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
            world.add(Rc::new(Sphere::new(center, 0.2, material)));
        }
    }

    let material1 = SharedMaterial::new(Material::new_dielectric(1.5));
    world.add(Rc::new(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = SharedMaterial::new(Material::new_lambertian(vec3(0.4, 0.2, 0.1), Vec3::ONE));
    world.add(Rc::new(Sphere::new(vec3(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = SharedMaterial::new(Material::new_metal(vec3(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(vec3(4.0, 1.0, 0.0), 1.0, material3)));

    let image_width = 1280;
    let image_height = 720;

    let camera = Camera::new(
        image_width,
        image_height,
        60.0,
        vec3(0.0, 1.2, -7.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        0.6,
        7.0,
        10,
        5,
    );

    let mut imgbuf = image::ImageBuffer::new(image_width as u32, image_height as u32);

    camera.render(&world, &mut imgbuf, &mut rng);

    imgbuf.save("output.png").unwrap();
}

fn main() {
    cover();
    // World
    // let mut world = HittableList::new();

    // let r = (std::f32::consts::PI / 4.0).cos();

    // let material_left =
    //     SharedMaterial::new(Material::new_lambertian(vec3(0.0, 0.0, 1.0), Vec3::ONE));
    // let material_right =
    //     SharedMaterial::new(Material::new_lambertian(vec3(1.0, 0.0, 0.0), Vec3::ONE));

    // world.add(Rc::new(Sphere::new(vec3(-r, 0.0, -1.0), r, material_left)));
    // world.add(Rc::new(Sphere::new(vec3(r, 0.0, -1.0), r, material_right)));

    // let material_ground =
    //     SharedMaterial::new(Material::new_lambertian(vec3(0.8, 0.8, 0.0), Vec3::ONE));
    // let material_center =
    //     SharedMaterial::new(Material::new_lambertian(vec3(0.1, 0.2, 0.5), Vec3::ONE));
    // let material_left = SharedMaterial::new(Material::new_dielectric(1.5));
    // let material_left = SharedMaterial::new(Material::new_dielectric(1.50));
    // let material_bubble = SharedMaterial::new(Material::new_dielectric(1.00 / 1.50));
    // let material_right = SharedMaterial::new(Material::new_metal(vec3(0.8, 0.6, 0.2), 1.0));
    // let material_top = SharedMaterial::new(Material::new_metal(vec3(0.9, 0.9, 0.9), 0.0));
    // let material1 = SharedMaterial::new(Material::new_lambertian(vec3(0.0, 0.6, 0.7), Vec3::ONE));
    // let material2 = SharedMaterial::new(Material::new_lambertian(vec3(0.8, 0.3, 0.3), Vec3::ONE));

    // world.add(Rc::new(Sphere::new(
    //     vec3(0.0, -100.5, -1.0),
    //     100.0,
    //     material_ground,
    // )));
    // world.add(Rc::new(Sphere::new(
    //     vec3(0.0, 0.0, -1.2),
    //     0.5,
    //     material_center,
    // )));
    // world.add(Rc::new(Sphere::new(
    //     vec3(-1.0, 0.0, -1.0),
    //     0.5,
    //     material_left,
    // )));
    // world.add(Rc::new(Sphere::new(
    //     vec3(-1.0, 0.0, -1.0),
    //     0.4,
    //     material_bubble,
    // )));
    // world.add(Rc::new(Sphere::new(
    //     vec3(1.0, 0.0, -1.0),
    //     0.5,
    //     material_right,
    // )));
    // world.add(Rc::new(Sphere::new(
    //     vec3(0.0, 1.0, -1.0),
    //     0.5,
    //     material_top,
    // )));

    // let camera = Camera::new(
    //     IMAGE_WIDTH,
    //     IMAGE_HEIGHT,
    //     20.0,
    //     vec3(-2.0, 2.0, 1.0),
    //     vec3(0.0, 0.0, -1.0),
    //     vec3(0.0, 1.0, 0.0),
    //     0.0,
    //     3.4,
    //     100,
    //     50,
    // );

    // let mut rng = rand::rng(); // move to camera?

    // let mut imgbuf = image::ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);

    // camera.render(&world, &mut imgbuf, &mut rng);

    // imgbuf.save("output.png").unwrap();
}

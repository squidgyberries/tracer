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
mod texture;
mod util;

use std::sync::Arc;

use crate::{
    bvh::BvhNode,
    camera::Camera,
    hittable_list::HittableList,
    material::Material,
    sphere::Sphere,
    texture::{SolidColor, SpatialChecker, ImageTexture},
    util::random_vec3,
};

use glam::{Vec3, vec3};
use rand::Rng;

fn spheres() {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Material::new_lambertian(
        // Arc::new(SolidColor::from_rgb(0.5, 0.5, 0.5)),
        Arc::new(SpatialChecker::new(
            1.0,
            Arc::new(SolidColor::from_rgb(0.1, 0.4, 0.1)),
            Arc::new(SolidColor::from_rgb(0.9, 0.9, 0.9)),
        )),
        Vec3::ONE,
    ));
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
                Arc::new(Material::new_lambertian(
                    Arc::new(SolidColor::new(albedo)),
                    Vec3::ONE,
                ))
            } else if random_mat < 0.75 {
                let albedo = random_vec3(Vec3::splat(0.5), Vec3::ONE, &mut rng);
                let fuzz = rng.random_range(0.0..0.5);
                Arc::new(Material::new_metal(Arc::new(SolidColor::new(albedo)), fuzz))
            } else {
                Arc::new(Material::new_dielectric(1.5))
            };
            world.add(Arc::new(Sphere::new(center, 0.2, material)));
        }
    }

    let material1 = Arc::new(Material::new_dielectric(1.5));
    world.add(Arc::new(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, material1)));

    let earth_texture = Arc::new(ImageTexture::load("8081_earthmap10k.jpg").unwrap());
    let earth_material = Arc::new(Material::new_lambertian(earth_texture, Vec3::ONE));
    let globe = Sphere::new(vec3(-4.0, 1.0, 0.0), 1.0, earth_material);
    // let material2 = Arc::new(Material::new_lambertian(
    //     Arc::new(SolidColor::from_rgb(0.4, 0.2, 0.1)),
    //     Vec3::ONE,
    // ));
    world.add(Arc::new(globe));

    let material3 = Arc::new(Material::new_metal(
        Arc::new(SolidColor::from_rgb(0.7, 0.6, 0.5)),
        0.0,
    ));
    world.add(Arc::new(Sphere::new(vec3(4.0, 1.0, 0.0), 1.0, material3)));

    let bvh = BvhNode::from_hittable_list(world);

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
        50,
    );

    let mut imgbuf = image::RgbImage::new(image_width as u32, image_height as u32);

    // rayon::ThreadPoolBuilder::new().num_threads(10).build_global().unwrap();

    camera.render_threaded(&bvh, &mut imgbuf);

    imgbuf.save("output.png").unwrap();
}

fn checkered_spheres() {
    let mut world = HittableList::new();

    let checker = Arc::new(SpatialChecker::new(
        0.32,
        Arc::new(SolidColor::from_rgb(0.1, 0.4, 0.1)),
        Arc::new(SolidColor::from_rgb(0.9, 0.9, 0.9)),
    ));

    world.add(Arc::new(Sphere::new(
        vec3(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Material::new_lambertian(checker.clone(), Vec3::ONE)),
    )));
    world.add(Arc::new(Sphere::new(
        vec3(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Material::new_lambertian(checker, Vec3::ONE)),
    )));

    // let bvh = BvhNode::from_hittable_list(world);

    let image_width = 800;
    let image_height = 600;

    let camera = Camera::new(
        image_width,
        image_height,
        20.0,
        vec3(13.0, 2.0, 3.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        0.0,
        7.0,
        100,
        10,
    );

    let mut imgbuf = image::RgbImage::new(image_width as u32, image_height as u32);

    // rayon::ThreadPoolBuilder::new().num_threads(10).build_global().unwrap();

    camera.render_threaded(&world, &mut imgbuf);

    imgbuf.save("output.png").unwrap();
}

fn earth() {
    let earth_texture = Arc::new(ImageTexture::load("8081_earthmap10k.jpg").unwrap());
    let earth_material = Arc::new(Material::new_lambertian(earth_texture, Vec3::ONE));
    let globe = Sphere::new(Vec3::ZERO, 2.0, earth_material);

    let image_width = 800;
    let image_height = 600;

    let camera = Camera::new(
        image_width,
        image_height,
        20.0,
        vec3(0.0, 0.0, 12.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        0.0,
        7.0,
        100,
        10,
    );

    let mut imgbuf = image::RgbImage::new(image_width as u32, image_height as u32);

    camera.render_threaded(&globe, &mut imgbuf);

    imgbuf.save("output.png").unwrap();
}

fn main() {
    match 1 {
        1 => spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        _ => spheres(),
    }
}

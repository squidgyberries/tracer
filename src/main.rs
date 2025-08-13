mod aabb;
mod bvh;
mod camera;
mod color;
mod hit;
mod hittable_list;
mod interval;
mod material;
mod mesh;
mod quad;
mod ray;
mod sphere;
mod texture;
mod triangle;
mod util;

use std::sync::Arc;

use crate::{
    bvh::BvhNode,
    camera::Camera,
    hittable_list::HittableList,
    material::Material,
    quad::Quad,
    sphere::Sphere,
    texture::{ImageTexture, SolidColor, SpatialChecker},
    triangle::Triangle,
    util::random_vec3,
    mesh::load_obj_meshes,
};

use glam::{Vec2, Vec3, vec2, vec3};
use rand::Rng;

fn spheres() -> anyhow::Result<()> {
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

    let earth_texture = Arc::new(ImageTexture::load("resources/8081_earthmap10k.jpg")?);
    let earth_material = Arc::new(Material::new_lambertian(earth_texture.clone(), Vec3::ONE));
    let globe = Sphere::new(vec3(-4.0, 1.0, 0.0), 1.0, earth_material);
    // let material2 = Arc::new(Material::new_lambertian(
    //     Arc::new(SolidColor::from_rgb(0.4, 0.2, 0.1)),
    //     Vec3::ONE,
    // ));
    world.add(Arc::new(globe));

    // let material3 = Arc::new(Material::new_metal(
    //     Arc::new(SolidColor::from_rgb(0.7, 0.6, 0.5)),
    //     0.0,
    // ));
    let material3 = Arc::new(Material::new_metal(earth_texture, 0.0));
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
        10,
        vec3(0.7, 0.8, 1.0),
    );

    let mut imgbuf = image::RgbImage::new(image_width as u32, image_height as u32);

    // rayon::ThreadPoolBuilder::new().num_threads(10).build_global()?;

    camera.render_threaded(&bvh, &mut imgbuf);

    imgbuf.save("output.png")?;
    Ok(())
}

fn checkered_spheres() -> anyhow::Result<()> {
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
        vec3(0.7, 0.8, 1.0),
    );

    let mut imgbuf = image::RgbImage::new(image_width as u32, image_height as u32);

    // rayon::ThreadPoolBuilder::new().num_threads(10).build_global()?;

    camera.render_threaded(&world, &mut imgbuf);

    imgbuf.save("output.png")?;
    Ok(())
}

fn earth() -> anyhow::Result<()> {
    let earth_texture = Arc::new(ImageTexture::load("resources/8081_earthmap10k.jpg")?);
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
        vec3(0.7, 0.8, 1.0),
    );

    let mut imgbuf = image::RgbImage::new(image_width as u32, image_height as u32);

    camera.render_threaded(&globe, &mut imgbuf);

    imgbuf.save("output.png")?;
    Ok(())
}

fn quads() -> anyhow::Result<()> {
    let mut world = HittableList::new();

    let left_red = Arc::new(Material::new_lambertian(
        Arc::new(SolidColor::from_rgb(1.0, 0.2, 0.2)),
        Vec3::ONE,
    ));
    // let back_green = Arc::new(Material::new_lambertian(
    //     Arc::new(SolidColor::from_rgb(0.2, 1.0, 0.2)),
    //     Vec3::ONE,
    // ));
    let back_earth = Arc::new(Material::new_lambertian(
        Arc::new(ImageTexture::load("resources/8081_earthmap10k.jpg")?),
        Vec3::ONE,
    ));
    let right_blue = Arc::new(Material::new_lambertian(
        Arc::new(SolidColor::from_rgb(0.2, 0.2, 1.0)),
        Vec3::ONE,
    ));
    let top_orange = Arc::new(Material::new_lambertian(
        Arc::new(SolidColor::from_rgb(1.0, 0.5, 0.0)),
        Vec3::ONE,
    ));
    let bottom_teal = Arc::new(Material::new_lambertian(
        Arc::new(SolidColor::from_rgb(0.2, 0.8, 0.8)),
        Vec3::ONE,
    ));

    world.add(Arc::new(Quad::new(
        vec3(-3.0, -2.0, 5.0),
        vec3(0.0, 0.0, -4.0),
        vec3(0.0, 4.0, 0.0),
        [
            vec2(0.0, 0.0),
            vec2(1.0, 0.0),
            vec2(0.0, 1.0),
            vec2(1.0, 1.0),
        ],
        left_red,
    )));
    world.add(Arc::new(Quad::new(
        vec3(-2.0, -2.0, 0.0),
        vec3(4.0, 0.0, 0.0),
        vec3(0.0, 4.0, 0.0),
        [
            vec2(0.0, 0.0),
            vec2(1.0, 0.0),
            vec2(0.0, 1.0),
            vec2(1.0, 1.0),
        ],
        back_earth.clone(),
    )));
    world.add(Arc::new(Triangle::new(
        vec3(-2.0, -2.0, 0.0),
        vec3(4.0, 0.0, 0.0),
        vec3(0.0, 4.0, 0.0),
        [vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(0.0, 1.0)],
        back_earth,
    )));
    world.add(Arc::new(Quad::new(
        vec3(3.0, -2.0, 1.0),
        vec3(0.0, 0.0, 4.0),
        vec3(0.0, 4.0, 0.0),
        [
            vec2(0.0, 0.0),
            vec2(1.0, 0.0),
            vec2(0.0, 1.0),
            vec2(1.0, 1.0),
        ],
        right_blue,
    )));
    world.add(Arc::new(Quad::new(
        vec3(-2.0, 3.0, 1.0),
        vec3(4.0, 0.0, 0.0),
        vec3(0.0, 0.0, 4.0),
        [
            vec2(0.0, 0.0),
            vec2(1.0, 0.0),
            vec2(0.0, 1.0),
            vec2(1.0, 1.0),
        ],
        top_orange,
    )));
    world.add(Arc::new(Quad::new(
        vec3(-2.0, -3.0, 5.0),
        vec3(4.0, 0.0, 0.0),
        vec3(0.0, 0.0, -4.0),
        [
            vec2(0.0, 0.0),
            vec2(1.0, 0.0),
            vec2(0.0, 1.0),
            vec2(1.0, 1.0),
        ],
        bottom_teal,
    )));

    let image_width = 800;
    let image_height = 800;

    let camera = Camera::new(
        image_width,
        image_height,
        80.0,
        vec3(0.0, 0.0, 9.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        0.0,
        7.0,
        100,
        50,
        vec3(0.7, 0.8, 1.0),
    );

    let mut imgbuf = image::RgbImage::new(image_width as u32, image_height as u32);

    camera.render_threaded(&world, &mut imgbuf);

    imgbuf.save("output.png")?;
    Ok(())
}

fn tricube() -> anyhow::Result<()> {
    let mut world = HittableList::new();

    let box_material = Arc::new(Material::new_lambertian(
        Arc::new(SpatialChecker::new(
            0.5,
            Arc::new(SolidColor::from_rgb(0.2, 0.2, 0.2)),
            Arc::new(SolidColor::from_rgb(0.8, 0.2, 0.2)),
        )),
        Vec3::ONE,
    ));

    let floor_material = Arc::new(Material::new_metal(
        Arc::new(SolidColor::from_rgb(0.8, 0.8, 0.8)),
        0.1,
    ));

    // back
    world.add(Arc::new(Triangle::new(
        vec3(1.0, -1.0, -1.0),
        vec3(-2.0, 0.0, 0.0),
        vec3(0.0, 2.0, 0.0),
        [Vec2::ZERO; 3],
        box_material.clone(),
    )));
    world.add(Arc::new(Triangle::new(
        vec3(-1.0, 1.0, -1.0),
        vec3(2.0, 0.0, 0.0),
        vec3(0.0, -2.0, 0.0),
        [Vec2::ZERO; 3],
        box_material.clone(),
    )));
    // front
    world.add(Arc::new(Triangle::new(
        vec3(-1.0, -1.0, 1.0),
        vec3(2.0, 0.0, 0.0),
        vec3(0.0, 2.0, 0.0),
        [Vec2::ZERO; 3],
        box_material.clone(),
    )));
    world.add(Arc::new(Triangle::new(
        vec3(1.0, 1.0, 1.0),
        vec3(-2.0, 0.0, 0.0),
        vec3(0.0, -2.0, 0.0),
        [Vec2::ZERO; 3],
        box_material.clone(),
    )));
    // left
    world.add(Arc::new(Triangle::new(
        vec3(-1.0, -1.0, -1.0),
        vec3(0.0, 0.0, 2.0),
        vec3(0.0, 2.0, 0.0),
        [Vec2::ZERO; 3],
        box_material.clone(),
    )));
    world.add(Arc::new(Triangle::new(
        vec3(-1.0, 1.0, 1.0),
        vec3(0.0, 0.0, -2.0),
        vec3(0.0, -2.0, 0.0),
        [Vec2::ZERO; 3],
        box_material.clone(),
    )));
    // right
    world.add(Arc::new(Triangle::new(
        vec3(1.0, -1.0, 1.0),
        vec3(0.0, 0.0, -2.0),
        vec3(0.0, 2.0, 0.0),
        [Vec2::ZERO; 3],
        box_material.clone(),
    )));
    world.add(Arc::new(Triangle::new(
        vec3(1.0, 1.0, -1.0),
        vec3(0.0, 0.0, 2.0),
        vec3(0.0, -2.0, 0.0),
        [Vec2::ZERO; 3],
        box_material.clone(),
    )));
    // bottom
    world.add(Arc::new(Triangle::new(
        vec3(-1.0, -1.0, -1.0),
        vec3(2.0, 0.0, 0.0),
        vec3(0.0, 0.0, 2.0),
        [Vec2::ZERO; 3],
        box_material.clone(),
    )));
    world.add(Arc::new(Triangle::new(
        vec3(1.0, -1.0, 1.0),
        vec3(-2.0, 0.0, 0.0),
        vec3(0.0, 0.0, -2.0),
        [Vec2::ZERO; 3],
        box_material.clone(),
    )));
    // top
    world.add(Arc::new(Triangle::new(
        vec3(-1.0, 1.0, 1.0),
        vec3(2.0, 0.0, 0.0),
        vec3(0.0, 0.0, -2.0),
        [Vec2::ZERO; 3],
        box_material.clone(),
    )));
    world.add(Arc::new(Triangle::new(
        vec3(1.0, 1.0, -1.0),
        vec3(-2.0, 0.0, 0.0),
        vec3(0.0, 0.0, 2.0),
        [Vec2::ZERO; 3],
        box_material.clone(),
    )));

    // floor
    world.add(Arc::new(Quad::new(
        vec3(-10.0, -1.0, 10.0),
        vec3(20.0, 0.0, 0.0),
        vec3(0.0, 0.0, -20.0),
        [Vec2::ZERO; 4],
        floor_material,
    )));

    // let mut rng = rand::rng();

    let bvh = BvhNode::from_hittable_list(world);

    let image_width = 800;
    let image_height = 600;

    let camera = Camera::new(
        image_width,
        image_height,
        60.0,
        vec3(5.0, 4.0, 7.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        0.6,
        9.487,
        500,
        50,
        vec3(0.7, 0.8, 1.0),
    );

    let mut imgbuf = image::RgbImage::new(image_width as u32, image_height as u32);

    // rayon::ThreadPoolBuilder::new().num_threads(10).build_global()?;

    camera.render_threaded(&bvh, &mut imgbuf);

    imgbuf.save("output.png")?;
    Ok(())
}

fn monkey() -> anyhow::Result<()> {
    let mut world = HittableList::new();

    let monkey_meshes = load_obj_meshes("resources/monkeybigearth.obj")?;

    for monkey_mesh in monkey_meshes {
        world.add(Arc::new(BvhNode::from_hittable_list(monkey_mesh)));
    }

    let material = Arc::new(Material::new_lambertian(
        Arc::new(SolidColor::new(Vec3::splat(0.8))),
        Vec3::ONE,
    ));

    // floor
    world.add(Arc::new(Quad::new(
        vec3(-5.0, -1.0, 5.0),
        vec3(10.0, 0.0, 0.0),
        vec3(0.0, 0.0, -10.0),
        [Vec2::ZERO; 4],
        material,
    )));

    // light
    let light_material = Arc::new(Material::new_diffuse_light(
        Arc::new(SolidColor::from_rgb(1.0, 1.0, 1.0)),
        4.0,
    ));
    world.add(Arc::new(Quad::new(
        vec3(2.0, 1.0, -2.0),
        vec3(2.0, 0.0, 2.0),
        vec3(0.0, 2.0, 0.0),
        [Vec2::ZERO; 4],
        light_material,
    )));

    let bvh = BvhNode::from_hittable_list(world);

    let image_width = 800;
    let image_height = 600;

    let camera = Camera::new(
        image_width,
        image_height,
        60.0,
        vec3(4.0, 2.0, 5.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        0.0,
        1.0,
        100,
        10,
        vec3(0.0, 0.0, 0.0),
    );

    let mut imgbuf = image::RgbImage::new(image_width as u32, image_height as u32);

    // rayon::ThreadPoolBuilder::new().num_threads(10).build_global()?;

    camera.render_threaded(&bvh, &mut imgbuf);

    imgbuf.save("output.png")?;
    Ok(())
}

fn cornell_monkey() -> anyhow::Result<()> {
    let mut world = HittableList::new();

    let red_material = Arc::new(Material::new_lambertian(
        Arc::new(SolidColor::from_rgb(0.65, 0.05, 0.05)),
        Vec3::ONE,
    ));
    let white_material = Arc::new(Material::new_lambertian(
        Arc::new(SolidColor::from_rgb(0.73, 0.73, 0.73)),
        Vec3::ONE,
    ));
    let green_material = Arc::new(Material::new_lambertian(
        Arc::new(SolidColor::from_rgb(0.12, 0.45, 0.15)),
        Vec3::ONE,
    ));
    let light_material = Arc::new(Material::new_diffuse_light(
        Arc::new(SolidColor::from_rgb(1.0, 1.0, 1.0)),
        15.0,
    ));

    // left wall
    world.add(Arc::new(Quad::new(
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 0.0, -555.0),
        vec3(0.0, 555.0, 0.0),
        [Vec2::ZERO; 4],
        red_material,
    )));
    // right wall
    world.add(Arc::new(Quad::new(
        vec3(555.0, 0.0, -555.0),
        vec3(0.0, 0.0, 555.0),
        vec3(0.0, 555.0, 0.0),
        [Vec2::ZERO; 4],
        green_material,
    )));
    // floor
    world.add(Arc::new(Quad::new(
        vec3(0.0, 0.0, 0.0),
        vec3(555.0, 0.0, 0.0),
        vec3(0.0, 0.0, -555.0),
        [Vec2::ZERO; 4],
        white_material.clone(),
    )));
    // back wall
    world.add(Arc::new(Quad::new(
        vec3(0.0, 0.0, -555.0),
        vec3(555.0, 0.0, 0.0),
        vec3(0.0, 555.0, 0.0),
        [Vec2::ZERO; 4],
        white_material.clone(),
    )));
    // ceiling
    world.add(Arc::new(Quad::new(
        vec3(0.0, 555.0, -555.0),
        vec3(555.0, 0.0, 0.0),
        vec3(0.0, 0.0, 555.0),
        [Vec2::ZERO; 4],
        white_material.clone(),
    )));
    // light
    world.add(Arc::new(Quad::new(
        vec3(212.0, 554.999, -343.0),
        vec3(131.0, 0.0, 0.0),
        vec3(0.0, 0.0, 131.0),
        [Vec2::ZERO; 4],
        light_material,
    )));

    let monkey_meshes = load_obj_meshes("resources/monkeybigearth.obj")?;

    for monkey_mesh in monkey_meshes {
        world.add(Arc::new(BvhNode::from_hittable_list(monkey_mesh)));
    }

    let bvh = BvhNode::from_hittable_list(world);

    let image_width = 800;
    let image_height = 800;

    let camera = Camera::new(
        image_width,
        image_height,
        40.0,
        vec3(277.5, 277.5, 800.0),
        vec3(277.5, 277.5, 0.0),
        vec3(0.0, 1.0, 0.0),
        0.0,
        1.0,
        100,
        10,
        vec3(0.0, 0.0, 0.0),
    );

    let mut imgbuf = image::RgbImage::new(image_width as u32, image_height as u32);

    // rayon::ThreadPoolBuilder::new().num_threads(10).build_global()?;

    camera.render_threaded(&bvh, &mut imgbuf);

    imgbuf.save("output.png")?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    match 7 {
        1 => spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => quads(),
        5 => tricube(),
        6 => monkey(),
        7 => cornell_monkey(),
        _ => spheres(),
    }
}

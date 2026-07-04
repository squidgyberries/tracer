mod aabb;
mod bvh;
mod camera;
mod color;
mod constant_medium;
mod hit;
mod hittable_list;
mod interval;
mod material;
mod mesh;
mod onb;
mod pdf;
mod quad;
mod ray;
mod sphere;
mod texture;
mod transform;
mod triangle;
mod util;

use std::sync::Arc;

use crate::{
    bvh::BvhNode,
    camera::Camera,
    hittable_list::HittableList,
    material::{DielectricMaterial, DiffuseLightMaterial, LambertianMaterial, MetalMaterial},
    mesh::load_obj_meshes,
    quad::Quad,
    sphere::Sphere,
    texture::SolidColor,
    transform::Transform,
};

use anyhow::Context;
use glam::{Mat4, Quat, Vec2, Vec3, vec3};

fn main() -> anyhow::Result<()> {
    let mut world = HittableList::new();

    let red_material = Arc::new(LambertianMaterial::new(
        Arc::new(SolidColor::from_rgb(0.65, 0.05, 0.05)),
        Vec3::ONE,
    ));
    let white_material = Arc::new(LambertianMaterial::new(
        Arc::new(SolidColor::from_rgb(0.73, 0.73, 0.73)),
        Vec3::ONE,
    ));
    let green_material = Arc::new(LambertianMaterial::new(
        Arc::new(SolidColor::from_rgb(0.12, 0.45, 0.15)),
        Vec3::ONE,
    ));
    let light_material = Arc::new(DiffuseLightMaterial::new(
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
    let light = Arc::new(Quad::new(
        vec3(212.0, 554.999, -343.0),
        vec3(131.0, 0.0, 0.0),
        vec3(0.0, 0.0, 131.0),
        [Vec2::ZERO; 4],
        light_material,
    ));
    world.add(light.clone());

    let cube_meshes = load_obj_meshes("resources/cube.obj", white_material.clone())?;
    let cube_mesh = cube_meshes
        .into_iter()
        .next()
        .context("Missing cube mesh from obj file.")?;

    let cube_mesh_bvh = Arc::new(BvhNode::from_hittable_list(cube_mesh, -1));
    world.add(Arc::new(Transform::new(
        cube_mesh_bvh.clone(),
        &Mat4::from_scale_rotation_translation(
            vec3(165.0, 330.0, 165.0),
            Quat::from_euler(glam::EulerRot::XYZ, 0.0, 15.0f32.to_radians(), 0.0),
            vec3(207.5, 165.0, -377.5),
        ),
    )));

    let glass_material = Arc::new(DielectricMaterial::new(1.5));
    let glass_ball = Arc::new(Sphere::new(
        vec3(342.5, 82.5, -147.5),
        90.0,
        glass_material,
    ));
    world.add(glass_ball.clone());

    let bvh = BvhNode::from_hittable_list(world, -1);

    let empty_material = (*crate::material::DEFAULT_MATERIAL).clone();
    let mut lights = HittableList::new();
    lights.add(Arc::new(Quad::new(
        vec3(212.0, 554.999, -343.0),
        vec3(131.0, 0.0, 0.0),
        vec3(0.0, 0.0, 131.0),
        [Vec2::ZERO; 4],
        empty_material.clone(),
    )));
    lights.add(Arc::new(Sphere::new(
        vec3(342.5, 82.5, -147.5),
        90.0,
        empty_material,
    )));

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
        10000,
        100,
        vec3(0.0, 0.0, 0.0),
    );

    let mut imgbuf = image::RgbImage::new(image_width as u32, image_height as u32);

    // rayon::ThreadPoolBuilder::new().num_threads(10).build_global()?;

    camera.render_threaded(&bvh, Arc::new(lights), &mut imgbuf);

    imgbuf.save("output.png")?;

    Ok(())
}

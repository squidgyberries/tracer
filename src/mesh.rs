use std::{fmt::Debug, path::Path, sync::Arc};

use crate::{
    hittable_list::HittableList,
    material::Material,
    texture::{ImageTexture, SolidColor},
    triangle::Triangle,
};

use glam::{Vec2, Vec3};

pub fn load_obj_meshes(
    path: impl AsRef<Path> + Debug,
    default_material: Arc<Material>,
) -> anyhow::Result<Vec<HittableList>> {
    let (models, materials) = tobj::load_obj(
        &path,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
    )?;

    let materials = materials?;

    let mut out_meshes = Vec::with_capacity(models.len());

    let parent_path = path.as_ref().parent().expect("This should have a parent.");

    for (index, model) in models.iter().enumerate() {
        let mesh = &model.mesh;

        eprintln!(
            "Loading model \"{}\" with {} vertices and {} indices...",
            model.name,
            mesh.positions.len(),
            mesh.indices.len()
        );

        let mut material = default_material.clone();

        if let Some(material_id) = mesh.material_id {
            let mtl_material = &materials[material_id];
            if let Some(diffuse) = mtl_material.diffuse {
                material = Arc::new(Material::new_lambertian(
                    Arc::new(SolidColor::new(Vec3::from(diffuse))),
                    Vec3::ONE,
                ));
            }
            if let Some(diffuse_texture) = &mtl_material.diffuse_texture {
                material = Arc::new(Material::new_lambertian(
                    Arc::new(ImageTexture::load(parent_path.join(diffuse_texture))?),
                    Vec3::ONE,
                ));
            }
        }

        let triangles = mesh.indices.len() / 3;
        out_meshes.push(HittableList::with_capacity(triangles));

        for i in 0..triangles {
            let indices = [
                mesh.indices[3 * i] as usize,
                mesh.indices[3 * i + 1] as usize,
                mesh.indices[3 * i + 2] as usize,
            ];
            let mut vertices = [Vec3::ZERO; 3];
            let mut texcoords = [Vec2::ZERO; 3];
            for (index, (vertex, texcoord)) in indices
                .iter()
                .zip(vertices.iter_mut().zip(texcoords.iter_mut()))
            {
                *vertex = Vec3::new(
                    mesh.positions[3 * index],
                    mesh.positions[3 * index + 1],
                    mesh.positions[3 * index + 2],
                );
                *texcoord = if !mesh.texcoords.is_empty() {
                    Vec2::new(mesh.texcoords[2 * index], mesh.texcoords[2 * index + 1])
                } else {
                    Vec2::ZERO
                }
            }
            out_meshes[index].objects.push(Arc::new(Triangle::new(
                vertices[0],
                vertices[1] - vertices[0],
                vertices[2] - vertices[0],
                texcoords,
                material.clone(),
            )));
        }
    }

    Ok(out_meshes)
}

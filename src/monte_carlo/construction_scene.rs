use super::{
    construction_scene_bvh_node::SceneBvhConstructionNode, hittable_primitive::HittablePrimitive,
    linear_constant_medium::LinearConstantMedium, linear_hittable::LinearHittable,
    linear_scene_bvh::LinearSceneBvh, material::Material, sphere::Sphere, triangle::Triangle,
};

pub fn recompute_bvh(scene: &mut LinearSceneBvh) {
    // Clear existing scene bvh nodes (and hittables reference)
    scene.bvh_nodes.clear();
    scene.hittables = scene
        .hittables
        .iter()
        .cloned()
        .filter(|h| h.geometry_type != 0)
        .collect();

    // Source objects are cloned at the array slice is used within the following recursive bvh construction function
    let source_objects = scene.hittables.clone();

    // Build a referenced structure bvh tree from the scene
    let bvh_construction = SceneBvhConstructionNode::new(scene, &source_objects[..]);

    // Flatten the bvh tree into a linearized structure and update the scene accordingly
    bvh_construction.flatten(scene);
}

/// Primary scene construction function
pub fn build_from_hittable_primitives(
    materials: &[Material],
    primitives: &[HittablePrimitive],
) -> LinearSceneBvh {
    // First create a new scene which will be assembled in the follow steps
    let mut scene = LinearSceneBvh::new();

    // Materials are directly added
    scene.materials = materials.to_vec();

    // Convert the hittable primitives to individual vectors of each primitives type
    // And update the vector of the linear hittable primitives (that now have an indexed reference
    // to the appropriate vector index for the appropriate primitive type
    for primitive in primitives {
        match primitive {
            HittablePrimitive::Sphere(sphere) => {
                scene.spheres.push(*sphere);
                scene.hittables.push(LinearHittable {
                    geometry_type: 1,
                    scene_index: (scene.spheres.len() - 1) as u32,
                });
            }
            HittablePrimitive::Cuboid(cuboid) => {
                scene.cuboids.push(*cuboid);
                scene.hittables.push(LinearHittable {
                    geometry_type: 2,
                    scene_index: (scene.cuboids.len() - 1) as u32,
                });
            }
            HittablePrimitive::ConstantMedium(constant_medium) => {
                let boundary_geometry_type: u32;
                let boundary_scene_index: u32;

                match *constant_medium.boundary_hittable {
                    HittablePrimitive::Sphere(sphere) => {
                        scene.spheres.push(sphere);
                        boundary_geometry_type = 1;
                        boundary_scene_index = (scene.spheres.len() - 1) as u32;
                    }
                    HittablePrimitive::Cuboid(cuboid) => {
                        scene.cuboids.push(cuboid);
                        boundary_geometry_type = 2;
                        boundary_scene_index = (scene.cuboids.len() - 1) as u32;
                    }
                    _ => {
                        panic!("Can't build, unsupported constant medium boundary primitive type");
                    }
                }

                let linear_constant_medium = LinearConstantMedium::new(
                    boundary_geometry_type,
                    boundary_scene_index,
                    constant_medium.material_index,
                    constant_medium.density,
                );

                scene.constant_mediums.push(linear_constant_medium);
                scene.hittables.push(LinearHittable {
                    geometry_type: 3,
                    scene_index: (scene.constant_mediums.len() - 1) as u32,
                });
            }
            HittablePrimitive::Mesh(mesh) => {
                println!("Building construction scene from mesh");
                // This is the offset to apply to the mesh_tris indices
                let offset = scene.tri_verts.len() as u32;

                let (mut mesh_tri_verts, mesh_tris) = mesh.get_default_first_mesh();

                // Append the mesh tri verts the scene tri verts.
                scene.tri_verts.append(&mut mesh_tri_verts);

                // Append the mesh tri ints the scene tri indices
                for tri in mesh_tris {
                    scene.tris.push(Triangle::new(
                        mesh.material_index,
                        [
                            tri.indices[0] + offset,
                            tri.indices[1] + offset,
                            tri.indices[2] + offset,
                        ],
                    ));
                    scene.hittables.push(LinearHittable {
                        geometry_type: 4,
                        scene_index: (scene.tris.len() - 1) as u32,
                    });
                }
            }
        }
    }

    recompute_bvh(&mut scene);

    // Finally, validate the scene and ensure that it has no empty arrays (otherwise throws error in the wgpu binding)
    scene.check_pad_empty_arrays();

    // Debug - pretty print the flattened scene bvh
    // scene.debug_print();

    // Return the constructed scene
    scene
}

/// Build a LinearSceneBvh from just materials and spheres
pub fn build_from_spheres(materials: &[Material], spheres: &[Sphere]) -> LinearSceneBvh {
    let hittables: Vec<HittablePrimitive> = spheres
        .iter()
        .map(|sphere| HittablePrimitive::Sphere(*sphere))
        .collect();

    self::build_from_hittable_primitives(materials, &hittables[..])
}

// pub fn build_from_meshes(materials: &[Material], meshes: &[Mesh]) -> LinearSceneBvh {
//     let hittables: Vec<HittablePrimitive> = meshes
//         .iter()
//         .map(|mesh| HittablePrimitive::Mesh(mesh.clone()))
//         .collect();
//     self::build_from_hittable_primitives(materials, &hittables[..])
// }

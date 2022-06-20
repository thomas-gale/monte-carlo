use super::{
    constant_medium,
    construction_scene_bvh_node::SceneBvhConstructionNode,
    hittable_primitive::HittablePrimitive,
    linear_constant_medium::{self, LinearConstantMedium},
    linear_hittable::LinearHittable,
    linear_scene_bvh::LinearSceneBvh,
    material::Material,
    sphere::Sphere,
};

/// Primary scene construction function
pub fn build_from_hittable_primitives(
    background: Material,
    materials: &[Material],
    primitives: &[HittablePrimitive],
) -> LinearSceneBvh {
    // First create a new scene which will be assembled in the follow steps
    let mut scene = LinearSceneBvh::new();

    // Background material directly added to the scene
    scene.background = background;

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
                let mut boundary_geometry_type = LinearSceneBvh::null_index_ptr();
                let mut boundary_scene_index = LinearSceneBvh::null_index_ptr();

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
            // Other hittable primitives will be added in due course.
            _ => {
                panic!("Can't build, unsupported hittable primitive type");
            }
        }
    }

    // Source objects are cloned at the array slice is used within the following recursive bvh construction function
    let source_objects = scene.hittables.clone();

    // Build a referenced structure bvh tree from the scene
    let bvh_construction = SceneBvhConstructionNode::new(&mut scene, &source_objects[..]);

    // Flatten the bvh tree into a linearized structure and update the scene accordingly
    bvh_construction.flatten(&mut scene);

    // Finally, validate the scene and ensure that it has no empty arrays (otherwise throws error in the wgpu binding)
    scene.check_pad_empty_arrays();

    // Debug - pretty print the flattened scene bvh
    // scene.debug_print();

    // Return the constructed scene
    scene
}

/// Build a LinearSceneBvh from just materials and spheres
pub fn build_from_spheres(
    background: Material,
    materials: &[Material],
    spheres: &[Sphere],
) -> LinearSceneBvh {
    let hittables: Vec<HittablePrimitive> = spheres
        .iter()
        .map(|sphere| HittablePrimitive::Sphere(*sphere))
        .collect();

    self::build_from_hittable_primitives(background, materials, &hittables[..])
}

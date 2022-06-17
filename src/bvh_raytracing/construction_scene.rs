use super::{
    construction_scene_bvh_node::SceneBvhConstructionNode, hittable_primitive::HittablePrimitive,
    linear_hittable::LinearHittable, linear_scene_bvh::LinearSceneBvh, material::Material,
    sphere::Sphere,
};

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
pub fn build_from_spheres(materials: &[Material], spheres: &[Sphere]) -> LinearSceneBvh {
    let hittables: Vec<HittablePrimitive> = spheres
        .iter()
        .map(|sphere| HittablePrimitive::Sphere(*sphere))
        .collect();

    self::build_from_hittable_primitives(materials, &hittables[..])
}

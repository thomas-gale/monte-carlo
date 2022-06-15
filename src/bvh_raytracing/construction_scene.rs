use super::{
    bvh_node::BvhNode, construction_scene_bvh_node::SceneBvhConstructionNode, cuboid::Cuboid,
    hittable_primitive::HittablePrimitive, linear_hittable::LinearHittable,
    linear_scene_bvh::LinearSceneBvh, material::Material, sphere::Sphere,
};

// pub struct ConstructionScene {
//     root_bvh_node: Vec<SceneBvhConstructionNode>,
//     materials: Vec<Material>,
// }

// impl ConstructionScene {
pub fn build_from_hittable_primitives(
    materials: &[Material],
    primitives: &[HittablePrimitive],
) -> LinearSceneBvh {
    // Rough and ready test code
    let mut scene = LinearSceneBvh::new();
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
                    scene_index: (scene.spheres.len() - 1) as u32,
                });
            }
            _ => {
                panic!("Can't build, unsupported hittable primitive type");
            }
        }
    }

    let source_objects = scene.hittables.clone();

    let bvh_construction = SceneBvhConstructionNode::new(&mut scene, &source_objects[..]);

    scene.debug_print();

    bvh_construction.flatten(&mut scene);

    // Debug - pretty print the flattened scene bvh
    scene.debug_print();

    scene
}

/// TODO - this is being replaced with direct construction from the "Construction Scene" methods
/// Experimental function to build a BVH from a slice of spheres
pub fn build_from_spheres(materials: &[Material], spheres: &[Sphere]) -> LinearSceneBvh {
    let hittables: Vec<HittablePrimitive> = spheres
        .iter()
        .map(|sphere| HittablePrimitive::Sphere(*sphere))
        .collect();

    self::build_from_hittable_primitives(materials, &hittables[..])
}

// TODO - this is being replaced with direct construction from the "Construction Scene" methods
// pub fn build_from_hittables(hittables: Vec<LinearHittable>) -> LinearSceneBvh {
//     LinearSceneBvh {
//         hittables,
//         bvh_nodes: vec![BvhNode::empty()],
//         spheres: vec![Sphere::empty()],
//         cuboids: vec![Cuboid::empty()],
//         materials: vec![Material::empty()],
//     }
// }
// }

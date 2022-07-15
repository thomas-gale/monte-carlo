use std::{fs::File, io::BufReader};

use cgmath::{prelude::*, Deg, Matrix4, Vector3};
use obj::Obj;

use super::{
    constant_medium::ConstantMedium, construction_scene, cuboid::Cuboid,
    hittable_primitive::HittablePrimitive, linear_scene_bvh::LinearSceneBvh, material::Material,
    mesh::Mesh, sphere::Sphere,
};

#[allow(dead_code)]
pub fn test_mesh_scene() -> LinearSceneBvh {
    // let bunny_raw = BufReader::new(
    //     File::open("src/monte_carlo/resources/bunny.obj").expect("Unable to open bunny file"),
    // );
    // let bunny_obj: Obj = load_obj(bunny_raw).expect("Unable to load bunny obj");
    // let bunny_obj: Obj = Obj::load("src/load_obj(bunny_raw).expect("Unable to load bunny obj");
    println!("Loading bunny");
    let bunny_obj: Obj = Obj::load("src/monte_carlo/resources/bunny.obj").expect("Unable to load bunny obj");
    println!("Loaded bunny");
    let bunny = Mesh::new(bunny_obj);

    construction_scene::build_from_meshes(
        Material::new(0, 0.0, 0.0, Vector3::new(0.70, 0.80, 1.00)),
        &vec![Material::new(
            0,
            0.0,
            0.0,
            Vector3::<f32>::new(1.0, 0.0, 0.0),
        )],
        &vec![bunny],
    )
}

#[allow(dead_code)]
pub fn simple_scene() -> LinearSceneBvh {
    construction_scene::build_from_spheres(
        Material::new(0, 0.0, 0.0, Vector3::new(0.70, 0.80, 1.00)),
        &vec![Material::new(
            0,
            0.0,
            0.0,
            Vector3::<f32>::new(1.0, 0.0, 0.0),
        )],
        &vec![Sphere::new(Vector3::<f32>::new(0.0, 0.0, 0.0), 0.5, 0)],
    )
}

#[allow(dead_code)]
pub fn test_scene_wos() -> LinearSceneBvh {
    construction_scene::build_from_hittable_primitives(
        Material::new(0, 0.0, 0.0, Vector3::new(0.70, 0.80, 1.00)),
        &vec![
            Material::new(5, 0.0, 0.0, Vector3::<f32>::new(0.0, 0.0, 0.0)),
            Material::new(0, 0.0, 0.0, Vector3::<f32>::new(0.7, 0.6, 0.7)),
            Material::new(0, 0.0, 0.0, Vector3::<f32>::new(0.8, 0.0, 0.0)),
            Material::new(0, 0.0, 0.0, Vector3::<f32>::new(0.0, 0.8, 0.0)),
            Material::new(0, 0.0, 0.0, Vector3::<f32>::new(0.0, 0.0, 0.8)),
            Material::new(2, 0.0, 1.5, Vector3::<f32>::new(1.0, 1.0, 1.0)),
        ],
        &vec![
            HittablePrimitive::Cuboid(Cuboid::new(
                Matrix4::identity()
                    * Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0))
                    * Matrix4::from_nonuniform_scale(5.0, 5.0, 0.1),
                0,
            )),
            HittablePrimitive::Cuboid(Cuboid::new(
                Matrix4::identity()
                    * Matrix4::from_translation(Vector3::new(0.0, -1.0, 0.0))
                    * Matrix4::from_nonuniform_scale(100.0, 1.0, 100.0),
                1,
            )),
            HittablePrimitive::Sphere(Sphere::new(Vector3::<f32>::new(2.0, 0.75, 0.4), 0.75, 2)),
            HittablePrimitive::Sphere(Sphere::new(Vector3::<f32>::new(-2.0, 1.0, -0.4), 1.0, 3)),
            HittablePrimitive::Cuboid(Cuboid::new(
                Matrix4::identity()
                    * Matrix4::from_translation(Vector3::new(0.0, 1.2, 0.2))
                    * Matrix4::from_angle_x(Deg(20.0))
                    * Matrix4::from_angle_y(Deg(30.0))
                    * Matrix4::from_nonuniform_scale(0.5, 1.0, 0.5),
                4,
            )),
            HittablePrimitive::Sphere(Sphere::new(Vector3::<f32>::new(0.0, 1.25, 5.0), 1.25, 5)),
            HittablePrimitive::Sphere(Sphere::new(Vector3::<f32>::new(0.0, 1.25, -5.0), 1.25, 5)),
        ],
    )
}

#[allow(dead_code)]
pub fn cornell_box() -> LinearSceneBvh {
    construction_scene::build_from_hittable_primitives(
        Material::new(0, 0.0, 0.0, Vector3::new(0.0, 0.0, 0.0)),
        &vec![
            Material::new(0, 0.0, 0.0, Vector3::<f32>::new(1.0, 1.0, 1.0)),
            Material::new(0, 0.0, 0.0, Vector3::<f32>::new(1.0, 0.0, 0.0)),
            Material::new(0, 0.0, 0.0, Vector3::<f32>::new(0.0, 1.0, 0.0)),
            Material::new(3, 0.0, 0.0, Vector3::<f32>::new(2.0, 2.0, 2.0)),
            Material::new(2, 0.0, 1.5, Vector3::<f32>::new(1.0, 1.0, 1.0)),
            Material::new(0, 0.0, 0.0, Vector3::<f32>::new(0.9, 0.9, 0.9)),
            Material::new(4, 0.0, 0.0, Vector3::<f32>::new(1.0, 1.0, 0.0)),
            Material::new(0, 0.0, 0.0, Vector3::<f32>::new(0.0, 0.0, 1.0)),
            Material::new(0, 0.0, 0.0, Vector3::<f32>::new(1.0, 1.0, 0.0)),
            Material::new(5, 0.0, 0.0, Vector3::<f32>::new(1.0, 1.0, 1.0)),
        ],
        &vec![
            HittablePrimitive::Cuboid(Cuboid::new(
                Matrix4::identity()
                    * Matrix4::from_translation(Vector3::new(0.0, 0.2, 0.0))
                    * Matrix4::from_angle_x(Deg(-60.0))
                    * Matrix4::from_angle_z(Deg(45.0))
                    * Matrix4::from_nonuniform_scale(0.51, 0.51, -0.01),
                9,
            )),
            HittablePrimitive::Cuboid(Cuboid::new(
                Matrix4::identity()
                    * Matrix4::from_translation(Vector3::new(0.0, -0.01, 0.0))
                    * Matrix4::from_nonuniform_scale(0.5, 0.01, 0.5),
                0,
            )),
            HittablePrimitive::Cuboid(Cuboid::new(
                Matrix4::identity()
                    * Matrix4::from_translation(Vector3::new(0.0, 1.01, 0.0))
                    * Matrix4::from_nonuniform_scale(0.5, 0.01, 0.5),
                0,
            )),
            HittablePrimitive::Cuboid(Cuboid::new(
                Matrix4::identity()
                    * Matrix4::from_translation(Vector3::new(0.0, 0.5, -0.51))
                    * Matrix4::from_nonuniform_scale(0.5, 0.5, 0.01),
                0,
            )),
            HittablePrimitive::Cuboid(Cuboid::new(
                Matrix4::identity()
                    * Matrix4::from_translation(Vector3::new(-0.51, 0.5, 0.0))
                    * Matrix4::from_nonuniform_scale(0.01, 0.5, 0.5),
                1,
            )),
            HittablePrimitive::Cuboid(Cuboid::new(
                Matrix4::identity()
                    * Matrix4::from_translation(Vector3::new(0.51, 0.5, 0.0))
                    * Matrix4::from_nonuniform_scale(0.01, 0.5, 0.5),
                2,
            )),
            HittablePrimitive::Cuboid(Cuboid::new(
                Matrix4::identity()
                    * Matrix4::from_translation(Vector3::new(0.0, 0.9999, 0.0))
                    * Matrix4::from_nonuniform_scale(0.25, 0.0001, 0.25),
                3,
            )),
            HittablePrimitive::Cuboid(Cuboid::new(
                Matrix4::identity()
                    * Matrix4::from_translation(Vector3::new(-0.25, 0.3, -0.25))
                    * Matrix4::from_angle_y(Deg(20.0))
                    * Matrix4::from_nonuniform_scale(0.125, 0.3, 0.125),
                7,
            )),
            HittablePrimitive::Cuboid(Cuboid::new(
                Matrix4::identity()
                    * Matrix4::from_translation(Vector3::new(0.125, 0.125, 0.25))
                    * Matrix4::from_angle_y(Deg(-20.0))
                    * Matrix4::from_nonuniform_scale(0.125, 0.125, 0.125),
                8,
            )),
            HittablePrimitive::Sphere(Sphere::new(
                Vector3::<f32>::new(-0.125, 0.125, 0.125),
                0.125,
                4,
            )),
            HittablePrimitive::ConstantMedium(ConstantMedium {
                boundary_hittable: Box::new(HittablePrimitive::Sphere(Sphere::new(
                    Vector3::<f32>::new(0.24, 0.25, -0.24),
                    0.25,
                    5,
                ))),
                material_index: 6,
                density: 5.0,
            }),
        ],
    )
}

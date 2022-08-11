use super::triangle::{Triangle, TriangleVertex};
use obj::Obj;

#[derive(Clone, Debug)]
pub struct Mesh {
    obj: Obj,
    pub material_index: u32,
}

impl Mesh {
    pub fn new(obj: Obj, material_index: u32) -> Self {
        println!("Len Positions: {:?}", obj.data.position.len());
        println!("Position 0: {:?}", obj.data.position[0]);
        println!(
            "Len Objects 0 Group 0 polys: {:?}",
            obj.data.objects[0].groups[0].polys.len()
        );
        println!(
            "Len Objects 0 Group 0 poly 0: {:?}",
            obj.data.objects[0].groups[0].polys[0]
        );
        Mesh {
            obj,
            material_index,
        }
    }

    /// Rather hacky code pulling the first mesh/object group out from the obj file.
    pub fn get_default_first_mesh(&self) -> (Vec<TriangleVertex>, Vec<Triangle>) {
        let verts: Vec<TriangleVertex> = self
            .obj
            .data
            .position
            .iter()
            .map(|pos| TriangleVertex::new(pos.clone()))
            .collect();

        let tris: Vec<Triangle> = self.obj.data.objects[0].groups[0]
            .polys
            .iter()
            .map(|poly| {
                Triangle::new(
                    self.material_index,
                    [poly.0[0].0 as u32, poly.0[1].0 as u32, poly.0[2].0 as u32],
                )
            })
            .collect();

        (verts, tris)
    }
}

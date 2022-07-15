use obj::Obj;

#[derive(Clone)]
pub struct Mesh {
    obj: Obj,
}

impl Mesh {
    pub fn new(obj: Obj) -> Self {
        Mesh { obj }
    }
}

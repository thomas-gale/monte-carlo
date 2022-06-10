use super::aabb::Aabb;

pub trait Hittable {
    fn bounding_box(&self) -> Aabb;
}

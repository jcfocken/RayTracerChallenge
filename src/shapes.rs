use crate::tuple;
/// An enum of all the shapes that can be intersected by a ray.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Shape {
    Sphere(Sphere),
}
/// A sphere.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Sphere {
    pub center: tuple::Tuple,
    pub radius: f32,
}
impl Sphere {
    /// Create a new sphere with the given center and radius.
    pub fn new(center: tuple::Tuple, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}
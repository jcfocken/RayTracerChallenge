use crate::matrix;
/// An enum of all the shapes that can be intersected by a ray.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Shape {
    Sphere(Sphere),
}
/// A sphere.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Sphere {
    pub transform: matrix::Matrix4x4,
}
impl Sphere {
    /// Create a new sphere
    pub fn new() -> Sphere {
        Sphere {transform : matrix::identity() }
    }
    pub fn set_transform(&mut self, t: matrix::Matrix4x4) {
        self.transform = t;
    }
}
impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}
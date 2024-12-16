use crate::{matrix, colour, tuple::{self, point}};
/// An enum of all the shapes that can be intersected by a ray.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Shape {
    Sphere(Sphere),
}
/// A sphere.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Sphere {
    pub transform: matrix::Matrix4x4,
    pub material: Material
}
impl Sphere {
    /// Create a new sphere
    pub fn new() -> Sphere {
        Sphere {transform: matrix::identity(), material: Material::new()}
    }
    pub fn set_transform(&mut self, t: matrix::Matrix4x4) {
        self.transform = t;
    }
    pub fn normal_at(self, world_point: tuple::Tuple) -> tuple::Tuple{
        let object_point = self.transform.inverse() * world_point;
        let object_normal = object_point - point(0.0, 0.0, 0.0);
        let mut world_normal = self.transform.inverse().transpose() * object_normal;
        world_normal.w = 0.0;
        world_normal.normalize()
    }
}
impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}
/// The optical properties of a shape
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Material {
    pub colour: colour::Colour,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}
impl Material {
    /// Create a new default material
    pub fn new() -> Material{
        Material {colour: colour::WHITE, ambient: 0.1, diffuse: 0.9, specular: 0.9, shininess: 200.0}
    }
}
#[cfg(test)]
mod tests {
    use crate::shapes::{Material, Sphere};
    
    #[test]
    fn sphere_has_default_material() {
        let s = Sphere::new();
        assert_eq!(s.material, Material::new());
    }
    #[test]
    fn sphere_material_can_be_assigned() {
        let mut s = Sphere::new();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.material = m;
        assert_eq!(s.material, m);
    }
}
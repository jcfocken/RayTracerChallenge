use crate::{matrix, colour, tuple::{self, point}};
/// An enum of all the shapes that can be intersected by a ray.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Shape {
    Sphere(),
    Test(),
    Plane(),
}
/// A sphere.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Object {
    pub transform: matrix::Matrix4x4,
    pub material: Material,
    pub shape: Shape
}
impl Object {
    /// Create a test object
    pub fn new() -> Object {
        Object {transform: matrix::identity(), material: Material::new(), shape: Shape::Test()}
    }
    /// Create a new sphere
    pub fn new_sphere() -> Object {
        Object {transform: matrix::identity(), material: Material::new(), shape: Shape::Sphere()}
    }
    /// Create a new plane
    pub fn new_plane() -> Object {
        Object {transform: matrix::identity(), material: Material::new(), shape: Shape::Plane()}
    }
    /// Set the objects transformation matrix
    pub fn set_transform(&mut self, t: matrix::Matrix4x4) {
        self.transform = t;
    }
    /// Compute the objects normal at a particular world point
    pub fn normal_at(self, world_point: tuple::Tuple) -> tuple::Tuple{
        let object_point = self.transform.inverse() * world_point;
        let object_normal;
        match self.shape {
            Shape::Sphere() => object_normal = object_point - point(0.0, 0.0, 0.0),
            Shape::Test() => object_normal = point(0.0, 0.0, 0.0),
            Shape::Plane() => object_normal = point(0.0, 1.0, 0.0),
        }
        let mut world_normal = self.transform.inverse().transpose() * object_normal;
        world_normal.w = 0.0;
        world_normal.normalize()
    }
}
impl Default for Object {
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
    use crate::{matrix::identity, shapes::{Material, Object}, transformation::translation, tuple::{point, vector}};
    
    #[test]
    fn sphere_has_default_material() {
        let s = Object::new_sphere();
        assert_eq!(s.material, Material::new());
    }
    #[test]
    fn sphere_material_can_be_assigned() {
        let mut s = Object::new_sphere();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.material = m;
        assert_eq!(s.material, m);
    }
    #[test]
    fn sphere_has_default_transformation() {
        let s = Object::new_sphere();
        assert_eq!(s.transform, identity());
    }
    #[test]
    fn sphere_transform_can_be_assigned() {
        let mut s = Object::new_sphere();
        s.transform = translation(2.0, 3.0, 4.0);
        assert_eq!(s.transform, translation(2.0, 3.0, 4.0));
    }
    #[test]
    fn test_shape_has_default_material() {
        let s = Object::new();
        assert_eq!(s.material, Material::new());
    }
    #[test]
    fn test_shape_material_can_be_assigned() {
        let mut s = Object::new();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.material = m;
        assert_eq!(s.material, m);
    }
    #[test]
    fn test_shape_has_default_transformation() {
        let s = Object::new();
        assert_eq!(s.transform, identity());
    }
    #[test]
    fn test_shape_transform_can_be_assigned() {
        let mut s = Object::new();
        s.transform = translation(2.0, 3.0, 4.0);
        assert_eq!(s.transform, translation(2.0, 3.0, 4.0));
    }
    #[test]
    fn normal_of_plane_is_constant() {
        let p = Object::new_plane();
        let n1 = p.normal_at(point(0.0, 0.0, 0.0));
        let n2 = p.normal_at(point(10.0, 0.0, -10.0));
        let n3 = p.normal_at(point(-5.0, 0.0, 150.0));
        assert_eq!(n1, vector(0.0, 1.0, 0.0));
        assert_eq!(n2, vector(0.0, 1.0, 0.0));
        assert_eq!(n3, vector(0.0, 1.0, 0.0));
    }
}
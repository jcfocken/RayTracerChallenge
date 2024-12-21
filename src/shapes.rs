use crate::{
    colour::{self, Colour},
    matrix::{self, identity, Matrix4x4},
    tuple::{self, point, Tuple}, DEFAULT_EPSILON,
};
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
    pub shape: Shape,
}
impl Object {
    /// Create a test object
    pub fn new() -> Object {
        Object {
            transform: matrix::identity(),
            material: Material::new(),
            shape: Shape::Test(),
        }
    }
    /// Create a new sphere
    pub fn new_sphere() -> Object {
        Object {
            transform: matrix::identity(),
            material: Material::new(),
            shape: Shape::Sphere(),
        }
    }
    /// Create a new plane
    pub fn new_plane() -> Object {
        Object {
            transform: matrix::identity(),
            material: Material::new(),
            shape: Shape::Plane(),
        }
    }
    /// Compute the objects normal at a particular world point
    pub fn normal_at(self, world_point: tuple::Tuple) -> tuple::Tuple {
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
    /// Compute the pattern colour at the given point
    pub fn pattern_at(&self, world_point: Tuple) -> Colour {
        if let Some(pattern) = self.material.pattern {
            let object_point = self.transform.inverse() * world_point;
            let pattern_point = pattern.transformation.inverse() * object_point;
            pattern.pattern_at(pattern_point)
        } else {
            self.material.colour
        }
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
    pub pattern: Option<Pattern>,
    pub reflective: f32,
}
impl Material {
    /// Create a new default material
    pub fn new() -> Material {
        Material {
            colour: colour::WHITE,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            pattern: None,
            reflective: 0.0,
        }
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PatternType {
    Striped(),
    Gradient(),
    Ring(),
    Checkers(),
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pattern {
    pub c1: colour::Colour,
    pub c2: colour::Colour,
    pub pattern_type: PatternType,
    pub transformation: Matrix4x4,
}
impl Pattern {
    pub fn new_striped(c1: colour::Colour, c2: colour::Colour) -> Pattern {
        Pattern {
            c1,
            c2,
            pattern_type: PatternType::Striped(),
            transformation: identity(),
        }
    }
    pub fn new_gradient(c1: colour::Colour, c2: colour::Colour) -> Pattern {
        Pattern {
            c1,
            c2,
            pattern_type: PatternType::Gradient(),
            transformation: identity(),
        }
    }
    pub fn new_ring(c1: colour::Colour, c2: colour::Colour) -> Pattern {
        Pattern {
            c1,
            c2,
            pattern_type: PatternType::Ring(),
            transformation: identity(),
        }
    }
    pub fn new_checkers(c1: colour::Colour, c2: colour::Colour) -> Pattern {
        Pattern {
            c1,
            c2,
            pattern_type: PatternType::Checkers(),
            transformation: identity(),
        }
    }
    pub fn pattern_at(&self, point: Tuple) -> Colour {
        match self.pattern_type {
            PatternType::Striped() => {
                if (point.x.floor().rem_euclid(2.0)) > 0.0 {
                    self.c2
                } else {
                    self.c1
                }
            }
            PatternType::Gradient() => {
                let distance = self.c2 - self.c1;
                let fraction = point.x - point.x.floor();
                self.c1 + distance * fraction
            }
            PatternType::Ring() => {
                if (((point.x.powi(2) + point.z.powi(2)).sqrt().floor()).rem_euclid(2.0)) == 0.0 {
                    self.c1
                } else {
                    self.c2
                }
            }
            PatternType::Checkers() => {
                // Move the point slightly positive incase they are actually 0.0 but FP errors have them below 0
                let point_x = (point.x + DEFAULT_EPSILON).floor(); 
                let point_y = (point.y + DEFAULT_EPSILON).floor(); 
                let point_z = (point.z + DEFAULT_EPSILON).floor(); 
                if (point_x + point_y + point_z).rem_euclid(2.0) == 0.0 {
                    self.c1
                } else {
                    self.c2
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use crate::{
        colour::{Colour, BLACK, WHITE},
        matrix::identity,
        shapes::{Material, Object, Pattern},
        transformation::{scale, translation},
        tuple::{point, vector},
        DEFAULT_EPSILON,
    };

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
    #[test]
    fn stripe_pattern_is_constant_y() {
        let p = Pattern::new_striped(WHITE, BLACK);
        assert_eq!(p.pattern_at(point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(p.pattern_at(point(0.0, 1.0, 0.0)), WHITE);
        assert_eq!(p.pattern_at(point(0.0, 2.0, 0.0)), WHITE);
    }
    #[test]
    fn stripe_pattern_is_constant_z() {
        let p = Pattern::new_striped(WHITE, BLACK);
        assert_eq!(p.pattern_at(point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(p.pattern_at(point(0.0, 0.0, 1.0)), WHITE);
        assert_eq!(p.pattern_at(point(0.0, 0.0, 2.0)), WHITE);
    }
    #[test]
    fn stripe_pattern_alternates_x() {
        let p = Pattern::new_striped(WHITE, BLACK);
        assert_eq!(p.pattern_at(point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(p.pattern_at(point(0.9, 0.0, 0.0)), WHITE);
        assert_eq!(p.pattern_at(point(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(p.pattern_at(point(-0.1, 0.0, 0.0)), BLACK);
        assert_eq!(p.pattern_at(point(-1.0, 0.0, 0.0)), BLACK);
        assert_eq!(p.pattern_at(point(-1.1, 0.0, 0.0)), WHITE);
    }
    #[test]
    fn stripe_object_transform() {
        let mut o = Object::new_sphere();
        o.transform = scale(2.0, 2.0, 2.0);
        o.material.pattern = Some(Pattern::new_striped(WHITE, BLACK));
        assert_eq!(o.pattern_at(point(1.5, 0.0, 0.0)), WHITE);
    }
    #[test]
    fn stripe_pattern_transform() {
        let mut o = Object::new_sphere();
        let mut p = Pattern::new_striped(WHITE, BLACK);
        p.transformation = scale(2.0, 2.0, 2.0);
        o.material.pattern = Some(p);
        assert_eq!(o.pattern_at(point(0.5, 0.0, 0.0)), WHITE);
        assert_eq!(o.pattern_at(point(1.5, 0.0, 0.0)), WHITE);
        assert_eq!(o.pattern_at(point(2.5, 0.0, 0.0)), BLACK);
    }
    #[test]
    fn stripe_object_pattern_transform() {
        let mut o = Object::new_sphere();
        o.transform = scale(2.0, 2.0, 2.0);
        let mut p = Pattern::new_striped(WHITE, BLACK);
        p.transformation = translation(0.5, 0.0, 0.0);
        o.material.pattern = Some(p);
        assert_eq!(o.pattern_at(point(2.5, 0.0, 0.0)), WHITE);
    }
    #[test]
    fn default_pattern_transform() {
        let p = Pattern::new_striped(WHITE, BLACK);
        assert_eq!(p.transformation, identity());
    }
    #[test]
    fn gradient_interpolates() {
        let p = Pattern::new_gradient(WHITE, BLACK);
        assert_eq!(p.pattern_at(point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(
            p.pattern_at(point(0.25, 0.0, 0.0)),
            Colour::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            p.pattern_at(point(0.5, 0.0, 0.0)),
            Colour::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            p.pattern_at(point(0.75, 0.0, 0.0)),
            Colour::new(0.25, 0.25, 0.25)
        );
        assert_eq!(p.pattern_at(point(1.0, 0.0, 0.0)), WHITE);
        assert_relative_eq!(
            p.pattern_at(point(0.9999999, 0.0, 0.0)),
            BLACK,
            epsilon = DEFAULT_EPSILON
        );
    }
    #[test]
    fn ring_pattern() {
        let p = Pattern::new_ring(WHITE, BLACK);
        assert_eq!(p.pattern_at(point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(p.pattern_at(point(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(p.pattern_at(point(0.0, 0.0, 1.0)), BLACK);
        assert_eq!(p.pattern_at(point(0.708, 0.0, 0.708)), BLACK);
    }
    #[test]
    fn checkers_repeat_x() {
        let p = Pattern::new_checkers(WHITE, BLACK);
        assert_eq!(p.pattern_at(point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(p.pattern_at(point(0.99, 0.0, 0.0)), WHITE);
        assert_eq!(p.pattern_at(point(1.01, 0.0, 0.0)), BLACK);
    }
    #[test]
    fn checkers_repeat_y() {
        let p = Pattern::new_checkers(WHITE, BLACK);
        assert_eq!(p.pattern_at(point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(p.pattern_at(point(0.0, 0.99, 0.0)), WHITE);
        assert_eq!(p.pattern_at(point(0.0, 1.01, 0.0)), BLACK);
    }
    #[test]
    fn checkers_repeat_z() {
        let p = Pattern::new_checkers(WHITE, BLACK);
        assert_eq!(p.pattern_at(point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(p.pattern_at(point(0.0, 0.0, 0.99)), WHITE);
        assert_eq!(p.pattern_at(point(0.0, 0.0, 1.01)), BLACK);
    }
    #[test]
    fn default_reflectivity() {
        let m = Material::new();
        assert_eq!(m.reflective, 0.0);
    }
}

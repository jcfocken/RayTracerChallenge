use crate::{
    colour::{self, Colour},
    matrix::Matrix4x4,
    shapes::{self, Object, Shape},
    tuple::{self, point, vector, Tuple},
    DEFAULT_EPSILON,
};
use std::cmp::Ordering;

/// A ray.
#[derive(Debug)]
pub struct Ray {
    pub origin: tuple::Tuple,
    pub direction: tuple::Tuple,
}
impl Ray {
    /// Create a new ray with the given origin and direction.
    pub fn new(origin: tuple::Tuple, direction: tuple::Tuple) -> Ray {
        Ray { origin, direction }
    }
    /// Calculate the position of the ray at the given time.
    pub fn position(&self, t: f32) -> tuple::Tuple {
        self.origin + self.direction * t
    }
    /// Calculate the intersections between the ray and the given shape.
    pub fn intersect(&self, object: &shapes::Object) -> Vec<Intersection> {
        let transformed_ray = self.transform(object.transform.inverse());
        match object.shape {
            Shape::Sphere() => {
                let origin_to_center = transformed_ray.origin - point(0.0, 0.0, 0.0);
                let a = transformed_ray.direction.dot(transformed_ray.direction);
                let b = 2.0 * transformed_ray.direction.dot(origin_to_center);
                let c = origin_to_center.dot(origin_to_center) - 1.0;
                let discriminant = b * b - 4.0 * a * c;
                if discriminant < 0.0 {
                    return vec![];
                }
                let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
                let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
                vec![
                    Intersection::new(t1, *object),
                    Intersection::new(t2, *object),
                ]
            }
            Shape::Test() => {
                vec![]
            }
            Shape::Plane() => {
                if self.direction.y.abs() < DEFAULT_EPSILON {
                    vec![]
                } else {
                    let t = (-transformed_ray.origin.y) / transformed_ray.direction.y;
                    vec![Intersection::new(t, *object)]
                }
            }
        }
    }
    /// Transform the ray by a 4x4 matrix.
    pub fn transform(&self, m: Matrix4x4) -> Ray {
        let p = m * self.origin;
        let d = m * self.direction;
        Ray::new(p, d)
    }
    pub fn prepare_computations(self, inter: &Intersection, inters: Intersections) -> Computations {
        let point = self.position(inter.t);
        let eyev = -(self.direction);
        let inside;
        let mut normalv = inter.object.normal_at(point);
        if normalv.dot(eyev) < 0.0 {
            inside = true;
            normalv = vector(0.0, 0.0, 0.0) - normalv;
        } else {
            inside = false;
            normalv = normalv;
        }
        let over_point = point + normalv * (DEFAULT_EPSILON); // TODO can  I reduce this factor and still stop the acne?
        let under_point = point - normalv * (DEFAULT_EPSILON); // TODO can  I reduce this factor and still stop the acne?
        let reflectv = self.direction.reflect(normalv);
        let mut n1 = 1.0;
        let mut n2 = 1.0;
        let mut containers: Vec<Object> = Vec::new();
        for x in inters.inters {
            if x == *inter {
                if let Some(o) = containers.last() {                    
                    n1 = o.material.refractive_index;
                } else {
                    n1 = 1.0;
                }
            }
            if let Some(index) = containers.iter().position(|value| *value == x.object) {
                //remove object from container if it already exists as we're exiting the object
                containers.remove(index);
            } else {
                //add object to container if it doesn't exist as we're entering the object
                containers.push(x.object);
            }
            if x == *inter {
                if let Some(o) = containers.last() {                    
                    n2 = o.material.refractive_index;
                } else {
                    n2 = 1.0;
                }
                break;                
            }
        }
        Computations {
            t: inter.t,
            object: inter.object,
            point,
            normalv,
            eyev,
            inside,
            over_point,
            under_point,
            reflectv,
            n1,
            n2
        }
    }
}
pub struct Computations {
    pub t: f32,
    pub object: Object,
    pub point: Tuple,
    pub normalv: Tuple,
    pub eyev: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
    pub under_point: Tuple,
    pub reflectv: Tuple,
    pub n1: f32,
    pub n2: f32,
}
/// An intersection between a ray and a shape.
#[derive(Clone, Copy, Debug)]
pub struct Intersection {
    pub t: f32,
    pub object: Object,
}
impl Intersection {
    /// Create a new intersection with the given t and shape.
    pub fn new(t: f32, object: Object) -> Intersection {
        Intersection { t, object }
    }
}
impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> Ordering {
        self.t.total_cmp(&other.t)
    }
}
impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        (self.t) == (other.t)
    }
}
impl Eq for Intersection {}

/// A collection of intersections.
#[derive(Debug)]
pub struct Intersections {
    pub inters: Vec<Intersection>,
}
impl Intersections {
    /// Create a new collection of intersections.
    pub fn new(inters: Vec<Intersection>) -> Intersections {
        let mut xs = Intersections { inters };
        xs.inters.sort();
        xs
    }
    /// Return the closest intersection that is not behind the ray.
    pub fn hit(&self) -> Option<Intersection> {
        let mut hit = None;
        for x in self.inters.clone() {
            if x.t > 0.0 {
                hit = Some(x);
                break;
            }
        }
        hit
    }
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Light {
    pub position: Tuple,
    pub intensity: Colour,
}
impl Light {
    /// Create a new light source
    pub fn new(position: Tuple, intensity: Colour) -> Light {
        Light {
            position,
            intensity,
        }
    }
}
pub fn lighting(
    object: Object,
    light: Light,
    point: Tuple,
    eyev: Tuple,
    normalv: Tuple,
    in_shadow: bool,
) -> Colour {
    let pattern_colour = object.pattern_at(point);
    let effective_colour = pattern_colour * light.intensity;
    let ambient = effective_colour * object.material.ambient;
    let lightv = (light.position - point).normalize();
    let light_dot_normal = lightv.dot(normalv);
    let diffuse;
    let specular;
    if in_shadow {
        specular = colour::BLACK;
        diffuse = colour::BLACK;
    } else if light_dot_normal < 0.0 {
        diffuse = colour::BLACK;
        specular = colour::BLACK;
    } else {
        diffuse = effective_colour * object.material.diffuse * lightv.dot(normalv);
        let reflectv = (-lightv).reflect(normalv);
        let reflect_dot_eye = reflectv.dot(eyev);
        if reflect_dot_eye <= 0.0 {
            specular = colour::BLACK;
        } else {
            let factor = f32::powf(reflect_dot_eye, object.material.shininess);
            specular = light.intensity * object.material.specular * factor;
        }
    }
    ambient + diffuse + specular
}
#[cfg(test)]
mod tests {
    use super::Intersection;
    use crate::{
        colour::{self, Colour, BLACK, WHITE},
        matrix,
        ray::{lighting, Intersections, Light, Ray},
        shapes::{Material, Object, Pattern},
        transformation::{rot_z, scale, translation},
        tuple::{point, vector},
        DEFAULT_EPSILON,
    };
    use approx::assert_relative_eq;
    use std::{f32::consts::PI, vec};

    #[test]
    fn create_ray() {
        let origin = point(1.0, 2.0, 3.0);
        let direction = vector(4.0, 5.0, 6.0);
        let r = Ray::new(origin, direction);
        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }
    #[test]
    fn compute_point_from_distance() {
        let r = Ray::new(point(2.0, 3.0, 4.0), vector(1.0, 0.0, 0.0));
        assert_eq!(r.position(0.0), point(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), point(4.5, 3.0, 4.0));
    }
    #[test]
    fn intersect_with_sphere_at_two_points() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Object::new_sphere();
        let xs = r.intersect(&s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }
    #[test]
    fn intersect_with_sphere_at_tangent() {
        let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Object::new_sphere();
        let xs = r.intersect(&s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }
    #[test]
    fn misses_sphere() {
        let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Object::new_sphere();
        let xs = r.intersect(&s);
        assert_eq!(xs.len(), 0);
    }
    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = Object::new_sphere();
        let xs = r.intersect(&s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }
    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let s = Object::new_sphere();
        let xs = r.intersect(&s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }
    #[test]
    fn intersection_encapsulates() {
        let s = Object::new_sphere();
        let i = Intersection::new(3.5, s);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);
    }
    #[test]
    fn aggregating_intersection() {
        let s = Object::new_sphere();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let intersections = Intersections::new(vec![i1, i2]);
        assert_eq!(intersections.inters.len(), 2);
        assert_eq!(intersections.inters[0].t, 1.0);
        assert_eq!(intersections.inters[1].t, 2.0);
    }
    #[test]
    fn intersect_sets_object() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Object::new_sphere();
        let xs = r.intersect(&s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, s);
        assert_eq!(xs[1].object, s);
    }
    #[test]
    fn hit_positive_t() {
        let s = Object::new_sphere();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let intersections = Intersections::new(vec![i2, i1]);
        assert_eq!(intersections.hit().unwrap(), i1);
    }
    #[test]
    fn hit_some_negative_t() {
        let s = Object::new_sphere();
        let i1 = Intersection::new(-1.0, s);
        let i2 = Intersection::new(2.0, s);
        let intersections = Intersections::new(vec![i2, i1]);
        assert_eq!(intersections.hit().unwrap(), i2);
    }
    #[test]
    fn hit_all_negative_t() {
        let s = Object::new_sphere();
        let i1 = Intersection::new(-2.0, s);
        let i2 = Intersection::new(-1.0, s);
        let intersections = Intersections::new(vec![i2, i1]);
        assert_eq!(intersections.hit(), None);
    }
    #[test]
    fn hit_is_lowest_nonnegative() {
        let s = Object::new_sphere();
        let i1 = Intersection::new(5.0, s);
        let i2 = Intersection::new(7.0, s);
        let i3 = Intersection::new(-3.0, s);
        let i4 = Intersection::new(2.0, s);
        let intersections = Intersections::new(vec![i2, i1, i3, i4]);
        assert_eq!(intersections.hit().unwrap(), i4);
    }
    #[test]
    fn translate_ray() {
        let r = Ray::new(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
        let m = translation(3.0, 4.0, 5.0);
        let r2 = r.transform(m);
        assert_eq!(r2.origin, point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, vector(0.0, 1.0, 0.0));
    }
    #[test]
    fn scale_ray() {
        let r = Ray::new(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
        let m = scale(2.0, 3.0, 4.0);
        let r2 = r.transform(m);
        assert_eq!(r2.origin, point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, vector(0.0, 3.0, 0.0));
    }
    #[test]
    fn sphere_default_transform() {
        let s = Object::new_sphere();
        assert_eq!(s.transform, matrix::identity());
    }
    #[test]
    fn change_sphere_transform() {
        let mut s = Object::new_sphere();
        let t = translation(2.0, 3.0, 4.0);
        s.transform = t;
        assert_eq!(s.transform, t);
    }
    #[test]
    fn intersect_scaled_sphere() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = Object::new_sphere();
        let t = scale(2.0, 2.0, 2.0);
        s.transform = t;
        let xs = r.intersect(&s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }
    #[test]
    fn intersect_translated_sphere() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = Object::new_sphere();
        let t = translation(5.0, 0.0, 0.0);
        s.transform = t;
        let xs = r.intersect(&s);
        assert_eq!(xs.len(), 0);
    }
    #[test]
    fn normal_x() {
        let s = Object::new_sphere();
        let n = s.normal_at(point(1.0, 0.0, 0.0));
        assert_eq!(n, vector(1.0, 0.0, 0.0));
    }
    #[test]
    fn normal_y() {
        let s = Object::new_sphere();
        let n = s.normal_at(point(0.0, 1.0, 0.0));
        assert_eq!(n, vector(0.0, 1.0, 0.0));
    }
    #[test]
    fn normal_z() {
        let s = Object::new_sphere();
        let n = s.normal_at(point(0.0, 0.0, 1.0));
        assert_eq!(n, vector(0.0, 0.0, 1.0));
    }
    #[test]
    fn normal_nonaxial() {
        let s = Object::new_sphere();
        let n = s.normal_at(point(
            f32::sqrt(3.0) / 3.0,
            f32::sqrt(3.0) / 3.0,
            f32::sqrt(3.0) / 3.0,
        ));
        assert_relative_eq!(
            n,
            vector(
                f32::sqrt(3.0) / 3.0,
                f32::sqrt(3.0) / 3.0,
                f32::sqrt(3.0) / 3.0
            ),
            epsilon = DEFAULT_EPSILON
        );
    }
    #[test]
    fn normal_is_normalised() {
        let s = Object::new_sphere();
        let n = s.normal_at(point(
            f32::sqrt(3.0) / 3.0,
            f32::sqrt(3.0) / 3.0,
            f32::sqrt(3.0) / 3.0,
        ));
        assert_relative_eq!(n, n.normalize(), epsilon = DEFAULT_EPSILON);
    }
    #[test]
    fn normal_translated() {
        let mut s = Object::new_sphere();
        let t = translation(0.0, 1.0, 0.0);
        s.transform = t;
        let n = s.normal_at(point(0.0, 1.70711, -0.70711));
        assert_relative_eq!(n, vector(0.0, 0.70711, -0.70711), epsilon = DEFAULT_EPSILON);
    }
    #[test]
    fn normal_transformed() {
        let mut s = Object::new_sphere();
        let t = scale(1.0, 0.5, 1.0) * rot_z(PI / 5.0);
        s.transform = t;
        let n = s.normal_at(point(0.0, f32::sqrt(2.0) / 2.0, -f32::sqrt(2.0) / 2.0));
        assert_relative_eq!(n, vector(0.0, 0.97014, -0.24254), epsilon = DEFAULT_EPSILON);
    }
    #[test]
    fn reflect_45() {
        let v = vector(1.0, -1.0, 0.0);
        let n = vector(0.0, 1.0, 0.0);
        let r = v.reflect(n);
        assert_eq!(r, vector(1.0, 1.0, 0.0));
    }
    #[test]
    fn reflect_slanted() {
        let v = vector(0.0, -1.0, 0.0);
        let n = vector(f32::sqrt(2.0) / 2.0, f32::sqrt(2.0) / 2.0, 0.0);
        let r = v.reflect(n);
        assert_relative_eq!(r, vector(1.0, 0.0, 0.0), epsilon = DEFAULT_EPSILON);
    }
    #[test]
    fn create_light() {
        let intensity = colour::Colour::new(1.0, 1.0, 1.0);
        let pos = point(0.0, 0.0, 0.0);
        let light = Light::new(pos, intensity);
        assert_eq!(light.position, pos);
        assert_eq!(light.intensity, intensity);
    }
    #[test]
    fn create_default_material() {
        let m = Material::new();
        assert_eq!(m.colour, colour::WHITE);
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }
    #[test]
    fn lighting_with_eye_between() {
        let o = Object::new_sphere();
        let p = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 0.0, -10.0), colour::WHITE);
        let result = lighting(o, light, p, eyev, normalv, false);
        assert_eq!(result, Colour::new(1.9, 1.9, 1.9));
    }
    #[test]
    fn lighting_with_eye_between_spec_only() {
        let mut m = Material::new();
        m.ambient = 0.0;
        m.diffuse = 0.0;
        m.specular = 0.5;
        let mut o = Object::new_sphere();
        o.material = m;
        let p = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 0.0, -10.0), colour::WHITE);
        let result = lighting(o, light, p, eyev, normalv, false);
        assert_eq!(result, Colour::new(0.5, 0.5, 0.5));
    }
    #[test]
    fn lighting_with_eye_at_45() {
        let o = Object::new_sphere();
        let p = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, f32::sqrt(2.0) / 2.0, -f32::sqrt(2.0) / 2.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 0.0, -10.0), colour::WHITE);
        let result = lighting(o, light, p, eyev, normalv, false);
        assert_eq!(result, Colour::new(1.0, 1.0, 1.0));
    }
    #[test]
    fn lighting_with_light_at_45() {
        let o = Object::new_sphere();
        let p = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 10.0, -10.0), colour::WHITE);
        let result = lighting(o, light, p, eyev, normalv, false);
        assert_relative_eq!(
            result,
            Colour::new(0.7364, 0.7364, 0.7364),
            epsilon = DEFAULT_EPSILON
        );
    }
    #[test]
    fn lighting_with_eye_and_light_at_45() {
        let o = Object::new_sphere();
        let p = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, -f32::sqrt(2.0) / 2.0, -f32::sqrt(2.0) / 2.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 10.0, -10.0), colour::WHITE);
        let result = lighting(o, light, p, eyev, normalv, false);
        assert_relative_eq!(
            result,
            Colour::new(1.63638, 1.63638, 1.63638),
            epsilon = DEFAULT_EPSILON
        );
    }
    #[test]
    fn lighting_behind_object() {
        let o = Object::new_sphere();
        let p = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 0.0, 10.0), colour::WHITE);
        let result = lighting(o, light, p, eyev, normalv, false);
        assert_eq!(result, Colour::new(0.1, 0.1, 0.1));
    }
    #[test]
    fn prepare_comps() {
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let s = Object::new_sphere();
        let i = Intersection::new(4., s);
        let comps = r.prepare_computations(&i, Intersections::new(vec![i]));
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
    }
    #[test]
    fn prepare_compus_hit_from_outside() {
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let s = Object::new_sphere();
        let i = Intersection::new(4., s);
        let comps = r.prepare_computations(&i, Intersections::new(vec![i]));
        assert!(!comps.inside);
    }
    #[test]
    fn prepare_compus_hit_from_inside() {
        let r = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
        let s = Object::new_sphere();
        let i = Intersection::new(1., s);
        let comps = r.prepare_computations(&i, Intersections::new(vec![i]));
        assert_eq!(comps.point, point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
        assert!(comps.inside);
    }
    #[test]
    fn lighting_in_shadow() {
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let l = Light::new(point(0.0, 0.0, -10.0), colour::WHITE);
        let in_shadow = true;
        let result = lighting(
            Object::new(),
            l,
            point(0.0, 0.0, 0.0),
            eyev,
            normalv,
            in_shadow,
        );
        assert_eq!(result, Colour::new(0.1, 0.1, 0.1));
    }
    #[test]
    fn hit_offsets_point() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = Object::new_sphere();
        s.transform = translation(0.0, 0.0, 1.0);
        let i = Intersection::new(5.0, s);
        let comps = r.prepare_computations(&i, Intersections::new(vec![i]));
        assert!(comps.point.z > comps.over_point.z);
    }
    #[test]
    fn intersect_plane_parallel() {
        let p = Object::new_plane();
        let r = Ray::new(point(0.0, 10.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = r.intersect(&p);
        assert_eq!(xs.len(), 0);
    }
    #[test]
    fn intersect_plane_coplanar() {
        let p = Object::new_plane();
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = r.intersect(&p);
        assert_eq!(xs.len(), 0);
    }
    #[test]
    fn intersect_plane_from_above() {
        let mut p = Object::new_plane();
        p.transform = translation(0.0, -1.0, 0.0);
        let r = Ray::new(point(0.0, 1.0, 0.0), vector(0.0, -1.0, 0.0));
        let xs = r.intersect(&p);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 2.0);
        assert_eq!(xs[0].object, p);
    }
    #[test]
    fn intersect_plane_from_below() {
        let p = Object::new_plane();
        let r = Ray::new(point(0.0, -1.0, 0.0), vector(0.0, 1.0, 0.0));
        let xs = r.intersect(&p);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, p);
    }
    #[test]
    fn lighting_with_pattern() {
        let mut m = Material::new();
        m.pattern = Some(Pattern::new_striped(WHITE, BLACK));
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;
        let mut o = Object::new_sphere();
        o.material = m;
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 0.0, -10.0), WHITE);
        let c1 = lighting(o, light, point(0.9, 0.0, 0.0), eyev, normalv, false);
        let c2 = lighting(o, light, point(1.1, 0.0, 0.0), eyev, normalv, false);
        assert_eq!(c1, WHITE);
        assert_eq!(c2, BLACK);
    }
    #[test]
    fn precomp_reflectv() {
        let object = Object::new_plane();
        let r = Ray::new(point(0.0, 1.0, -1.0), vector(0.0, -(f32::sqrt(2.0)/2.0), f32::sqrt(2.0)/2.0));
        let i = Intersection::new(f32::sqrt(2.0), object);
        let comps = r.prepare_computations(&i, Intersections::new(vec![i]));
        assert_eq!(comps.reflectv, vector(0.0, f32::sqrt(2.0)/2.0, f32::sqrt(2.0)/2.0));
    }
    fn find_n1_n2_setup(index: usize) -> (f32, f32) {
        let mut a = Object::glass_sphere();
        a.transform = scale(2.0, 2.0, 2.0);
        a.material.refractive_index = 1.5;
        let mut b = Object::glass_sphere();
        b.transform = translation(0.0, 0.0, -0.25);
        b.material.refractive_index = 2.0;
        let mut c = Object::glass_sphere();
        c.transform = translation(0.0, 0.0, 0.25);
        c.material.refractive_index = 2.5;
        let r = Ray::new(point(0.0, 0.0, -4.0), vector(0.0, 0.0, 1.0));
        let xs = Intersections::new(vec![Intersection::new(2.0, a),
                                                                Intersection::new(2.75, b),
                                                                Intersection::new(3.25, c),
                                                                Intersection::new(4.75, b),
                                                                Intersection::new(5.25, c),
                                                                Intersection::new(6.0, a)]);
        let comps = r.prepare_computations(&xs.inters[index].clone(), xs); //TODO check if this is the right way to borrow
        (comps.n1, comps.n2)
    }
    #[test]
    fn find_n1_n2_0() {        
        assert_eq!(find_n1_n2_setup(0), (1.0, 1.5));
        assert_eq!(find_n1_n2_setup(1), (1.5, 2.0));
        assert_eq!(find_n1_n2_setup(2), (2.0, 2.5));
        assert_eq!(find_n1_n2_setup(3), (2.5, 2.5));
        assert_eq!(find_n1_n2_setup(4), (2.5, 1.5));
        assert_eq!(find_n1_n2_setup(5), (1.5, 1.0));
    }
    #[test]
    fn compute_under_point() {        
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = Object::glass_sphere();
        s.transform = translation(0.0, 0.0, 1.0); 
        let i = Intersection::new(5.0, s);
        let xs = Intersections::new(vec![i]);
        let comps= r.prepare_computations(&i, xs);
        println!("underpoint z {}", comps.under_point.z);
        assert!(comps.under_point.z > DEFAULT_EPSILON/2.0);
        assert!(comps.point.z < comps.under_point.z);
    }
}

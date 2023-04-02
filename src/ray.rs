use crate::{tuple, shapes::{self, Shape}};

/// A ray.
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
    pub fn intersect(&self, s: &shapes::Shape) -> Vec<Intersection> {
        match s  {
            Shape::Sphere(sphere) => {
                let origin_to_center = self.origin - sphere.center;
                let a = self.direction.dot(self.direction);
                let b = 2.0 * self.direction.dot(origin_to_center);
                let c = origin_to_center.dot(origin_to_center) - sphere.radius * sphere.radius;
                let discriminant = b * b - 4.0 * a * c;
                if discriminant < 0.0 {
                    return vec![];
                }
                let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
                let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
                vec![Intersection::new(t1, shapes::Shape::Sphere(*sphere)), Intersection::new(t2, shapes::Shape::Sphere(*sphere))]
            }
        }
    }
}
/// An intersection between a ray and a shape.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Intersection {
    pub t: f32,
    pub shape: shapes::Shape,
}
impl Intersection {
    /// Create a new intersection with the given t and shape.
    pub fn new(t: f32, shape: shapes::Shape) -> Intersection {
        Intersection { t, shape }
    }
}
/// A collection of intersections.
pub struct Intersections {
    pub inters: Vec<Intersection>,
}
impl Intersections {
    /// Create a new collection of intersections.
    pub fn new(inters: Vec<Intersection>) -> Intersections{
        Intersections{inters}
    }
    /// Return the closest intersection that is not behind the ray.
    pub fn hit(&self) -> Option<Intersection>{
        let min_t = std::f32::INFINITY;
        let mut closest_inter= None;
        for inter in &self.inters {
            if inter.t < min_t && inter.t > 0.0{
                closest_inter = Some(*inter);
            }
        }
        closest_inter
    }
}
// #[cfg(test)]
mod tests {
    use crate::{ray::{Ray, Intersections}, tuple::{self, point, vector}, shapes};
    use super::Intersection;

    #[test]
    fn create_ray() {
        let origin = tuple::Tuple::point(1.0, 2.0, 3.0);
        let direction = tuple::Tuple::vector(4.0, 5.0, 6.0);
        let r = Ray::new(origin, direction);
        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }
    #[test]
    fn compute_point_from_distance() {
        let r = Ray::new(tuple::Tuple::point(2.0, 3.0, 4.0), tuple::Tuple::vector(1.0, 0.0, 0.0));
        assert_eq!(r.position(0.0), tuple::Tuple::point(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), tuple::Tuple::point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), tuple::Tuple::point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), tuple::Tuple::point(4.5, 3.0, 4.0));
    }
    #[test]
    fn intersect_with_sphere_at_two_points() {
        let r = Ray::new(tuple::Tuple::point(0.0, 0.0, -5.0), tuple::Tuple::vector(0.0, 0.0, 1.0));
        let s = shapes::Shape::Sphere(shapes::Sphere::new(tuple::Tuple::point(0.0, 0.0, 0.0), 1.0));
        let xs = r.intersect(&s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }
    #[test]
    fn intersect_with_sphere_at_tangent() {
        let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = shapes::Shape::Sphere(shapes::Sphere::new(tuple::Tuple::point(0.0, 0.0, 0.0), 1.0));
        let xs = r.intersect(&s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }
    #[test]
    fn misses_sphere() {
        let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = shapes::Shape::Sphere(shapes::Sphere::new(tuple::Tuple::point(0.0, 0.0, 0.0), 1.0));
        let xs = r.intersect(&s);
        assert_eq!(xs.len(), 0);
    }
    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = shapes::Shape::Sphere(shapes::Sphere::new(tuple::Tuple::point(0.0, 0.0, 0.0), 1.0));
        let xs = r.intersect(&s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }
    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let s = shapes::Shape::Sphere(shapes::Sphere::new(tuple::Tuple::point(0.0, 0.0, 0.0), 1.0));
        let xs = r.intersect(&s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }
    #[test]
    fn intersection_encapsulates() {
        let s = shapes::Shape::Sphere(shapes::Sphere::new(tuple::Tuple::point(0.0, 0.0, 0.0), 1.0));
        let i = Intersection::new(3.5, s);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.shape, s);
    }
    #[test]
    fn aggregating_intersection() {
        let s = shapes::Shape::Sphere(shapes::Sphere::new(tuple::Tuple::point(0.0, 0.0, 0.0), 1.0));
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
        let s = shapes::Shape::Sphere(shapes::Sphere::new(tuple::Tuple::point(0.0, 0.0, 0.0), 1.0));        
        let xs = r.intersect(&s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].shape, s);
        assert_eq!(xs[1].shape, s);
    }
    #[test]
    fn hit_positive_t() {
        let s = shapes::Shape::Sphere(shapes::Sphere::new(tuple::Tuple::point(0.0, 0.0, 0.0), 1.0));  
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s); 
        let intersections = Intersections::new(vec![i2, i1]);
        assert_eq!(intersections.hit().unwrap(), i1);
    }
    #[test]
    fn hit_some_negative_t() {
        let s = shapes::Shape::Sphere(shapes::Sphere::new(tuple::Tuple::point(0.0, 0.0, 0.0), 1.0));  
        let i1 = Intersection::new(-1.0, s);
        let i2 = Intersection::new(2.0, s); 
        let intersections = Intersections::new(vec![i2, i1]);
        assert_eq!(intersections.hit().unwrap(), i2);
    }
    #[test]
    fn hit_all_negative_t() {
        let s = shapes::Shape::Sphere(shapes::Sphere::new(tuple::Tuple::point(0.0, 0.0, 0.0), 1.0));  
        let i1 = Intersection::new(-2.0, s);
        let i2 = Intersection::new(-1.0, s); 
        let intersections = Intersections::new(vec![i2, i1]);
        assert_eq!(intersections.hit(), None);
    }
    #[test]
    fn hit_is_lowest_nonnegative() {
        let s = shapes::Shape::Sphere(shapes::Sphere::new(tuple::Tuple::point(0.0, 0.0, 0.0), 1.0));  
        let i1 = Intersection::new(5.0, s);
        let i2 = Intersection::new(7.0, s);
        let i3 = Intersection::new(-3.0, s);
        let i4 = Intersection::new(2.0, s); 
        let intersections = Intersections::new(vec![i2, i1, i3, i4]);
        assert_eq!(intersections.hit().unwrap(), i4);
    }
}
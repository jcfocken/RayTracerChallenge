use crate::{canvas::Canvas, colour::{self, Colour, BLACK}, matrix::{identity, Matrix4x4}, ray::{self, lighting, Computations, Intersections, Light, Ray}, shapes::Object, transformation::{scale, translation}, tuple::{point, Tuple}};
pub struct World {
    pub objects: Vec<Object>,
    pub lights: Vec<ray::Light>,
}

impl World {
    pub fn new() -> World {
        World{ objects: Vec::new(), lights: Vec::new() }
    }
    // TODO use the default function
    pub fn default_world() -> World {
        let light = Light::new(point(-10.0, 10.0, -10.0), colour::WHITE);
        let mut s1 = Object::new_sphere();
        s1.material.colour = Colour::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = Object::new_sphere();
        s2.set_transform(scale(0.5, 0.5, 0.5));
        World{ objects: vec![s1, s2], lights: vec![light],}
    }
    /// Find all the intersections of a ray and the objects in the world
    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let mut xs = vec![];
        for _o in self.objects.iter() {
            let mut x = ray.intersect(&_o);
            xs.append(&mut x);
        }
        Intersections::new(xs)
    }
    /// Calculate the shaded colour at a hit 
    pub fn shade_hit(&self, comps: Computations) -> Colour {
        let shadowed = self.is_shadowed(comps.over_point);
        // TODO check there are any lights, iter over all
        lighting(comps.object.material, self.lights[0], comps.point, comps.eyev, comps.normalv, shadowed)
    }
    /// Intersect a ray with the world and find the shade if it hits
    pub fn colour_at(&self, ray: Ray) -> Colour {
        let inters = self.intersect(&ray);
        if let Some(hit) = inters.hit() {
            let comps = ray.prepare_computations(&hit);
            self.shade_hit(comps)
        } else {
            colour::BLACK
        }
    }
    /// Render the world from cam perspective
    pub fn render(self, cam: Camera) -> Canvas {
        let mut image = Canvas::new(cam.hsize, cam.vsize, BLACK);
        for _x in 0..cam.hsize {                    
            for _y in 0..cam.vsize {
                let r = cam.ray_for_pixel(_x, _y);
                let colour = self.colour_at(r);
                image.write_pixel(_x, _y, colour);
            }
        }
        image
    }
    /// Check if the point is shadowed by any object in the world
    pub fn is_shadowed(&self, point: Tuple) -> bool {
        // TODO do this for all lights
        let v = self.lights[0].position - point;
        let distance = v.magnitude();
        let direction = v.normalize();
        let r = Ray::new(point, direction);
        let inters = self.intersect(&r);
        if let Some(hit) = inters.hit() {
            if hit.t < distance { // TODO can I add tis to the if let pattern?
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}
/// Create a view transformation matrix
pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Matrix4x4 {
    let forward = (to-from).normalize();
    let upn = up.normalize();
    let left = forward.cross(upn);
    let true_up = left.cross(forward);
    let mut orientation = Matrix4x4::new();
    orientation.write_value(0, 0, left.x);
    orientation.write_value(0, 1, left.y);
    orientation.write_value(0, 2, left.z);
    orientation.write_value(1, 0, true_up.x);
    orientation.write_value(1, 1, true_up.y);
    orientation.write_value(1, 2, true_up.z);
    orientation.write_value(2, 0, -forward.x);
    orientation.write_value(2, 1, -forward.y);
    orientation.write_value(2, 2, -forward.z);
    orientation.write_value(3, 3, 1.0);
    orientation*translation(-from.x, -from.y, -from.z)
}
pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub fow: f32,
    pub transform: Matrix4x4,
    pub pixel_size: f32,
    half_width: f32,
    half_height: f32,
}
impl Camera {
    pub fn new(hsize: usize, vsize: usize, fow: f32) -> Camera {
        let transform = identity();
        let half_view = (fow/2.0).tan();
        let aspect = hsize as f32/vsize as f32;
        let half_width;
        let half_height;
        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view/aspect;
        } else {            
            half_width = half_view*aspect;
            half_height = half_view;
        }
        let pixel_size = (half_width*2.0)/hsize as f32;
        Camera{hsize, vsize, fow, transform, pixel_size, half_width, half_height}
    }
    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let x_offset = (px as f32 + 0.5) * self.pixel_size;
        let y_offset = (py as f32 + 0.5) * self.pixel_size;
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;
        let pixel = self.transform.inverse() * point(world_x, world_y, -1.0);
        let origin = self.transform.inverse() * point(0.0, 0.0, 0.0);
        let direction = (pixel-origin).normalize();
        Ray::new(origin, direction)
    }
}
#[cfg(test)]
mod tests {
    use std::{f32::consts::PI, vec};
    use approx::assert_relative_eq;
    use super::{World, view_transform};
    use crate::{colour::{self, Colour}, matrix::{identity, Matrix4x4}, ray::{Intersection, Light, Ray}, shapes::Object, transformation::{rot_y, scale, translation}, tuple::{point, vector}, world::Camera, DEFAULT_EPSILON};
    
    #[test]
    fn create_world() {
        let world = World::new();
        assert_eq!(world.objects.len(), 0);
        assert_eq!(world.lights.len(), 0);
    }
    #[test]
    fn create_default_world() {
        let world = World::default_world();
        let light = Light::new(point(-10.0, 10.0, -10.0), colour::WHITE);
        let mut s1 = Object::new_sphere();
        s1.material.colour = Colour::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = Object::new_sphere();
        s2.set_transform(scale(0.5, 0.5, 0.5));
        assert!(world.lights.contains(&light));
        assert!(world.objects.contains(&s1));
        assert!(world.objects.contains(&s2));
    }
    #[test]
    fn intersect_world() {
        let world = World::default_world();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = world.intersect(&r);
        assert_eq!(xs.inters.len(), 4);
        assert_eq!(xs.inters[0].t, 4.0);
        assert_eq!(xs.inters[1].t, 4.5);
        assert_eq!(xs.inters[2].t, 5.5);
        assert_eq!(xs.inters[3].t, 6.0);
    }
    #[test]
    fn shading_intersection() {
        let world = World::default_world();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s =  world.objects[0];
        let x = Intersection::new(4.0, s);
        let comps = r.prepare_computations(&x);
        let c = world.shade_hit(comps);
        assert_relative_eq!(c, Colour::new(0.38066, 0.47583, 0.2855), epsilon=DEFAULT_EPSILON);
    }
    #[test]
    fn shading_intersection_outside() {
        let mut world = World::default_world();
        world.lights = vec![Light::new(point(0.0, 0.25, 0.0), colour::WHITE)];
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s =  world.objects[1];
        let x = Intersection::new(0.5, s);
        let comps = r.prepare_computations(&x);
        let c = world.shade_hit(comps);
        assert_relative_eq!(c, Colour::new(0.90498, 0.90498, 0.90498), epsilon=DEFAULT_EPSILON);
    }
    #[test]
    fn colour_ray_misses() {
        let world = World::default_world();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
        let c = world.colour_at(r);
        assert_eq!(c, colour::BLACK);
    }
    #[test]
    fn colour_ray_hits() {
        let world = World::default_world();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let c = world.colour_at(r);
        assert_relative_eq!(c, Colour::new(0.38066, 0.47583, 0.2855), epsilon=DEFAULT_EPSILON);
    }
    #[test]
    fn trans_matrix_default() {
        let from = point(0.0, 0.0, 0.0);
        let to = point(0.0, 0.0, -1.0);
        let up = vector(0.0, 1.0, 0.0);
        let t = view_transform(from, to, up);
        assert_eq!(t, identity());
    }
    #[test]
    fn trans_matrix_positive_z() {
        let from = point(0.0, 0.0, 0.0);
        let to = point(0.0, 0.0, 1.0);
        let up = vector(0.0, 1.0, 0.0);
        let t = view_transform(from, to, up);
        assert_eq!(t, scale(-1.0, 1.0, -1.0));
    }
    #[test]
    fn trans_matrix_moves_world() {
        let from = point(0.0, 0.0, 8.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        let t = view_transform(from, to, up);
        assert_eq!(t, translation(0.0, 0.0, -8.0));
    }
    #[test]
    fn trans_matrix_arbitrary() {
        let from = point(1.0, 3.0, 2.0);
        let to = point(4.0, -2.0, 8.0);
        let up = vector(1.0, 1.0, 0.0);
        let t = view_transform(from, to, up);
        let mut m = Matrix4x4::new();
        m.write_value(0, 0, -0.50709);
        m.write_value(0, 1, 0.50709);
        m.write_value(0, 2, 0.67612);
        m.write_value(0, 3, -2.36643);
        m.write_value(1, 0, 0.76772);
        m.write_value(1, 1, 0.60609);
        m.write_value(1, 2, 0.12122);
        m.write_value(1, 3, -2.82843);
        m.write_value(2, 0, -0.35857);
        m.write_value(2, 1, 0.59761);
        m.write_value(2, 2, -0.71714);
        m.write_value(3, 3, 1.0);
        assert_relative_eq!(t, m, epsilon=DEFAULT_EPSILON);
    }
    #[test]
    fn create_camera() {
        let cam = Camera::new(160, 120, PI/2.0);
        assert_eq!(cam.hsize, 160);
        assert_eq!(cam.vsize, 120);
        assert_eq!(cam.fow, PI/2.0);
        assert_eq!(cam.transform, identity());
    }
    #[test]
    fn create_camera_pixel_size() {
        let cam = Camera::new(200, 125, PI/2.0);
        assert_eq!(cam.pixel_size, 0.01);
    }
    #[test]
    fn create_camera_pixel_size_vert() {
        let cam = Camera::new(125, 200, PI/2.0);
        assert_eq!(cam.pixel_size, 0.01);
    }
    #[test]
    fn camera_create_ray_center() {
        let cam = Camera::new(201, 101, PI/2.0);
        let r = cam.ray_for_pixel(100, 50);
        assert_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_relative_eq!(r.direction, vector(0.0, 0.0, -1.0), epsilon=DEFAULT_EPSILON);
    }
    #[test]
    fn camera_create_ray_corner() {
        let cam = Camera::new(201, 101, PI/2.0);
        let r = cam.ray_for_pixel(0, 0);
        assert_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_relative_eq!(r.direction, vector(0.66519, 0.33259, -0.66851), epsilon=DEFAULT_EPSILON);
    }
    #[test]
    fn camera_create_ray_transformed() {
        let mut cam = Camera::new(201, 101, PI/2.0);
        cam.transform = rot_y(PI/4.0)*translation(0.0, -2.0, 5.0);
        let r = cam.ray_for_pixel(100, 50);
        assert_eq!(r.origin, point(0.0, 2.0, -5.0000005));
        assert_relative_eq!(r.direction, vector(f32::sqrt(2.0)/2.0, 0.0, -f32::sqrt(2.0)/2.0), epsilon=DEFAULT_EPSILON);
    }
    #[test]
    fn render_world() {
        let world = World::default_world();
        let mut cam = Camera::new(11, 11, PI/2.0);        
        let from = point(0.0, 0.0, -5.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        cam.transform = view_transform(from, to, up);
        let image = world.render(cam);
        assert_relative_eq!(image.pixel_at(5, 5), Colour::new(0.38066, 0.47583, 0.2855), epsilon=DEFAULT_EPSILON);
    }
    #[test]
    fn shadow_default_world() {
        let world = World::default_world();
        let p = point(0.0, 10.0, 0.0);
        let is_shadowed = world.is_shadowed(p);
        assert!(!is_shadowed);
    }
    #[test]
    fn shadow_when_obj_between_point_and_light() {
        let world = World::default_world();
        let p = point(10.0, -10.0, 10.0);
        let is_shadowed = world.is_shadowed(p);
        assert!(is_shadowed);
    }
    #[test]
    fn shadow_when_obj_behind_light() {
        let world = World::default_world();
        let p = point(-20.0, 20.0, -20.0);
        let is_shadowed = world.is_shadowed(p);
        assert!(!is_shadowed);
    }
    #[test]
    fn shadow_when_obj_behind_point() {
        let world = World::default_world();
        let p = point(-2.0, 20.0, -2.0);
        let is_shadowed = world.is_shadowed(p);
        assert!(!is_shadowed);
    }
    #[test]
    fn shade_hit_in_shadow() {
        let light = Light::new(point(0.0,0.0, -10.0), colour::WHITE);
        let s1 = Object::new_sphere();
        let mut s2 = Object::new_sphere();
        s2.set_transform(translation(0.0, 0.0, 10.0));        
        let world = World{ objects: vec![s1, s2], lights: vec![light],};
        let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, s2);
        let comps = r.prepare_computations(&i);
        let c = world.shade_hit(comps);
        assert_eq!(c, Colour::new(0.1, 0.1, 0.1));
    }
}
pub mod canvas;
pub mod colour;
pub mod matrix;
pub mod projectile;
pub mod ray;
pub mod shapes;
pub mod transformation;
pub mod tuple;
pub mod world;
pub mod run {
    use crate::colour::{Colour, BLACK, BLUE, GREEN, RED, WHITE, YELLOW};
    use crate::ray::{lighting, Intersections, Light, Ray};
    use crate::shapes::{Object, Pattern};
    use crate::transformation::{rot_x, rot_y, rot_z, scale, translation};
    use crate::tuple::{point, vector};
    use crate::world::{view_transform, Camera, World};
    use crate::{canvas, colour, projectile, transformation, tuple};
    use std::f32::consts::PI;
    use std::fs;

    pub fn run_projectiles() {
        let mut projectiles: Vec<projectile::Projectile> = Vec::new();
        let mut canv = canvas::Canvas::new(100, 100, colour::WHITE);
        let stepsize: f32 = 100.0;
        let grav = tuple::vector(0.0, -9.81, 0.0);
        projectiles.push(projectile::Projectile::new(
            tuple::point(0.0, 0.0, 0.0),
            tuple::vector(10.0, 40.0, 0.0),
        ));
        projectiles.push(projectile::Projectile::new(
            tuple::point(0.0, 2.0, 0.0),
            tuple::vector(15.0, 30.0, 0.0),
        ));
        'outer: loop {
            for projectile in &mut projectiles {
                let x = projectile.pos.x as isize;
                let y = canv.get_height() as isize - projectile.pos.y.round() as isize;
                if (x < canv.get_width() as isize)
                    && (x >= 0)
                    && (y < canv.get_height() as isize)
                    && (y >= 0)
                {
                    //write pixel if in frame
                    canv.write_pixel(x as usize, y as usize, colour::BLACK);
                }
                if projectile.pos.y < 0.0 {
                    break 'outer;
                }
                projectile.pos = projectile.pos + projectile.vel / stepsize;
                projectile.vel = projectile.vel + grav / stepsize;
            }
        }
        println!("Printing");
        fs::write("renders/pic.ppm", canv.to_ppm()).expect("Error writing image to disk");
    }
    pub fn run_clock() {
        let mut canv = canvas::Canvas::new(100, 100, colour::WHITE);
        let translate_to_center = transformation::translation(50.0, 0.0, 50.0);
        let rotate_one_hour = rot_y(PI / 6.0);
        let mut hour = tuple::point(0.0, 0.0, 30.0);
        for _i in 0..12 {
            canv.write_pixel(
                (translate_to_center * hour).x as usize,
                (translate_to_center * hour).z as usize,
                colour::BLACK,
            );
            hour = rotate_one_hour * hour;
        }
        println!("Printing");
        fs::write("renders/clock.ppm", canv.to_ppm()).expect("Error writing image to disk");
    }
    pub fn run_sphere() {
        let canv_width_px = 1000;
        let canv_height_px = 1000;
        let canv_width = 10.0;
        let canv_height = 10.0;
        let mut canv = canvas::Canvas::new(canv_width_px, canv_height_px, colour::BLACK);
        let origin = point(0.0, 0.0, -5.0);
        let wall_z = 10.0;
        let mut red_sphere = Object::new_sphere();
        red_sphere.material.colour = Colour::new(1.0, 0.2, 1.0);
        red_sphere.material.ambient = 0.05;
        let light = Light::new(point(-10.0, 10.0, -10.0), colour::WHITE);
        let pixel_size_x = canv_width / canv_width_px as f32;
        let pixel_size_y = canv_height / canv_height_px as f32;
        for _x in 0..canv_width_px {
            let x_vector = -(canv_width / 2.0) + (pixel_size_x) * (_x as f32 + 0.5);
            for _y in 0..canv_height_px {
                let y_vector = (canv_height / 2.0) - (pixel_size_y) * (_y as f32 + 0.5); // add half a pixel otherwise camera is slightly offset
                let direction = point(x_vector, y_vector, wall_z);
                let r = Ray::new(origin, (direction - origin).normalize());
                let inter = r.intersect(&red_sphere);
                let inters = Intersections::new(inter);
                if let Some(hit_inter) = inters.hit() {
                    let point = r.position(hit_inter.t);
                    let normal = hit_inter.object.normal_at(point);
                    let eye = -(r.direction);
                    let colour = lighting(hit_inter.object, light, point, eye, normal, false);
                    canv.write_pixel(_x, _y, colour);
                }
            }
        }
        fs::write("renders/sphere.ppm", canv.to_ppm()).expect("Error writing image to disk");
    }
    pub fn run_sphere_render() {
        let light = Light::new(point(-10.0, 10.0, -10.0), colour::WHITE);
        let mut red_sphere = Object::new_sphere();
        red_sphere.material.colour = Colour::new(1.0, 0.2, 1.0);
        red_sphere.material.ambient = 0.05;
        let world = World {
            objects: vec![red_sphere],
            lights: vec![light],
        };
        let mut cam = Camera::new(1000, 1000, PI / 5.0);
        let from = point(0.0, 0.0, -5.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        cam.transform = view_transform(from, to, up);
        let image = world.render(cam);
        fs::write("renders/sphere_render.ppm", image.to_ppm())
            .expect("Error writing image to disk");
    }
    pub fn run_scene_render() {
        let light = Light::new(point(-10.0, 10.0, -10.0), colour::WHITE);

        let mut floor = Object::new_sphere();
        floor.transform = scale(10.0, 0.01, 10.0);
        floor.material.colour = Colour::new(1.0, 0.9, 0.9);
        floor.material.specular = 0.0;

        let mut l_wall = Object::new_sphere();
        l_wall.transform = translation(0.0, 0.0, 5.0)
            * rot_y((-PI) / 4.0)
            * rot_x(PI / 2.0)
            * scale(10.0, 0.01, 10.0);

        let mut r_wall = Object::new_sphere();
        r_wall.transform = translation(0.0, 0.0, 5.0)
            * rot_y(PI / 4.0)
            * rot_x(PI / 2.0)
            * scale(10.0, 0.01, 10.0);

        let mut middle = Object::new_sphere();
        middle.transform = translation(-0.5, 1.0, 0.5);
        middle.material.colour = Colour::new(0.1, 1.0, 0.5);
        middle.material.diffuse = 0.7;
        middle.material.specular = 0.3;

        let mut right = Object::new_sphere();
        right.transform = translation(1.5, 0.5, -0.5) * scale(0.5, 0.5, 0.5);
        right.material.colour = Colour::new(0.5, 1.0, 0.1);
        right.material.diffuse = 0.7;
        right.material.specular = 0.3;

        let mut left = Object::new_sphere();
        left.transform = translation(-1.5, 0.33, -0.75) * scale(0.33, 0.33, 0.33);
        left.material.colour = Colour::new(1.0, 0.8, 0.1);
        left.material.diffuse = 0.7;
        left.material.specular = 0.3;

        let objects = vec![floor, l_wall, r_wall, middle, right, left];
        let world = World {
            objects: objects,
            lights: vec![light],
        };
        let mut cam = Camera::new(2000, 1000, PI / 3.0);
        let from = point(0.0, 1.5, -5.0);
        let to = point(0.0, 1.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        cam.transform = view_transform(from, to, up);
        let image = world.render(cam);
        fs::write("renders/scene_render.ppm", image.to_ppm()).expect("Error writing image to disk");
    }
    pub fn run_planes_render() {
        let light = Light::new(point(-10.0, 10.0, -10.0), colour::WHITE);

        let mut floor = Object::new_plane();
        floor.transform = translation(0.0, 0.0, 0.0);
        floor.material.colour = Colour::new(1.0, 0.9, 0.9);
        floor.material.specular = 0.0;

        let mut l_wall = Object::new_plane();
        l_wall.transform = translation(0.0, 0.0, 5.0) * rot_y((-PI) / 4.0) * rot_x(PI / 2.0);

        let mut r_wall = Object::new_plane();
        r_wall.transform = translation(0.0, 0.0, 5.0)
            * rot_y(PI / 4.0)
            * rot_x(PI / 2.0)
            * scale(10.0, 0.01, 10.0);

        let mut middle = Object::new_sphere();
        middle.transform = translation(-0.5, 1.0, 0.5);
        middle.material.colour = Colour::new(0.1, 1.0, 0.5);
        middle.material.diffuse = 0.7;
        middle.material.specular = 0.3;

        let mut right = Object::new_sphere();
        right.transform = translation(1.5, 0.5, -0.5) * scale(0.5, 0.5, 0.5);
        right.material.colour = Colour::new(0.5, 1.0, 0.1);
        right.material.diffuse = 0.7;
        right.material.specular = 0.3;

        let mut left = Object::new_sphere();
        left.transform = translation(-1.5, 0.33, -0.75) * scale(0.33, 0.33, 0.33);
        left.material.colour = Colour::new(1.0, 0.8, 0.1);
        left.material.diffuse = 0.7;
        left.material.specular = 0.3;

        let objects = vec![floor, l_wall, r_wall, middle, right, left];
        let world = World {
            objects: objects,
            lights: vec![light],
        };
        let mut cam = Camera::new(2000, 1000, PI / 3.0);
        let from = point(0.0, 1.5, -5.0);
        let to = point(0.0, 1.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        cam.transform = view_transform(from, to, up);
        let image = world.render(cam);
        fs::write("renders/scene_render.ppm", image.to_ppm()).expect("Error writing image to disk");
    }
    pub fn run_pattern_render() {
        let light = Light::new(point(-10.0, 10.0, -10.0), colour::WHITE);

        let mut floor = Object::new_plane();
        floor.transform = translation(0.0, 0.0, 0.0);
        let p = Pattern::new_checkers(WHITE, BLACK);
        floor.material.pattern = Some(p);
        floor.material.specular = 0.0;

        let mut l_wall = Object::new_plane();
        l_wall.transform = translation(0.0, 0.0, 5.0) * rot_y((-PI) / 4.0) * rot_x(PI / 2.0);
        l_wall.material.pattern = Some(Pattern::new_striped(BLUE, RED));

        let mut r_wall = Object::new_plane();
        r_wall.transform = translation(0.0, 0.0, 5.0) * rot_y(PI / 4.0) * rot_x(PI / 2.0);
        r_wall.material.pattern = Some(Pattern::new_ring(WHITE, GREEN));

        let mut middle = Object::new_sphere();
        middle.transform = translation(-0.5, 1.0, 0.5);
        let mut p = Pattern::new_striped(WHITE, RED);
        p.transformation = translation(0.6, 0.0, 0.0) * scale(0.5, 0.5, 0.5);
        middle.material.pattern = Some(p);
        middle.material.diffuse = 0.7;
        middle.material.specular = 0.3;

        let mut right = Object::new_sphere();
        right.transform = translation(1.5, 0.5, -0.5) * scale(0.5, 0.5, 0.5);
        let mut p = Pattern::new_striped(BLACK, YELLOW);
        p.transformation = scale(0.2, 0.2, 0.2) * rot_z(PI / 2.0);
        right.material.pattern = Some(p);
        right.material.diffuse = 0.7;
        right.material.specular = 0.3;

        let mut left = Object::new_sphere();
        left.transform = translation(-1.5, 0.33, -0.75) * scale(0.33, 0.33, 0.33);
        let mut p = Pattern::new_gradient(BLUE, WHITE);
        p.transformation =  rot_z(PI / 4.0) * scale(2.0, 2.0, 2.0) * translation(0.5, 0.0, 0.0);
        left.material.pattern = Some(p);
        left.material.diffuse = 0.7;
        left.material.specular = 0.3;

        let objects = vec![floor, l_wall, r_wall, middle, right, left];
        let world = World {
            objects: objects,
            lights: vec![light],
        };
        let mut cam = Camera::new(2000, 1000, PI / 3.0);
        let from = point(0.0, 1.5, -5.0);
        let to = point(0.0, 1.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        cam.transform = view_transform(from, to, up);
        let image = world.render(cam);
        fs::write("renders/scene_render.ppm", image.to_ppm()).expect("Error writing image to disk");
    }
    pub fn run_reflective_render() {
        let light = Light::new(point(-10.0, 10.0, -10.0), colour::WHITE);

        let mut floor = Object::new_plane();
        floor.transform = translation(0.0, 0.0, 0.0);
        let p = Pattern::new_checkers(WHITE, BLACK);
        floor.material.pattern = Some(p);
        floor.material.specular = 0.0;
        floor.material.reflective = 0.5;

        let mut l_wall = Object::new_plane();
        l_wall.transform = translation(0.0, 0.0, 5.0) * rot_y((-PI) / 4.0) * rot_x(PI / 2.0);
        let mut p = Pattern::new_striped(BLUE, RED);
        p.transformation = rot_y(PI/8.0);
        l_wall.material.pattern = Some(p);

        let mut r_wall = Object::new_plane();
        r_wall.transform = translation(0.0, 0.0, 5.0) * rot_y(PI / 4.0) * rot_x(PI / 2.0);
        r_wall.material.pattern = Some(Pattern::new_ring(WHITE, GREEN));

        let mut middle = Object::glass_sphere();
        middle.transform = translation(-0.5, 1.0, 0.5);
        p.transformation = translation(0.6, 0.0, 0.0) * scale(0.5, 0.5, 0.5);
        middle.material.ambient = 0.05;
        middle.material.diffuse = 0.1;
        middle.material.specular = 1.0;
        middle.material.shininess = 300.0;
        middle.material.reflective = 0.5;
        middle.material.transparency = 0.9;
        middle.material.refractive_index = 1.0;
        //middle.material.colour = Colour::new(0.1, 0.1, 0.1);

        let mut right = Object::new_sphere();
        right.transform = translation(1.5, 0.5, -0.5) * scale(0.5, 0.5, 0.5);
        let mut p = Pattern::new_striped(BLACK, YELLOW);
        p.transformation = scale(0.2, 0.2, 0.2) * rot_z(PI / 2.0);
        right.material.pattern = Some(p);
        right.material.diffuse = 0.7;
        right.material.specular = 0.3;

        let mut left = Object::new_sphere();
        left.transform = translation(-1.5, 0.33, -0.75) * scale(0.33, 0.33, 0.33);
        let mut p = Pattern::new_gradient(BLUE, WHITE);
        p.transformation =  rot_z(PI / 4.0) * scale(2.0, 2.0, 2.0) * translation(0.5, 0.0, 0.0);
        left.material.pattern = Some(p);
        left.material.diffuse = 0.7;
        left.material.specular = 0.3;

        let mut mirror_ball = Object::new_sphere();
        mirror_ball.transform = translation(1.5, 1.2, -0.75) * scale(0.5, 0.5, 0.5);
        mirror_ball.material.diffuse = 0.7;
        mirror_ball.material.specular = 0.3;
        mirror_ball.material.reflective = 1.0;
        mirror_ball.material.colour = BLACK;

        let objects = vec![floor, l_wall, r_wall, middle, right, left, mirror_ball];
        let world = World {
            objects: objects,
            lights: vec![light],
        };
        let mut cam = Camera::new(2000, 1000, PI / 3.0);
        let from = point(0.0, 1.5, -5.0);
        let to = point(0.0, 1.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        cam.transform = view_transform(from, to, up);
        let image = world.render(cam);
        let time_stamp = chrono::offset::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        fs::write(format!("renders/scene_render_{}.ppm", time_stamp), image.to_ppm()).expect("Error writing image to disk");
    }
}
const DEFAULT_EPSILON: f32 = 0.00001; //TODO does this belong here?

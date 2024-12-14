pub mod tuple;
pub mod colour;
pub mod canvas;
pub mod projectile;
pub mod matrix;
pub mod transformation;
pub mod ray;
pub mod shapes;
pub mod run {
    use crate::ray::{Intersections, Ray};
    use crate::shapes::{self, Sphere};
    use crate::transformation::{rot_z, rot_y, scale, shear};
    use crate::tuple::{point, vector};
    use crate::{canvas, colour, projectile, tuple, transformation};
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
    }pub fn run_clock() {
        let mut canv = canvas::Canvas::new(100, 100, colour::WHITE);
        let translate_to_center = transformation::translation(50.0, 0.0, 50.0);
        let rotate_one_hour = rot_y(PI/6.0);
        let mut hour = tuple::point(0.0, 0.0, 30.0);
        for _i in 0..12 {            
            canv.write_pixel((translate_to_center*hour).x as usize, (translate_to_center*hour).z as usize, colour::BLACK);
            hour = rotate_one_hour*hour;
        }
        println!("Printing");
        fs::write("renders/clock.ppm", canv.to_ppm()).expect("Error writing image to disk");
    }pub fn run_sphere() {
        let canv_width_px = 100;
        let canv_height_px = 100;
        let canv_width = 10.0;
        let canv_height = 10.0;
        let mut canv = canvas::Canvas::new(canv_width_px, canv_height_px, colour::WHITE);
        let origin = point(0.0, 0.0, -100.0);
        let mut red_sphere = Sphere::new();
        //red_sphere.set_transform(scale(5.0, 5.0, 5.0));
        //red_sphere.set_transform(scale(1.0, 0.5, 1.0));
        //red_sphere.set_transform(scale(0.5, 1.0, 1.0));
        //red_sphere.set_transform(rot_z(PI/4.0)*scale(0.5, 1.0, 1.0));
        red_sphere.set_transform(shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0)*scale(0.5, 1.0, 1.0));
        for _x in 0..canv_width_px {    
            for _y in 0..canv_height_px {
                let pixel_size_x = canv_width/canv_width_px as f32;
                let pixel_size_y = canv_height/canv_height_px as f32;
                let x_vector = (canv_width/2.0) - (pixel_size_x) * (_x as f32 + 0.5); // add half a pixel otherwise camera is slightly offset
                let y_vector = -(canv_height/2.0) + (pixel_size_y) * (_y as f32 + 0.5);
                let direction = vector(0.0, 0.0, 100.0) + vector(x_vector, y_vector, 0.0);
                let r = Ray::new(origin, direction.normalize());
                let inter = r.intersect(&shapes::Shape::Sphere(red_sphere));
                let inters = Intersections::new(inter);
                if inters.hit().is_some() {
                    canv.write_pixel(_x, _y, colour::GREEN);
                }
            }        
        }
        println!("Printing");
        fs::write("renders/sphere.ppm", canv.to_ppm()).expect("Error writing image to disk");
    }
}
#[cfg(test)]
const DEFAULT_EPSILON: f32 = 0.00001; //TODO does this belong here?
pub mod tuple;
pub mod colour;
pub mod canvas;
pub mod projectile;
pub mod matrix;
pub mod transformation;
pub mod run {
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
        fs::write("pic.ppm", canv.to_ppm()).expect("Error writing image to disk");
    }pub fn run_clock() {
        let mut canv = canvas::Canvas::new(100, 100, colour::WHITE);
        let translate_to_center = transformation::translation(50.0, 0.0, 50.0);
        let rotate_one_hour = transformation::rot_y(PI/6.0);
        let mut hour = tuple::point(0.0, 0.0, 30.0);
        for _i in 0..12 {            
            canv.write_pixel((translate_to_center*hour).x as usize, (translate_to_center*hour).z as usize, colour::BLACK);
            hour = rotate_one_hour*hour;
        }
        println!("Printing");
        fs::write("clock.ppm", canv.to_ppm()).expect("Error writing image to disk");
    }
}
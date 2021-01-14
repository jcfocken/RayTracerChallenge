use std::fmt;
use ray_tracing::tuple;
use ray_tracing::tuple::Tuple;
use ray_tracing::colour;
use ray_tracing::colour::Colour;

#[derive(Debug,Clone,PartialEq)]
struct Projectile {
    pos : Tuple,
    vel : Tuple,
}

impl fmt::Display for Projectile {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "At {:.2},{:.2},{:.2} moving at {:.2},{:.2},{:.2}",
               self.pos.x,self.pos.y,self.pos.z,self.vel.x,self.vel.y,self.vel.z)
    }
}

fn main() {
    let mut projectiles : Vec<Projectile> = Vec::new();
    let mut stop : bool = false;
    let mut step = 0;
    let stepsize : f32 = 100.0;
    let grav = tuple::vector(0.0,0.0,-9.81);

    projectiles.push(Projectile{
        pos: tuple::point(0.0,0.0,2.0),
        vel: tuple::vector(5.0,0.0,5.0)
    });

    while !stop {
        step += 1;
        for projectile in &mut projectiles {
            println!("Step: {} {}", step, projectile);
            if projectile.pos.z <= 0.0 {
                println!("end");
                stop = true;
            }
            projectile.pos = projectile.pos.clone() + projectile.vel.clone()/stepsize;
            projectile.vel = projectile.vel.clone() + grav.clone()/stepsize;
        }
    }
}
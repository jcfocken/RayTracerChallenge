use crate::tuple;
use std::fmt;
#[derive(Debug, Clone, PartialEq)]
pub struct Projectile {
    pub pos: tuple::Tuple,
    pub vel: tuple::Tuple,
}

impl Projectile {
    pub fn new(pos: tuple::Tuple, vel: tuple::Tuple) -> Projectile {
        Projectile { pos, vel }
    }
}

impl fmt::Display for Projectile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "At {:.2},{:.2},{:.2} moving at {:.2},{:.2},{:.2}",
            self.pos.x, self.pos.y, self.pos.z, self.vel.x, self.vel.y, self.vel.z
        )
    }
}

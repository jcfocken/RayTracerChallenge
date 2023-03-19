use std::ops;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Colour {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

pub const WHITE: Colour = Colour {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
};
pub const BLACK: Colour = Colour {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
};
pub const RED: Colour = Colour {
    red: 1.0,
    green: 0.0,
    blue: 0.0,
};
pub const GREEN: Colour = Colour {
    red: 0.0,
    green: 1.0,
    blue: 0.0,
};
pub const BLUE: Colour = Colour {
    red: 0.0,
    green: 0.0,
    blue: 1.0,
};

impl Colour {
    pub fn new(red: f32, green: f32, blue: f32) -> Colour {
        Colour { red, green, blue }
    }
    pub fn normalize(&self, max: usize) -> (usize, usize, usize) {
        let red: usize;
        let green: usize;
        let blue: usize;
        match self.red {
            x if x < 0.0 => red = 0,
            x if x > 1.0 => red = max,
            x => red = (x * max as f32).round() as usize,
        }
        match self.green {
            x if x < 0.0 => green = 0,
            x if x > 1.0 => green = max,
            x => green = (x * max as f32).round() as usize,
        }
        match self.blue {
            x if x < 0.0 => blue = 0,
            x if x > 1.0 => blue = max,
            x => blue = (x * max as f32).round() as usize,
        }
        (red, green, blue)
    }
}

impl ops::Add for Colour {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}
impl ops::Sub for Colour {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}
impl ops::Mul for Colour {
    type Output = Self;

    fn mul(self, rhs: Colour) -> Self::Output {
        Self {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}
impl ops::Mul<f32> for Colour {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}
impl almost::AlmostEqual for Colour {
    type Float = f32;
    const DEFAULT_TOLERANCE: Self::Float = almost::F32_TOLERANCE;
    const MACHINE_EPSILON: Self::Float = f32::EPSILON;
    fn almost_equals_with(self, rhs: Self, tol: Self::Float) -> bool {
        almost::equal_with(self.red, rhs.red, tol)
            && almost::equal_with(self.green, rhs.green, tol)
            && almost::equal_with(self.blue, rhs.blue, tol)
    }

    fn almost_zero_with(self, tol: Self::Float) -> bool {
        almost::zero_with(self.red, tol)
            && almost::zero_with(self.green, tol)
            && almost::zero_with(self.blue, tol)
    }
}

#[cfg(test)]
mod tests {
    use crate::colour::Colour;
    use almost;
    #[test]
    fn is_tuple() {
        let a = Colour {
            red: -0.5,
            green: 0.4,
            blue: 1.7,
        };
        assert_eq!(a.red, -0.5);
        assert_eq!(a.green, 0.4);
        assert_eq!(a.blue, 1.7);
    }
    #[test]
    fn add_colour() {
        let c1 = Colour {
            red: 0.9,
            green: 0.6,
            blue: 0.75,
        };
        let c2 = Colour {
            red: 0.7,
            green: 0.1,
            blue: 0.25,
        };
        let c3 = Colour {
            red: 1.6,
            green: 0.7,
            blue: 1.0,
        };
        assert!(almost::equal(c1 + c2, c3));
    }
    #[test]
    fn subtract_colour() {
        let c1 = Colour {
            red: 0.9,
            green: 0.6,
            blue: 0.75,
        };
        let c2 = Colour {
            red: 0.7,
            green: 0.1,
            blue: 0.25,
        };
        let c3 = Colour {
            red: 0.2,
            green: 0.5,
            blue: 0.5,
        };
        assert!(almost::equal(c1 - c2, c3));
    }
    #[test]
    fn scale_colour() {
        let c1 = Colour {
            red: 0.2,
            green: 0.3,
            blue: 0.4,
        };
        let c2 = Colour {
            red: 0.4,
            green: 0.6,
            blue: 0.8,
        };
        assert!(almost::equal(c1 * 2.0, c2));
    }
    #[test]
    fn mix_colour() {
        let c1 = Colour {
            red: 1.0,
            green: 0.2,
            blue: 0.4,
        };
        let c2 = Colour {
            red: 0.9,
            green: 1.0,
            blue: 0.1,
        };
        let c3 = Colour {
            red: 0.9,
            green: 0.2,
            blue: 0.04,
        };
        assert!(almost::equal(c1 * c2, c3));
    }
}

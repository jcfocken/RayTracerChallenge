use std::ops;
#[derive(Debug, Clone, Copy, PartialEq)]
/// A colour struct. Allows mixing of colours using math operators and normalization.
/// RGB values range from 0.0 to 1.0
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
pub const YELLOW: Colour = Colour {
    red: 1.0,
    green: 1.0,
    blue: 0.0,
};

impl Colour {
    /// Create a new colour struct
    pub fn new(red: f32, green: f32, blue: f32) -> Colour {
        Colour { red, green, blue }
    }
    /// Return RGB values as a tuple normalized from 0 to max
    pub fn normalize(&self, max: usize) -> (usize, usize, usize) {
        let red: usize = match self.red {
            x if x < 0.0 => 0,
            x if x > 1.0 => max,
            x => (x * max as f32).round() as usize,
        };
        let green: usize = match self.green {
            x if x < 0.0 => 0,
            x if x > 1.0 => max,
            x => (x * max as f32).round() as usize,
        };
        let blue: usize = match self.blue {
            x if x < 0.0 => 0,
            x if x > 1.0 => max,
            x => (x * max as f32).round() as usize,
        };
        (red, green, blue)
    }
}
/// Add two colours together
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
/// Subtract the individual RGB channels of the second operand from the corresponding channel of the first operand
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
/// Multiply the individual RGB channels by the corresponding channel from the other colour
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
/// Multiply the individual RGB channels by a scalar
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
impl approx::AbsDiffEq for Colour {
    type Epsilon = f32;
    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        f32::abs_diff_eq(&self.red, &other.red, epsilon)
            && f32::abs_diff_eq(&self.green, &other.green, epsilon)
            && f32::abs_diff_eq(&self.blue, &other.blue, epsilon)
    }

    fn abs_diff_ne(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        !Self::abs_diff_eq(self, other, epsilon)
    }
}
impl approx::RelativeEq for Colour {
    fn default_max_relative() -> Self::Epsilon {
        f32::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        f32::relative_eq(&self.red, &other.red, epsilon, max_relative)
            && f32::relative_eq(&self.green, &other.green, epsilon, max_relative)
            && f32::relative_eq(&self.blue, &other.blue, epsilon, max_relative)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use crate::colour::Colour;
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
        assert_relative_eq!(c1 + c2, c3)
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
        assert_relative_eq!(c1 - c2, c3)
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
        assert_relative_eq!(c1 * 2.0, c2)
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
        assert_relative_eq!(c1 * c2, c3)
    }
}

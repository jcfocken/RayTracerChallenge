use std::{ops, fmt};
#[derive(Debug, Clone, Copy, PartialEq)]
/// A tuple struct. Can be used to represent points and vectors.
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Tuple {
    /// Create a new tuple struct
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Tuple {
        Tuple { x, y, z, w }
    }
    /// Check if tuple is a point
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }
    /// Check if tuple is a vector
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
    /// Return the absolute magnitude of a tuple
    pub fn magnitude(&self) -> f32 {
        let mut sum = 0.0;
        sum += self.x.powi(2);
        sum += self.y.powi(2);
        sum += self.z.powi(2);
        sum += self.w.powi(2);
        sum.sqrt()
    }
    /// Return a normalized tuple
    pub fn normalize(&self) -> Tuple {
        let mag = self.magnitude();
        Tuple {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }
    /// Return the dot product of two tuples
    pub fn dot(&self, other: Tuple) -> f32 {
        let mut sum = 0.0;
        sum += self.x * other.x;
        sum += self.y * other.y;
        sum += self.z * other.z;
        sum += self.w * other.w;
        sum
    }
    /// Return the cross product of two tuples
    pub fn cross(&self, other: Tuple) -> Tuple {
        vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}
impl ops::Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}
impl ops::Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}
impl ops::Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: 0.0 - self.x,
            y: 0.0 - self.y,
            z: 0.0 - self.z,
            w: 0.0 - self.w,
        }
    }
}
impl ops::Mul<f32> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}
impl ops::Div<f32> for Tuple {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        if rhs == 0.0 {
            panic!("Cannot divide by zero-valued number!");
        }
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}
impl approx::AbsDiffEq for Tuple {
    type Epsilon = f32;
    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        f32::abs_diff_eq(&self.x, &other.x, epsilon) &&
        f32::abs_diff_eq(&self.y, &other.y, epsilon) &&
        f32::abs_diff_eq(&self.z, &other.z, epsilon) &&
        f32::abs_diff_eq(&self.w, &other.w, epsilon)
    }

    fn abs_diff_ne(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        !Self::abs_diff_eq(self, other, epsilon)
    }
}
impl approx::RelativeEq for Tuple{
    fn default_max_relative() -> Self::Epsilon {
        f32::default_max_relative()
    }

    fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon)
            -> bool {
        f32::relative_eq(&self.x, &other.x, epsilon, max_relative) &&
        f32::relative_eq(&self.y, &other.y, epsilon, max_relative) &&
        f32::relative_eq(&self.z, &other.z, epsilon, max_relative) &&
        f32::relative_eq(&self.w, &other.w, epsilon, max_relative)        
    }
}
impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:}, {:}, {:}, {:}", self.x, self.y, self.z, self.w)
    }   
}

pub fn point(x: f32, y: f32, z: f32) -> Tuple {
    Tuple { x, y, z, w: 1.0 }
}
pub fn vector(x: f32, y: f32, z: f32) -> Tuple {
    Tuple { x, y, z, w: 0.0 }
}

#[cfg(test)]
mod tests {
    use approx::{relative_eq, assert_relative_eq};

    use crate::tuple::{point, vector, Tuple};
    #[test]
    fn is_point() {
        let a = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }
    #[test]
    fn is_vector() {
        let a = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert!(!a.is_point());
        assert!(a.is_vector());
    }
    #[test]
    fn create_point() {
        let a: Tuple = point(4.3, -4.2, 3.1);
        let b = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert!(a.is_point());
        assert!(!a.is_vector());
        assert_eq! {a,b};
    }
    #[test]
    fn create_vector() {
        let a: Tuple = vector(4.3, -4.2, 3.1);
        let b = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert!(!a.is_point());
        assert!(a.is_vector());
        assert_eq! {a,b};
    }
    #[test]
    fn add_tuples() {
        let a: Tuple = vector(3.0, -2.0, 5.0);
        let b: Tuple = point(-2.0, 3.0, 1.0);
        let c: Tuple = a + b;
        let d: Tuple = point(1.0, 1.0, 6.0);
        assert_eq! {c,d};
    }
    #[test]
    fn sub_points() {
        let a: Tuple = point(3.0, 2.0, 1.0);
        let b: Tuple = point(5.0, 6.0, 7.0);
        let c: Tuple = a - b;
        let d: Tuple = vector(-2.0, -4.0, -6.0);
        assert_eq! {c,d};
    }
    #[test]
    fn sub_vec_from_point() {
        let a: Tuple = point(3.0, 2.0, 1.0);
        let b: Tuple = vector(5.0, 6.0, 7.0);
        let c: Tuple = a - b;
        let d: Tuple = point(-2.0, -4.0, -6.0);
        assert_eq! {c,d};
    }
    #[test]
    fn sub_vecs() {
        let a: Tuple = vector(3.0, 2.0, 1.0);
        let b: Tuple = vector(5.0, 6.0, 7.0);
        let c: Tuple = a - b;
        let d: Tuple = vector(-2.0, -4.0, -6.0);
        assert_eq! {c,d};
    }
    #[test]
    fn sub_vec_from_zero() {
        let z: Tuple = vector(0.0, 0.0, 0.0);
        let v: Tuple = vector(1.0, -2.0, 3.0);
        let c: Tuple = z - v;
        let d: Tuple = vector(-1.0, 2.0, -3.0);
        assert_eq! {c,d};
    }
    #[test]
    fn negate_vec() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        let b = Tuple {
            x: -1.0,
            y: 2.0,
            z: -3.0,
            w: 4.0,
        };
        assert_eq! {-a,b};
    }
    #[test]
    fn mult_by_scalar() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        let b = a * 3.5;
        let c = Tuple {
            x: 3.5,
            y: -7.0,
            z: 10.5,
            w: -14.0,
        };
        assert_eq! {b,c};
    }
    #[test]
    fn mult_by_fraction() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        let b = a * 0.5;
        let c = Tuple {
            x: 0.5000001,
            y: -1.0,
            z: 1.5,
            w: -2.0,
        };
        assert_relative_eq!(b,c);
    }
    #[test]
    fn div_by_scalar() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        let b = a / 2.0;
        let c = Tuple {
            x: 0.5,
            y: -1.0,
            z: 1.5,
            w: -2.0,
        };
        assert_eq! {b,c};
    }
    #[test]
    fn magnitude1() {
        let a: Tuple = vector(1.0, 0.0, 0.0);
        let mag = a.magnitude();
        assert_eq! {mag, 1.0};
    }
    #[test]
    fn magnitude2() {
        let a: Tuple = vector(0.0, 1.0, 0.0);
        let mag: f32 = a.magnitude();
        assert_eq! {mag, 1.0};
    }
    #[test]
    fn magnitude3() {
        let a: Tuple = vector(0.0, 0.0, 1.0);
        let mag: f32 = a.magnitude();
        assert_eq! {mag, 1.0};
    }
    #[test]
    fn magnitude4() {
        let a: Tuple = vector(1.0, 2.0, 3.0);
        let mag: f32 = a.magnitude();
        let b: f32 = 14.0;
        assert_eq! {mag, b.sqrt()};
    }
    #[test]
    fn magnitude5() {
        let a: Tuple = vector(-1.0, -2.0, -3.0);
        let mag: f32 = a.magnitude();
        let b: f32 = 14.0;
        assert_eq! {mag, b.sqrt()};
    }
    #[test]
    fn normalize1() {
        let a: Tuple = vector(4.0, 0.0, 0.0);
        let b: Tuple = a.normalize();
        let c: Tuple = vector(1.0, 0.0, 0.0);
        assert_eq! {b, c};
    }
    #[test]
    fn normalize2() {
        let a: Tuple = vector(1.0, 2.0, 3.0);
        let b: Tuple = a.normalize();
        let c: Tuple = vector(0.26726, 0.53452, 0.80178);
        assert!(relative_eq!(b,c, epsilon = 0.00001))
    }
    #[test]
    fn normalize3() {
        let a: Tuple = vector(1.0, 2.0, 3.0);
        let b: Tuple = a.normalize();
        let mag = b.magnitude();
        assert_relative_eq!(mag, 1.0);
    }
    #[test]
    fn dot() {
        let a: Tuple = vector(1.0, 2.0, 3.0);
        let b: Tuple = vector(2.0, 3.0, 4.0);
        assert_eq! {a.dot(b), 20.0};
    }
    #[test]
    fn cross() {
        let a: Tuple = vector(1.0, 2.0, 3.0);
        let b: Tuple = vector(2.0, 3.0, 4.0);
        let c = b.clone();
        assert_eq! {a.cross(b), vector(-1.0,2.0,-1.0)};
        assert_eq! {c.cross(a), vector(1.0,-2.0,1.0)};
    }
}

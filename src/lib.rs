pub mod tuple {
    use std::ops;
    #[derive(Debug,Clone,PartialEq)]
    pub struct Tuple {
        pub x : f32,
        pub y : f32,
        pub z : f32,
        pub w : f32,
    }

    impl Tuple {
        pub fn is_point(&self) -> bool {
            return if self.w == 1.0 {
                true
            } else {
                false
            }
        }
        pub fn is_vector(&self) -> bool {
            return if self.w == 0.0 {
                true
            } else {
                false
            }
        }
        pub fn magnitude(&self) -> f32 {
            let mut sum = 0.0;
            sum += self.x.powi(2);
            sum += self.y.powi(2);
            sum += self.z.powi(2);
            sum += self.w.powi(2);
            return sum.sqrt();
        }
        pub fn normalize(&self) -> Tuple {
            let mag = self.magnitude();
            Tuple{
                x: self.x/mag,
                y: self.y/mag,
                z: self.z/mag,
                w: self.w/mag,
            }
        }
        pub fn dot(&self, other: Tuple) -> f32 {
            let mut sum = 0.0;
            sum += self.x*other.x;
            sum += self.y*other.y;
            sum += self.z*other.z;
            sum += self.w*other.w;
            sum
        }
        pub fn cross(&self, other: Tuple) -> Tuple {
            vector(self.y*other.z-self.z*other.y,
                   self.z*other.x-self.x*other.z,
                   self.x*other.y-self.y*other.x )
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
            Self { x: self.x*rhs, y: self.y*rhs, z: self.z*rhs, w: self.w*rhs }
        }
    }
    impl ops::Div<f32> for Tuple {
        type Output = Self;

        fn div(self, rhs: f32) -> Self::Output {
            if rhs == 0.0 {
                panic!("Cannot divide by zero-valued number!");
            }
           Self { x: self.x/rhs, y: self.y/rhs, z: self.z/rhs, w: self.w/rhs }
        }
    }
    impl almost::AlmostEqual for Tuple {
        type Float = f32;
        const DEFAULT_TOLERANCE: Self::Float = almost::F32_TOLERANCE;
        const MACHINE_EPSILON: Self::Float = f32::EPSILON;
        fn almost_equals_with(self, rhs: Self, tol: Self::Float) -> bool {
            almost::equal_with(self.x, rhs.x, tol) &&
            almost::equal_with(self.y, rhs.y, tol) &&
            almost::equal_with(self.z, rhs.z, tol) &&
            almost::equal_with(self.w, rhs.w, tol)
        }

        fn almost_zero_with(self, tol: Self::Float) -> bool {
            almost::zero_with(self.x,tol) &&
            almost::zero_with(self.y,tol) &&
            almost::zero_with(self.z,tol) &&
            almost::zero_with(self.w,tol)
        }
    }

    pub fn point(x:f32,y:f32,z:f32) -> Tuple {
        Tuple{
            x,
            y,
            z,
            w: 1.0,
        }
    }
    pub fn vector(x:f32,y:f32,z:f32) -> Tuple {
        Tuple{
            x,
            y,
            z,
            w: 0.0,
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::tuple::{Tuple, point, vector};
        use almost;
        #[test]
        fn is_point() {
            let a = Tuple{x:4.3, y:-4.2, z:3.1, w:1.0};
            assert_eq!(a.x, 4.3);
            assert_eq!(a.y, -4.2);
            assert!(a.is_point());
            assert!(!a.is_vector());
        }
        #[test]
        fn is_vector() {
            let a = Tuple{x:4.3, y:-4.2, z:3.1, w:0.0};
            assert_eq!(a.x, 4.3);
            assert_eq!(a.y, -4.2);
            assert!(!a.is_point());
            assert!(a.is_vector());
        }
        #[test]
        fn create_point() {
            let a :Tuple = point(4.3,-4.2,3.1);
            let b = Tuple{x:4.3, y:-4.2, z:3.1, w:1.0};
            assert_eq!(a.x, 4.3);
            assert_eq!(a.y, -4.2);
            assert!(a.is_point());
            assert!(!a.is_vector());
            assert_eq!{a,b};
        }
        #[test]
        fn create_vector() {
            let a :Tuple = vector(4.3,-4.2,3.1);
            let b = Tuple{x:4.3, y:-4.2, z:3.1, w:0.0};
            assert_eq!(a.x, 4.3);
            assert_eq!(a.y, -4.2);
            assert!(!a.is_point());
            assert!(a.is_vector());
            assert_eq!{a,b};
        }
        #[test]
        fn add_tuples() {
            let a :Tuple = vector(3.0,-2.0,5.0);
            let b :Tuple = point(-2.0,3.0,1.0);
            let c :Tuple = a+b;
            let d :Tuple = point(1.0,1.0,6.0);
            assert_eq!{c,d};
        }
        #[test]
        fn sub_points() {
            let a :Tuple = point(3.0,2.0,1.0);
            let b :Tuple = point(5.0,6.0,7.0);
            let c :Tuple = a-b;
            let d :Tuple = vector(-2.0,-4.0,-6.0);
            assert_eq!{c,d};
        }
        #[test]
        fn sub_vec_from_point() {
            let a :Tuple = point(3.0,2.0,1.0);
            let b :Tuple = vector(5.0,6.0,7.0);
            let c :Tuple = a-b;
            let d :Tuple = point(-2.0,-4.0,-6.0);
            assert_eq!{c,d};
        }
        #[test]
        fn sub_vecs() {
            let a :Tuple = vector(3.0,2.0,1.0);
            let b :Tuple = vector(5.0,6.0,7.0);
            let c :Tuple = a-b;
            let d :Tuple = vector(-2.0,-4.0,-6.0);
            assert_eq!{c,d};
        }
        #[test]
        fn sub_vec_from_zero() {
            let z :Tuple = vector(0.0,0.0,0.0);
            let v :Tuple = vector(1.0,-2.0,3.0);
            let c :Tuple = z-v;
            let d :Tuple = vector(-1.0,2.0,-3.0);
            assert_eq!{c,d};
        }
        #[test]
        fn negate_vec() {
            let a = Tuple{x:1.0, y:-2.0, z:3.0, w:-4.0};
            let b = Tuple{x:-1.0, y:2.0, z:-3.0, w:4.0};
            assert_eq!{-a,b};
        }
        #[test]
        fn mult_by_scalar() {
            let a = Tuple{x:1.0, y:-2.0, z:3.0, w:-4.0};
            let b = a*3.5;
            let c = Tuple{x:3.5, y:-7.0, z:10.5, w:-14.0};

            assert_eq!{b,c};
        }
        #[test]
        fn mult_by_fraction() {
            let a = Tuple{x:1.0, y:-2.0, z:3.0, w:-4.0};
            let b = a*0.5;
            let c = Tuple{x:0.5, y:-1.0, z:1.5, w:-2.0};

            assert_eq!{b,c};
        }
        #[test]
        fn div_by_scalar() {
            let a = Tuple{x:1.0, y:-2.0, z:3.0, w:-4.0};
            let b = a/2.0;
            let c = Tuple{x:0.5, y:-1.0, z:1.5, w:-2.0};

            assert_eq!{b,c};
        }
        #[test]
        fn magnitude1() {
            let a : Tuple = vector(1.0,0.0,0.0);
            let mag = a.magnitude();
            assert_eq!{mag, 1.0};
        }
        #[test]
        fn magnitude2() {
            let a : Tuple = vector(0.0,1.0,0.0);
            let mag :f32 = a.magnitude();
            assert_eq!{mag, 1.0};
        }
        #[test]
        fn magnitude3() {
            let a : Tuple = vector(0.0,0.0,1.0);
            let mag :f32 = a.magnitude();
            assert_eq!{mag, 1.0};
        }
        #[test]
        fn magnitude4() {
            let a : Tuple = vector(1.0,2.0,3.0);
            let mag :f32 = a.magnitude();
            let b : f32 = 14.0;
            assert_eq!{mag, b.sqrt()};
        }
        #[test]
        fn magnitude5() {
            let a : Tuple = vector(-1.0,-2.0,-3.0);
            let mag :f32 = a.magnitude();
            let b : f32 = 14.0;
            assert_eq!{mag, b.sqrt()};
        }
        #[test]
        fn normalize1() {
            let a : Tuple = vector(4.0,0.0,0.0);
            let b : Tuple = a.normalize();
            let c : Tuple = vector(1.0,0.0,0.0);
            assert_eq!{b, c};
        }
        #[test]
        fn normalize2() {
            let a : Tuple = vector(1.0,2.0,3.0);
            let b : Tuple = a.normalize();
            let c : Tuple = vector(0.267261, 0.5345225, 0.8017837);
            assert!{almost::equal(b, c)};
        }
        #[test]
        fn normalize3() {
            let a : Tuple = vector(1.0,2.0,3.0);
            let b : Tuple = a.normalize();
            let mag = b.magnitude();
            assert!{almost::equal(mag , 1.0)};
        }
        #[test]
        fn dot() {
            let a : Tuple = vector(1.0,2.0,3.0);
            let b : Tuple = vector(2.0,3.0,4.0);
            assert_eq!{a.dot(b), 20.0};
        }
        #[test]
        fn cross() {
            let a : Tuple = vector(1.0,2.0,3.0);
            let b : Tuple = vector(2.0,3.0,4.0);
            let c = b.clone();
            assert_eq!{a.cross(b), vector(-1.0,2.0,-1.0)};
            assert_eq!{c.cross(a), vector(1.0,-2.0,1.0)};
        }
    }
}
pub mod colour {
    use std::ops;
    #[derive(Debug,Clone,PartialEq)]
    pub struct Colour {
        red : f32,
        green : f32,
        blue : f32,
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
    impl ops::Mul<f32> for Colour {
        type Output = Self;

        fn mul(self, rhs: f32) -> Self::Output {
            Self {red: self.red*rhs, green: self.green*rhs, blue: self.blue*rhs}
        }
    }
    impl ops::Mul for Colour {
        type Output = Self;

        fn mul(self, rhs: Colour) -> Self::Output {
            Self {red: self.red*rhs.red, green: self.green*rhs.green, blue: self.blue*rhs.blue}
        }
    }
    impl almost::AlmostEqual for Colour {
        type Float = f32;
        const DEFAULT_TOLERANCE: Self::Float = almost::F32_TOLERANCE;
        const MACHINE_EPSILON: Self::Float = f32::EPSILON;
        fn almost_equals_with(self, rhs: Self, tol: Self::Float) -> bool {
            almost::equal_with(self.red, rhs.red, tol) &&
            almost::equal_with(self.green, rhs.green, tol) &&
            almost::equal_with(self.blue, rhs.blue, tol)
        }

        fn almost_zero_with(self, tol: Self::Float) -> bool {
            almost::zero_with(self.red,tol) &&
            almost::zero_with(self.green,tol) &&
            almost::zero_with(self.blue,tol)
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::colour::Colour;
        use almost;
        #[test]
        fn is_tuple() {
            let a = Colour { red: -0.5, green: 0.4, blue: 1.7 };
            assert_eq!(a.red, -0.5);
            assert_eq!(a.green, 0.4);
            assert_eq!(a.blue, 1.7);
        }
        #[test]
        fn add_colour() {
            let c1 = Colour { red: 0.9, green: 0.6, blue: 0.75};
            let c2 = Colour { red: 0.7, green: 0.1, blue: 0.25};
            let c3 = Colour { red: 1.6, green: 0.7, blue: 1.0};
            assert!(almost::equal(c1+c2,c3));
        }
        #[test]
        fn subtract_colour() {
            let c1 = Colour { red: 0.9, green: 0.6, blue: 0.75};
            let c2 = Colour { red: 0.7, green: 0.1, blue: 0.25};
            let c3 = Colour { red: 0.2, green: 0.5, blue: 0.5};
            assert!(almost::equal(c1-c2,c3));
        }
        #[test]
        fn scale_colour() {
            let c1 = Colour { red: 0.2, green: 0.3, blue: 0.4};
            let c2 = Colour { red: 0.4, green: 0.6, blue: 0.8};
            assert!(almost::equal(c1*2.0,c2));
        }
        #[test]
        fn mix_colour() {
            let c1 = Colour { red: 1.0, green: 0.2, blue: 0.4};
            let c2 = Colour { red: 0.9, green: 1.0, blue: 0.1};
            let c3 = Colour { red: 0.9, green: 0.2, blue: 0.04};
            assert!(almost::equal(c1*c2,c3));
        }
    }
}


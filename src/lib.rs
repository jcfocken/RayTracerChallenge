pub mod tuple {
    use std::ops;
    use almost::AlmostEqual;
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Tuple {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub w: f32,
    }

    impl Tuple {
        pub fn new(x: f32, y: f32, z: f32, w: f32) -> Tuple {
            Tuple { x, y, z, w }
        }
        pub fn is_point(&self) -> bool {
            self.w.almost_equals(1.0)
        }
        pub fn is_vector(&self) -> bool {
            self.w.almost_zero()
        }
        pub fn magnitude(&self) -> f32 {
            let mut sum = 0.0;
            sum += self.x.powi(2);
            sum += self.y.powi(2);
            sum += self.z.powi(2);
            sum += self.w.powi(2);
            sum.sqrt()
        }
        pub fn normalize(&self) -> Tuple {
            let mag = self.magnitude();
            Tuple {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
                w: self.w / mag,
            }
        }
        pub fn dot(&self, other: Tuple) -> f32 {
            let mut sum = 0.0;
            sum += self.x * other.x;
            sum += self.y * other.y;
            sum += self.z * other.z;
            sum += self.w * other.w;
            sum
        }
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
    impl almost::AlmostEqual for Tuple {
        type Float = f32;
        const DEFAULT_TOLERANCE: Self::Float = almost::F32_TOLERANCE;
        const MACHINE_EPSILON: Self::Float = f32::EPSILON;
        fn almost_equals_with(self, rhs: Self, tol: Self::Float) -> bool {
            almost::equal_with(self.x, rhs.x, tol)
                && almost::equal_with(self.y, rhs.y, tol)
                && almost::equal_with(self.z, rhs.z, tol)
                && almost::equal_with(self.w, rhs.w, tol)
        }

        fn almost_zero_with(self, tol: Self::Float) -> bool {
            almost::zero_with(self.x, tol)
                && almost::zero_with(self.y, tol)
                && almost::zero_with(self.z, tol)
                && almost::zero_with(self.w, tol)
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
        use crate::tuple::{point, vector, Tuple};
        use almost;
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
                x: 0.5,
                y: -1.0,
                z: 1.5,
                w: -2.0,
            };

            assert_eq! {b,c};
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
            let c: Tuple = vector(0.267261, 0.5345225, 0.8017837);
            assert! {almost::equal(b, c)};
        }
        #[test]
        fn normalize3() {
            let a: Tuple = vector(1.0, 2.0, 3.0);
            let b: Tuple = a.normalize();
            let mag = b.magnitude();
            assert! {almost::equal(mag , 1.0)};
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
}
pub mod colour {
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
}
pub mod canvas {
    use crate::colour::Colour;
    pub struct Canvas {
        width: usize,
        height: usize,
        pixels: Vec<Colour>,
    }

    impl Canvas {
        pub fn new(width: usize, height: usize, colour: Colour) -> Canvas {
            let pixel_vec: Vec<Colour> = vec![colour; width * height];
            Canvas {
                width,
                height,
                pixels: pixel_vec,
            }
        }
        pub fn write_pixel(&mut self, width: usize, height: usize, colour: Colour) {
            if (width < self.width) && (height < self.height) {
                let loc = height * self.width + width;
                self.pixels[loc] = colour;
            } else {
                panic!("Writing pixel outside of canvas");
            }
        }
        pub fn pixel_at(&self, width: usize, height: usize) -> Colour {
            let loc = height * self.width + width;
            self.pixels[loc]
        }
        pub fn to_ppm(&self) -> String {
            /*
            convert canvas to ppm
            */

            const MAX_LENGTH: usize = 70;
            let mut column = 0;
            let mut str = format!("P3\n{} {}\n255\n", self.width, self.height);
            let mut new_line = String::new();
            for pixel in &self.pixels {
                for i in 0..3 {
                    match i {
                        0 => new_line.push_str(&pixel.normalize(255).0.to_string()),
                        1 => new_line.push_str(&pixel.normalize(255).1.to_string()),
                        2 => {
                            new_line.push_str(&pixel.normalize(255).2.to_string());
                            column += 1;
                        }
                        _ => (),
                    }
                    if column >= self.width {
                        column = 0;
                        new_line.push('\n');
                    } else if new_line.len() > (MAX_LENGTH - 4) {
                        new_line.push('\n');
                    } else {
                        new_line.push(' ');
                    }
                    if new_line.ends_with('\n') {
                        str.push_str(&new_line);
                        new_line = String::new();
                    }
                }
            }
            str
        }
        pub fn get_height(&self) -> usize {
            self.height
        }
        pub fn get_width(&self) -> usize {
            self.width
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::canvas;
        use crate::colour;
        #[test]
        fn create_canvas() {
            let a = canvas::Canvas::new(100, 50, colour::BLUE);
            assert_eq!(a.width, 100);
            assert_eq!(a.height, 50);
            assert_eq!(a.pixels[0], colour::BLUE);
            assert_eq!(a.pixels[4999], colour::BLUE);
        }
        #[test]
        #[should_panic]
        fn create_canvas_panic() {
            let a = canvas::Canvas::new(100, 50, colour::BLUE);
            assert_eq!(a.pixels[5000], colour::BLUE);
        }
        #[test]
        fn write_pixel() {
            let mut a = canvas::Canvas::new(100, 50, colour::BLUE);
            a.write_pixel(5, 5, colour::RED);
            assert_eq!(a.pixel_at(5, 5), colour::RED);
            assert_eq!(a.pixel_at(5, 6), colour::BLUE);
            a.write_pixel(5, 6, colour::RED);
            assert_eq!(a.pixel_at(5, 6), colour::RED);
        }
        #[test]
        fn canvas_to_ppm() {
            let a = canvas::Canvas::new(5, 3, colour::BLUE);
            let str = a.to_ppm();
            let mut lines = str.lines();
            let mut line: &str;
            match lines.next() {
                Some(i) => line = i,
                None => line = "",
            }
            assert_eq!(line, "P3");

            match lines.next() {
                Some(i) => line = i,
                None => line = "",
            }
            assert_eq!(line, "5 3");

            match lines.next() {
                Some(i) => line = i,
                None => line = "",
            }
            assert_eq!(line, "255");
        }
        #[test]
        fn constructing_pixels() {
            let mut a = canvas::Canvas::new(5, 3, colour::BLACK);
            let c1 = colour::Colour::new(1.5, 0.0, 0.0);
            let c2 = colour::Colour::new(0.0, 0.5, 0.0);
            let c3 = colour::Colour::new(-0.5, 0.0, 1.0);
            a.write_pixel(0, 0, c1);
            a.write_pixel(2, 1, c2);
            a.write_pixel(4, 2, c3);

            let str = a.to_ppm();
            let mut lines = str.lines();
            let mut line: &str;
            for _i in 1..4 {
                lines.next();
            }
            match lines.next() {
                Some(i) => line = i,
                None => line = "",
            }
            assert_eq!(line, "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");

            match lines.next() {
                Some(i) => line = i,
                None => line = "",
            }
            assert_eq!(line, "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");

            match lines.next() {
                Some(i) => line = i,
                None => line = "",
            }
            assert_eq!(line, "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
        }
        #[test]
        //#[ignore]
        fn ppm_linebreak() {
            let a = canvas::Canvas::new(10, 2, colour::Colour::new(1.0, 0.8, 0.6));
            let str = a.to_ppm();
            let mut lines = str.lines();
            let mut line: &str;
            for _ in 1..4 {
                lines.next();
            }
            match lines.next() {
                Some(i) => line = i,
                None => line = "",
            }
            assert_eq!(
                line,
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
            );

            match lines.next() {
                Some(i) => line = i,
                None => line = "",
            }
            assert_eq!(line, "153 255 204 153 255 204 153 255 204 153 255 204 153");

            match lines.next() {
                Some(i) => line = i,
                None => line = "",
            }
            assert_eq!(
                line,
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
            );

            match lines.next() {
                Some(i) => line = i,
                None => line = "",
            }
            assert_eq!(line, "153 255 204 153 255 204 153 255 204 153 255 204 153");
        }
        #[test]
        fn newline_at_end() {
            let a = canvas::Canvas::new(5, 5, colour::RED);
            let str = a.to_ppm();
            let last = str.chars().last().unwrap();
            assert_eq!(last, '\n');
        }
    }
}
pub mod projectile {
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
        // This trait requires `fmt` with this exact signature.
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Write strictly the first element into the supplied output
            // stream: `f`. Returns `fmt::Result` which indicates whether the
            // operation succeeded or failed. Note that `write!` uses syntax which
            // is very similar to `println!`.
            write!(
                f,
                "At {:.2},{:.2},{:.2} moving at {:.2},{:.2},{:.2}",
                self.pos.x, self.pos.y, self.pos.z, self.vel.x, self.vel.y, self.vel.z
            )
        }
    }
}
pub mod run {
    pub fn run() {
        use crate::{canvas, colour, projectile, tuple};
        use std::fs;

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
    }
}
pub mod matrix {
    use std::ops;

    use almost::AlmostEqual;

    use crate::tuple::Tuple;
    
    #[derive(Default, Debug, Clone, Copy)]
    pub struct Matrix2x2 {
        values: [f32; 4],
    }

    impl Matrix2x2 {
        pub fn new() -> Matrix2x2 {
            let vector = [0.0; 4];
            Matrix2x2 { values: vector }
        }
        pub fn fill(&mut self, list: [f32; 4]) {
            if list.len() > (4) {
                panic!("Input list to long");
            }
            self.values = list;
        }        
        pub fn write_value(&mut self, x: usize, y: usize, value: f32) {
            if x >= 2 {
                panic!("x out of bounds");
            }            
            if y >= 2 {
                panic!("y out of bounds");
            }
            let index = x * 2 + y;
            self.values[index] = value;
        }
        pub fn value_at(&self, x: usize, y: usize) -> f32 {
            if x >= 2 {
                panic!("x out of bounds");
            }            
            if y >= 2 {
                panic!("y out of bounds");
            }
            let index = x * 2 + y;
            self.values[index]
        }
        pub fn determinant(&self) -> f32 {
            self.value_at(0, 0)*self.value_at(1, 1)-self.value_at(0, 1)*self.value_at(1, 0)
        }
    }
    impl almost::AlmostEqual for Matrix2x2 {
        type Float = f32;
        const DEFAULT_TOLERANCE: Self::Float = almost::F32_TOLERANCE;
        const MACHINE_EPSILON: Self::Float = f32::EPSILON;
        fn almost_equals_with(self, rhs: Self, tol: Self::Float) -> bool {
            self.values.iter().zip(rhs.values.iter()).map(|(a, b)| a.almost_equals_with(*b, tol)).all(|x|x)
        }
        fn almost_zero_with(self, tol: Self::Float) -> bool {
            self.values.iter().map(|a| a.almost_zero_with(tol)).all(|x|x)
        }
    }
    
    #[derive(Default, Debug, Clone, Copy)]
    pub struct Matrix3x3 {
        values: [f32; 9],
    }

    impl Matrix3x3 {
        pub fn new() -> Matrix3x3 {
            let vector = [0.0; 9];
            Matrix3x3 { values: vector }
        }   
        pub fn fill(&mut self, list: [f32; 9]) {
            if list.len() > (9) {
                panic!("Input list to long");
            }
            self.values = list;
        }   
        pub fn write_value(&mut self, x: usize, y: usize, value: f32) {
            if x >= 3 {
                panic!("x out of bounds");
            }            
            if y >= 3 {
                panic!("y out of bounds");
            }
            let index = x * 3 + y;
            self.values[index] = value;
        }
        pub fn value_at(&self, x: usize, y: usize) -> f32 {
            if x >= 3 {
                panic!("x out of bounds");
            }            
            if y >= 3 {
                panic!("y out of bounds");
            }
            let index = x * 3 + y;
            self.values[index]
        }
        pub fn submatrix(&self, x: usize, y: usize) -> Matrix2x2 {
            if x >= 3 {
                panic!("x out of bounds");
            }            
            if y >= 3 {
                panic!("y out of bounds");
            }
            let mut sub = Matrix2x2::new();
            let mut i = 0;
            for col in 0..3 {
                if col != x {
                    for row in 0..3 {
                        if row != y {
                            sub.values[i] = self.value_at(col, row);
                            i = i+1;
                        }
                    }
                }
            }
            sub
        }
        pub fn minor(&self, x: usize, y: usize) -> f32 {
            let sub = self.submatrix(x, y);
            sub.determinant()
        }
        pub fn cofactor(&self, x: usize, y: usize) -> f32 {
            let minor = self.minor(x, y);
            if (x + y) & 1  == 0 {
                minor
            } else {
                -minor
            }
        }
        pub fn determinant(&self) -> f32 {
            let mut determinant = 0.0;
            for x in 0..3 {
                determinant = determinant + self.value_at(x, 0) * self.cofactor(x, 0);
            }
            determinant
        }
    }
    impl almost::AlmostEqual for Matrix3x3 {
        type Float = f32;
        const DEFAULT_TOLERANCE: Self::Float = almost::F32_TOLERANCE;
        const MACHINE_EPSILON: Self::Float = f32::EPSILON;
        fn almost_equals_with(self, rhs: Self, tol: Self::Float) -> bool {
            self.values.iter().zip(rhs.values.iter()).map(|(a, b)| a.almost_equals_with(*b, tol)).all(|x|x)
        }
        fn almost_zero_with(self, tol: Self::Float) -> bool {
            self.values.iter().map(|a| a.almost_zero_with(tol)).all(|x|x)
        }
    }

    #[derive(Default, Debug, Clone, Copy)]
    pub struct Matrix4x4 {
        values: [f32; 16],
    }

    impl Matrix4x4 {
        pub fn new() -> Matrix4x4 {
            let vector = [0.0; 16];
            Matrix4x4 { values: vector }
        }        
        pub fn fill(&mut self, list: [f32; 16]) {
            if list.len() > (16) {
                panic!("Input list to long");
            }
            self.values = list;
        } 
        pub fn write_value(&mut self, x: usize, y: usize, value: f32) {
            if x >= 4 {
                panic!("x out of bounds");
            }            
            if y >= 4 {
                panic!("y out of bounds");
            }
            let index = x * 4 + y;
            self.values[index] = value;
        }
        pub fn value_at(&self, x: usize, y: usize) -> f32 {
            if x >= 4 {
                panic!("x out of bounds");
            }            
            if y >= 4 {
                panic!("y out of bounds");
            }
            let index = x * 4 + y;
            self.values[index]
        }
        pub fn transpose(&self) -> Matrix4x4 {
            let mut transposed = Matrix4x4::new();
            for x in 0..4 {
                for y in 0..4 {
                    transposed.write_value(x, y, self.value_at(y, x))
                }
            }
            transposed
        }
        pub fn submatrix(&self, x: usize, y: usize) -> Matrix3x3 {
            if x >= 4 {
                panic!("x out of bounds");
            }            
            if y >= 4 {
                panic!("y out of bounds");
            }
            let mut sub = Matrix3x3::new();
            let mut i = 0;
            for col in 0..4 {
                if col != x {
                    for row in 0..4 {
                        if row != y {
                            sub.values[i] = self.value_at(col, row);
                            i = i+1;
                        }
                    }
                }
            }
            sub
        }
        pub fn minor(&self, x: usize, y: usize) -> f32 {
            let sub = self.submatrix(x, y);
            sub.determinant()
        }
        pub fn cofactor(&self, x: usize, y: usize) -> f32 {
            let minor = self.minor(x, y);
            if (x + y) & 1  == 0 {
                minor
            } else {
                -minor
            }
        }
        pub fn determinant(&self) -> f32 {
            let mut determinant = 0.0;
            for x in 0..4 {
                determinant = determinant + self.value_at(x, 0) * self.cofactor(x, 0);
            }
            determinant
        }
        pub fn invertible(&self) -> bool {
            self.determinant() != 0.0
        }
        pub fn inverse(&self) -> Matrix4x4 {
            let det = self.determinant();

            if det.almost_zero() {
                panic!("Matrix is not invertible");
            }
            let mut inv = Matrix4x4::new();
            for x in 0..4 {
                for y in 0..4 {
                    let c = self.cofactor(x, y);
                    inv.write_value(y, x, c/self.determinant())
                }
            }
            inv
        }
    }
    impl almost::AlmostEqual for Matrix4x4 {
        type Float = f32;
        const DEFAULT_TOLERANCE: Self::Float = almost::F32_TOLERANCE;
        const MACHINE_EPSILON: Self::Float = f32::EPSILON;
        fn almost_equals_with(self, rhs: Self, tol: Self::Float) -> bool {
            self.values.iter().zip(rhs.values.iter()).map(|(a, b)| a.almost_equals_with(*b, tol)).all(|x|x)
        }
        fn almost_zero_with(self, tol: Self::Float) -> bool {
            self.values.iter().map(|a| a.almost_zero_with(tol)).all(|x|x)
        }
    }
    impl ops::Mul<Matrix4x4> for Matrix4x4 {
        type Output = Self;

        fn mul(self, rhs: Matrix4x4) -> Self::Output {            
            let mut vector = [0.0; 16];
            for x in 0..4 {
                for y in 0..4 {
                    vector[x * 4 + y] = self.value_at(x, 0)*rhs.value_at(0, y) +
                                        self.value_at(x, 1)*rhs.value_at(1, y) +
                                        self.value_at(x, 2)*rhs.value_at(2, y) +
                                        self.value_at(x, 3)*rhs.value_at(3, y);
                }
            }
            Self {
                values: vector
            }
        }
    }
    impl ops::Mul<Tuple> for Matrix4x4 {
        type Output = Tuple;

        fn mul(self, rhs: Tuple) -> Self::Output {            
            let mut vector = [0.0; 4];
            for x in 0..4 {
                vector[x] = self.value_at(x, 0)*rhs.x +
                            self.value_at(x, 1)*rhs.y +
                            self.value_at(x, 2)*rhs.z +
                            self.value_at(x, 3)*rhs.w;
            }
            Tuple {
                x: vector[0],
                y: vector[1],
                z: vector[2],
                w: vector[3],
            }
        }
    }

    #[cfg(test)]
    mod tests4x4 {
        use almost::AlmostEqual;

        use crate::matrix;
        use crate::tuple;

        #[test]
        fn create_matrix4x4() {
            let mut m = matrix::Matrix4x4::new();

            m.fill([
                1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5,
                16.5,
            ]);

            assert_eq!(m.value_at(0, 0), 1.0);
            assert_eq!(m.value_at(0, 3), 4.0);
            assert_eq!(m.value_at(1, 0), 5.5);
            assert_eq!(m.value_at(1, 2), 7.5);
            assert_eq!(m.value_at(2, 2), 11.0);
            assert_eq!(m.value_at(3, 0), 13.5);
            assert_eq!(m.value_at(3, 2), 15.5);
        }
        #[should_panic]
        #[test]
        fn value_at_oob4x4() {
            let mut m = matrix::Matrix4x4::new();

            m.fill([
                1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5,
                16.5,
            ]);

            assert_eq!(m.value_at(3, 5), 13.5);
        }
        #[should_panic]
        #[test]
        fn value_at_oob4x4_2() {
            let mut m = matrix::Matrix4x4::new();

            m.fill([
                1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5,
                16.5,
            ]);

            m.value_at(1, 5);
        }
        #[test]
        fn almost_zero4x4() {
            let m = matrix::Matrix4x4::new();

            assert!(m.almost_zero());
        }
        #[should_panic]
        #[test]
        fn almost_zero_panic4x4() {
            let mut m = matrix::Matrix4x4::new();

            m.fill([
                1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5,
                16.5,
            ]);
            assert!(m.almost_zero());
        }
        #[test]
        fn almost_equal4x4() {
            let mut m = matrix::Matrix4x4::new();
            let mut n = matrix::Matrix4x4::new();
            
            m.fill([
                1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5,
                16.5,
            ]);
            n.fill([
                1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5,
                16.5,
            ]);

            assert!(m.almost_equals(n));
        }
        #[should_panic]
        #[test]
        fn almost_equal_panic4x4() {
            let mut m = matrix::Matrix4x4::new();
            let mut n = matrix::Matrix4x4::new();
            
            
            m.fill([
                1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5,
                16.5,
            ]);
            n.fill([
                1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5,
                16.51,
            ]);

            assert!(m.almost_equals(n));
        }
        #[test]
        fn multiply4x4() {
            let mut m = matrix::Matrix4x4::new();
            let mut n = matrix::Matrix4x4::new();
            let mut x = matrix::Matrix4x4::new();            
            
            m.fill([
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0,
                2.0,
            ]);
            n.fill([
                -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0,
                8.0,
            ]);
            x.fill([
                20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0, 26.0, 46.0,
                42.0,
            ]);
            
            assert!((m*n).almost_equals(x));
        }
        #[test]
        fn multiply4x4_with_tuple() {
            let mut m = matrix::Matrix4x4::new();
            let n = tuple::Tuple::new(1.0, 2.0, 3.0, 1.0);
            let x = tuple::Tuple::new(18.0, 24.0, 33.0, 1.0);            
            
            m.fill([
                1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0,
                1.0,
            ]);
            
            assert!((m*n).almost_equals(x));
        }
        #[test]
        fn multiply_by_identity() {
            let mut m = matrix::Matrix4x4::new();  
            let mut m2 = matrix::Matrix4x4::new();   
            let mut i = matrix::Matrix4x4::new();        
            
            m.fill([
                0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 16.0, 4.0, 8.0, 16.0,
                32.0,
            ]);
            m2.fill([
                0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 16.0, 4.0, 8.0, 16.0,
                32.0,
            ]);
            i.fill([
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
                1.0,
            ]);
            
            assert!((m*i).almost_equals(m2));
        }
        #[test]
        fn get_sub4x4() {
            let mut m = matrix::Matrix4x4::new();
            let mut n = matrix::Matrix3x3::new();
            
            m.fill([-6.0, 1.0, 1.0, 6.0,
                          -8.0, 5.0, 8.0, 6.0,
                          -1.0, 0.0, 8.0, 2.0,
                          -7.0, 1.0, -1.0, 1.0]);
            n.fill([-6.0, 1.0, 6.0,
                          -8.0, 8.0, 6.0,
                          -7.0, -1.0, 1.0]);

            assert!(m.submatrix(2, 1).almost_equals(n));
        }
        #[test]
        fn transpose4x4() {
            let mut m = matrix::Matrix4x4::new();  
            let mut n = matrix::Matrix4x4::new();        
            
            m.fill([
                0.0, 9.0, 3.0, 0.0,
                9.0, 8.0, 0.0, 8.0,
                1.0, 8.0, 5.0, 3.0,
                0.0, 0.0, 5.0, 8.0,
            ]);
            n.fill([
                0.0, 9.0, 1.0, 0.0,
                9.0, 8.0, 8.0, 0.0,
                3.0, 0.0, 5.0, 5.0,
                0.0, 8.0, 3.0, 8.0,
            ]);
            
            assert!(m.transpose().almost_equals(n));
        }
        #[test]
        fn calc_minor4x4() {
            let mut m = matrix::Matrix4x4::new();
            let mut n = matrix::Matrix3x3::new();
            
            m.fill([-6.0, 1.0, 1.0, 6.0,
                          -8.0, 5.0, 8.0, 6.0,
                          -1.0, 0.0, 8.0, 2.0,
                          -7.0, 1.0, -1.0, 1.0]);
            n.fill([-6.0, 1.0, 6.0,
                          -8.0, 8.0, 6.0,
                          -7.0, -1.0, 1.0]);

            //assert!(m.minor(1, 0).almost_equals(25.0));
        }
        #[test]
        fn determinant4x4() {
            let mut m = matrix::Matrix4x4::new();
            
            m.fill([-2.0, -8.0, 3.0, 5.0,
                          -3.0, 1.0, 7.0, 3.0,
                          1.0, 2.0, -9.0, 6.0,
                          -6.0, 7.0, 7.0, -9.0]);

            assert!(m.cofactor(0, 0).almost_equals(690.0));
            assert!(m.cofactor(0, 1).almost_equals(447.0));
            assert!(m.cofactor(0, 2).almost_equals(210.0));
            assert!(m.cofactor(0, 3).almost_equals(51.0));
            assert!(m.determinant().almost_equals(-4071.0));
        }
        #[test]
        fn invertible4x4() {
            let mut m = matrix::Matrix4x4::new();
            
            m.fill([6.0, 4.0, 4.0, 4.0,
                          5.0, 5.0, 7.0, 6.0,
                          4.0, -9.0, 3.0, -7.0,
                          9.0, 1.0, 7.0, -6.0]);

            assert!(m.determinant().almost_equals(-2120.0));
            assert!(m.invertible());
        }
        #[test]
        fn not_invertible4x4() {
            let mut m = matrix::Matrix4x4::new();
            
            m.fill([-4.0, 2.0, -2.0, -3.0,
                          9.0, 6.0, 2.0, 6.0,
                          0.0, -5.0, 1.0, -5.0,
                          0.0, 0.0, 0.0, 0.0]);

            assert!(m.determinant().almost_equals(-0.0));
            assert!(!m.invertible());
        }
        #[test]
        fn invert4x4() {
            let mut m = matrix::Matrix4x4::new();
            let mut b = matrix::Matrix4x4::new();
            
            m.fill([-5.0, 2.0, 6.0, -8.0,
                          1.0, -5.0, 1.0, 8.0,
                          7.0, 7.0, -6.0, -7.0,
                          1.0, -3.0, 7.0, 4.0]);
            b.fill([0.21805, 0.45113, 0.24060, -0.04511,
                        -0.80827, -1.45677, -0.44361, 0.52068,
                        -0.07895, -0.22368, -0.05263, 0.19737,
                        -0.52256, -0.81391, -0.30075, 0.30639]);

            let n = m.inverse();

            assert!(m.determinant().almost_equals(532.0));
            assert!(m.cofactor(2, 3).almost_equals(-160.0));
            assert!(n.value_at(3, 2).almost_equals(-160.0/532.0));
            assert!(m.cofactor(3, 2).almost_equals(105.0));
            assert!(n.value_at(2, 3).almost_equals(105.0/532.0));
            assert!(n.almost_equals(b));
        }
        #[test]
        fn invert4x4_2() {
            let mut m = matrix::Matrix4x4::new();
            let mut b = matrix::Matrix4x4::new();
            
            m.fill([8.0, -5.0, 9.0, 2.0,
                          7.0, 5.0, 6.0, 1.0,
                          -6.0, 0.0, 9.0, 6.0,
                          -3.0, 0.0, -9.0, -4.0]);
            b.fill([-0.15385, -0.15385, -0.28205, -0.53846,
                         -0.07692, 0.12308, 0.02564, 0.03077,
                          0.35897, 0.35897, 0.43590, 0.92308,
                         -0.69231, -0.69231, -0.76923, -1.92308]);

            let n = m.inverse();

            assert!(n.almost_equals(b));
        }
        #[test]
        fn invert_multiply4x4() {
            let mut a = matrix::Matrix4x4::new();
            let mut b = matrix::Matrix4x4::new();
            
            a.fill([3.0, -9.0, 7.0, 3.0,
                          3.0, -8.0, 2.0, -9.0,
                          -4.0, 4.0, 4.0, 1.0,
                          -6.0, 5.0, -1.0, 1.0]);
            b.fill([8.0, -5.0, 9.0, 2.0,
                        7.0, 5.0, 6.0, 1.0,
                        -6.0, 0.0, 9.0, 6.0,
                        -3.0, 0.0, -9.0, -4.0]);

            let c = a*b;

            assert!((c*b.inverse()).almost_equals(a));
        }
    }
    
    #[cfg(test)]
    mod tests2x2 {
        use almost::AlmostEqual;

        use crate::matrix;
        #[test]
        fn create_matrix2x2() {
            let mut m = matrix::Matrix2x2::new();

            m.fill([-3.0, 5.0, 1.0, -2.0]);

            assert_eq!(m.value_at(0, 0), -3.0);
            assert_eq!(m.value_at(0, 1), 5.0);
            assert_eq!(m.value_at(1, 0), 1.0);
            assert_eq!(m.value_at(1, 1), -2.0);
        }
        #[should_panic]
        #[test]
        fn value_at_oob2x2() {
            let mut m = matrix::Matrix2x2::new();

            m.fill([-3.0, 5.0, 1.0, -2.0]);

            assert_eq!(m.value_at(3, 5), 13.5);
        }
        #[should_panic]
        #[test]
        fn value_at_oob2x2_2() {
            let mut m = matrix::Matrix2x2::new();

            m.fill([-3.0, 5.0, 1.0, -2.0]);

            m.value_at(0, 2);
        }
        #[test]
        fn almost_zero2x2() {
            let m = matrix::Matrix2x2::new();

            assert!(m.almost_zero());
        }
        #[should_panic]
        #[test]
        fn almost_zero_panic2x2() {
            let mut m = matrix::Matrix2x2::new();

            m.fill([-3.0, 5.0, 1.0, -2.0]);

            assert!(m.almost_zero());
        }
        #[test]
        fn almost_equal2x2() {
            let mut m = matrix::Matrix2x2::new();
            let mut n = matrix::Matrix2x2::new();
            
            m.fill([-3.0, 5.0, 1.0, -2.0]);
            n.fill([-3.0, 5.0, 1.0, -2.0]);

            assert!(m.almost_equals(n));
        }
        #[should_panic]
        #[test]
        fn almost_equal_panic2x2() {
            let mut m = matrix::Matrix2x2::new();
            let mut n = matrix::Matrix2x2::new();
            
            m.fill([-3.0, 5.0, 1.0, -2.0]);
            n.fill([-3.0, 5.0, 1.0, -2.001]);

            assert!(m.almost_equals(n));
        }
        #[test]
        fn find_determinant_2x2() {
            let mut m = matrix::Matrix2x2::new();
            
            m.fill([1.0, 5.0, 
                          -3.0, 2.0]);

            assert_eq!(m.determinant(),17.0);
        }
    }
    
    #[cfg(test)]
    mod tests3x3 {
        use almost::AlmostEqual;

        use crate::matrix;
        #[test]
        fn create_matrix3x3() {
            let mut m = matrix::Matrix3x3::new();

            m.fill([-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);

            assert_eq!(m.value_at(0, 0), -3.0);
            assert_eq!(m.value_at(1, 1), -2.0);
            assert_eq!(m.value_at(2, 2), 1.0);
        }
        #[should_panic]
        #[test]
        fn value_at_oob3x3() {
            let mut m = matrix::Matrix3x3::new();

            m.fill([-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);

            assert_eq!(m.value_at(5, 4), 13.5);
        }
        #[should_panic]
        #[test]
        fn value_at_oob3x3_2() {
            let mut m = matrix::Matrix3x3::new();

            m.fill([-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);

            m.value_at(1, 4);
        }
        #[test]
        fn almost_zero3x3() {
            let m = matrix::Matrix3x3::new();

            assert!(m.almost_zero());
        }
        #[should_panic]
        #[test]
        fn almost_zero_panic3x3() {
            let mut m = matrix::Matrix3x3::new();

            m.fill([-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);

            assert!(m.almost_zero());
        }
        #[test]
        fn almost_equal3x3() {
            let mut m = matrix::Matrix3x3::new();
            let mut n = matrix::Matrix3x3::new();
            
            m.fill([-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);
            n.fill([-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);

            assert!(m.almost_equals(n));
        }
        #[should_panic]
        #[test]
        fn almost_equal_panic3x3() {
            let mut m = matrix::Matrix3x3::new();
            let mut n = matrix::Matrix3x3::new();
            
            m.fill([-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);
            n.fill([-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.01]);

            assert!(m.almost_equals(n));
        }
        #[test]
        fn get_sub3x3() {
            let mut m = matrix::Matrix3x3::new();
            let mut n = matrix::Matrix2x2::new();
            
            m.fill([1.0, 5.0, 0.0,
                          -3.0, 2.0, 7.0,
                          0.0, 6.0, -3.0]);
            n.fill([-3.0, 2.0,
                          0.0, 6.0,]);

            assert!(m.submatrix(0, 2).almost_equals(n));
        }
        #[test]
        fn calc_minor3x3() {
            let mut m = matrix::Matrix3x3::new();
            let mut n = matrix::Matrix2x2::new();
            
            m.fill([3.0, 5.0, 0.0,
                          2.0, -1.0, -7.0,
                          6.0, -1.0, 5.0]);
            n.fill([-3.0, 2.0,
                          0.0, 6.0,]);

            assert!(m.minor(1, 0).almost_equals(25.0));
        }
        #[test]
        fn cofactor3x3() {
            let mut m = matrix::Matrix3x3::new();
            
            m.fill([3.0, 5.0, 0.0,
                          2.0, -1.0, -7.0,
                          6.0, -1.0, 5.0]);

            assert!(m.minor(0, 0).almost_equals(-12.0));
            assert!(m.cofactor(0, 0).almost_equals(-12.0));
            assert!(m.minor(1, 0).almost_equals(25.0));
            assert!(m.cofactor(1, 0).almost_equals(-25.0));
        }
        #[test]
        fn determinant3x3() {
            let mut m = matrix::Matrix3x3::new();
            
            m.fill([1.0, 2.0, 6.0,
                          -5.0, 8.0, -4.0,
                          2.0, 6.0, 4.0]);

            assert!(m.cofactor(0, 0).almost_equals(56.0));
            assert!(m.cofactor(0, 1).almost_equals(12.0));
            assert!(m.cofactor(0, 2).almost_equals(-46.0));
            assert!(m.determinant().almost_equals(-196.0));
        }
    }
}
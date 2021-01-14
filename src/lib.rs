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
        pub fn new(x:f32,y:f32,z:f32,w:f32) -> Tuple {
            Tuple{
                x,
                y,
                z,
                w,
            }
        }
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
        pub red : f32,
        pub green : f32,
        pub blue : f32,
    }

    pub const WHITE: Colour = Colour {
        red: 1.0 ,
        green: 1.0,
        blue: 1.0};
    pub const BLACK: Colour = Colour {
        red: 0.0 ,
        green: 0.0,
        blue: 0.0};
    pub const RED: Colour = Colour {
        red: 1.0 ,
        green: 0.0,
        blue: 0.0};
    pub const GREEN: Colour = Colour {
        red: 0.0 ,
        green: 1.0,
        blue: 0.0};
    pub const BLUE: Colour = Colour {
        red: 0.0 ,
        green: 0.0,
        blue: 1.0};

    impl Colour {
        pub fn new(red:f32,green:f32,blue:f32) -> Colour {
            Colour{
                red,
                green,
                blue,
            }
        }
        pub fn normalize(&self, max:usize) -> (usize,usize,usize) {
            let red :usize;
            let green :usize;
            let blue :usize;
            if self.red < 0.0 {
                red = 0;
            } else if self.red > 1.0 {
                red = max;
            } else {
                red = (self.red * max as f32).round() as usize;
            }
            if self.green < 0.0 {
                green = 0;
            } else if self.green > 1.0 {
                green = max;
            } else {
                green = (self.green * max as f32).round() as usize;
            }
            if self.blue < 0.0 {
                blue = 0;
            } else if self.blue > 1.0 {
                blue = max;
            } else {
                blue = (self.blue * max as f32).round() as usize;
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
            Self {red: self.red*rhs.red, green: self.green*rhs.green, blue: self.blue*rhs.blue}
        }
    }
    impl ops::Mul<f32> for Colour {
        type Output = Self;

        fn mul(self, rhs: f32) -> Self::Output {
            Self {red: self.red*rhs, green: self.green*rhs, blue: self.blue*rhs}
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
pub mod canvas {
    use crate::colour::Colour;
    pub struct Canvas {
        pub(crate) width  : usize,
        pub(crate) height : usize,
        pixels : Vec<Colour>,
    }

    impl Canvas {
        pub(crate) fn new(width:usize, height:usize, colour:Colour) -> Canvas{
            let vec: Vec<Colour> = vec!(colour; width*height);
            Canvas { width, height, pixels: vec }
        }
        pub fn write_pixel(&mut self, width:usize, height:usize, colour:Colour) {
            if width < self.width && height < self.height {
                let loc = height * self.width + width;
                self.pixels[loc] = colour;
            } else {
                panic!("Writing pixel outside of canvas");
            }
        }
        pub fn pixel_at(&self, width:usize, height:usize) -> Colour{
            let loc = height * self.width + width;
            self.pixels[loc].clone()
        }
        pub fn to_ppm(&self) -> String{
            let mut column = 0;
            let mut row = 0;
            //let mut line_length = 0;
            //const max_length : usize = 70;
            let mut str = format!("P3\n{} {}\n255\n",self.width,self.height);
            for pixel in &self.pixels {
                str.push_str(&format!("{} {} {}",
                                      pixel.normalize(255).0,
                                      pixel.normalize(255).1,
                                      pixel.normalize(255).2,)
                );
                column += 1;
                if column >= self.width {
                    column = 0;
                    row += 1;
                    str.push_str("\n");
                } else {
                    str.push_str(" ");
                }
            }
            str
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::colour;
        use crate::canvas;
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
            a.write_pixel(5,5,colour::RED);
            assert_eq!(a.pixel_at(5,5), colour::RED);
            assert_eq!(a.pixel_at(5,6), colour::BLUE);
            a.write_pixel(5,6,colour::RED);
            assert_eq!(a.pixel_at(5,6), colour::RED);
        }
        #[test]
        fn canvas_to_ppm() {
            let a = canvas::Canvas::new(5, 3, colour::BLUE);
            let str = a.to_ppm();
            let mut lines = str.lines();
            let mut line : &str;
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
            let c1 = colour::Colour::new(1.5,0.0,0.0);
            let c2 = colour::Colour::new(0.0,0.5,0.0);
            let c3 = colour::Colour::new(-0.5,0.0,1.0);
            a.write_pixel(0,0, c1);
            a.write_pixel(2,1, c2);
            a.write_pixel(4,2, c3);

            let str = a.to_ppm();
            let mut lines = str.lines();
            let mut line : &str;
            for i in 1..4 {
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
        //#[test]
        fn ppm_linebreak() {
            let a = canvas::Canvas::new(10, 2, colour::Colour::new(1.0,0.8,0.6));
            let str = a.to_ppm();
            let mut lines = str.lines();
            let mut line : &str;
            for i in 1..4 {
                lines.next();
            }
            match lines.next() {
                Some(i) => line = i,
                None => line = "",
            }
            assert_eq!(line, "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204");

            match lines.next() {
                Some(i) => line = i,
                None => line = "",
            }
            assert_eq!(line, "153 255 204 153 255 204 153 255 204 153 255 204 153");

            match lines.next() {
                Some(i) => line = i,
                None => line = "",
            }
            assert_eq!(line, "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204");

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
    use std::fmt;
    use crate::tuple;
    #[derive(Debug,Clone,PartialEq)]
    pub struct Projectile {
        pub pos : tuple::Tuple,
        pub vel : tuple::Tuple,
    }

    impl Projectile {
        pub fn new(pos: tuple::Tuple,vel: tuple::Tuple) -> Projectile {
            Projectile {
                pos,
                vel,
            }
        }
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
}
pub mod run {
    pub fn run() {
        use crate::tuple;
        use crate::projectile;
        use crate::canvas;
        use crate::colour;
        use std::fs;

        let mut projectiles : Vec<projectile::Projectile> = Vec::new();
        let mut canv = canvas::Canvas::new(100,100,colour::WHITE);
        let mut stop : bool = false;
        let mut step = 0;
        let stepsize : f32 = 100.0;
        let grav = tuple::vector(0.0,-9.81,0.0);
        projectiles.push(projectile::Projectile::new(tuple::point(0.0,0.0,0.0),
                                                      tuple::vector(10.0,40.0,0.0 )));
        projectiles.push(projectile::Projectile::new(tuple::point(0.0,2.0,0.0),
                                                      tuple::vector(15.0,30.0,0.0 )));
        while !stop {
            step += 1;
            for projectile in &mut projectiles {
                let x = projectile.pos.x as isize;
                let y = canv.height as isize - projectile.pos.y.round() as isize;
                if x < canv.width as isize && x >= 0 && y < canv.height as isize && y >= 0{
                    //write pixel if in frame
                    canv.write_pixel(x as usize, y as usize, colour::BLACK);
                }
                //println!("{} {}", x, y);
                //println!("Step: {}: {}", step, projectile);

                if projectile.pos.y < 0.0 {
                    println!("end");
                    stop = true;
                }
                projectile.pos = projectile.pos.clone() + projectile.vel.clone()/stepsize;
                projectile.vel = projectile.vel.clone() + grav.clone()/stepsize;
            }
        }
        fs::write("pic.ppm", canv.to_ppm());
    }
}
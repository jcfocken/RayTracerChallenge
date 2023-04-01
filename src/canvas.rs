use crate::colour::Colour;

/// A struct representing a canvas. It can create a string containing a representation of itself in ppm format. 
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Colour>,
}
impl Canvas {
    /// Create a new blank canvas of size width x height with colour colour.
    pub fn new(width: usize, height: usize, colour: Colour) -> Canvas {
        let pixel_vec: Vec<Colour> = vec![colour; width * height];
        Canvas {
            width,
            height,
            pixels: pixel_vec,
        }
    }
    /// Set the colour at location (width, height) to colour.
    pub fn write_pixel(&mut self, width: usize, height: usize, colour: Colour) {
        if (width < self.width) && (height < self.height) {
            let loc = height * self.width + width;
            self.pixels[loc] = colour;
        } else {
            panic!("Writing pixel outside of canvas");
        }
    }
    /// Return the pixel colour at a location (width, height).
    pub fn pixel_at(&self, width: usize, height: usize) -> Colour {
        let loc = height * self.width + width;
        self.pixels[loc]
    }
    ///  Return canvas as a string containing a representation in ppm format.
    pub fn to_ppm(&self) -> String {
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
    /// Return the height of the canvas.
    pub fn get_height(&self) -> usize {
        self.height
    }
    /// Return the width of the canvas.
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

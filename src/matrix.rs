use std::{ops, fmt};
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
    pub fn write_value(&mut self, m: usize, n: usize, value: f32) {
        if m >= 2 {
            panic!("m out of bounds");
        }            
        if n >= 2 {
            panic!("n out of bounds");
        }
        let index = m * 2 + n;
        self.values[index] = value;
    }
    pub fn value_at(&self, m: usize, n: usize) -> f32 {
        if m >= 2 {
            panic!("m out of bounds");
        }            
        if n >= 2 {
            panic!("n out of bounds");
        }
        let index = m * 2 + n;
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
impl fmt::Display for Matrix2x2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:5}, {:5}\n{:5}, {:5}", self.values[0], self.values[1], self.values[2], self.values[3])
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
    pub fn write_value(&mut self, m: usize, n: usize, value: f32) {
        if m >= 3 {
            panic!("m out of bounds");
        }            
        if n >= 3 {
            panic!("n out of bounds");
        }
        let index = m * 3 + n;
        self.values[index] = value;
    }
    pub fn value_at(&self, m: usize, n: usize) -> f32 {
        if m >= 3 {
            panic!("m out of bounds");
        }            
        if n >= 3 {
            panic!("n out of bounds");
        }
        let index = m * 3 + n;
        self.values[index]
    }
    pub fn submatrix(&self, m: usize, n: usize) -> Matrix2x2 {
        if m >= 3 {
            panic!("m out of bounds");
        }            
        if n >= 3 {
            panic!("n out of bounds");
        }
        let mut sub = Matrix2x2::new();
        let mut i = 0;
        for row in 0..3 {
            if row != m {
                for col in 0..3 {
                    if col != n {
                        sub.values[i] = self.value_at(row, col);
                        i += 1;
                    }
                }
            }
        }
        sub
    }
    pub fn minor(&self, m: usize, n: usize) -> f32 {
        let sub = self.submatrix(m, n);
        sub.determinant()
    }
    pub fn cofactor(&self, m: usize, n: usize) -> f32 {
        let minor = self.minor(m, n);
        if (m + n) & 1  == 0 {
            minor
        } else {
            -minor
        }
    }
    pub fn determinant(&self) -> f32 {
        let mut determinant = 0.0;
        for m in 0..3 {
            determinant += self.value_at(m, 0) * self.cofactor(m, 0);
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
impl fmt::Display for Matrix3x3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:5}, {:5}, {:5}\n{:5}, {:5}, {:5}\n{:5}, {:5}, {:5}\n",
                self.values[0], self.values[1], self.values[2],
                self.values[3], self.values[4], self.values[5],
                self.values[6], self.values[7], self.values[8])
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
    pub fn write_value(&mut self, m: usize, n: usize, value: f32) {
        if m >= 4 {
            panic!("m out of bounds");
        }            
        if n >= 4 {
            panic!("n out of bounds");
        }
        let index = m * 4 + n;
        self.values[index] = value;
    }
    pub fn value_at(&self, m: usize, n: usize) -> f32 {
        if m >= 4 {
            panic!("m out of bounds");
        }            
        if n >= 4 {
            panic!("n out of bounds");
        }
        let index = m * 4 + n;
        self.values[index]
    }
    pub fn transpose(&self) -> Matrix4x4 {
        let mut transposed = Matrix4x4::new();
        for row in 0..4 {
            for col in 0..4 {
                transposed.write_value(row, col, self.value_at(col, row))
            }
        }
        transposed
    }
    pub fn submatrix(&self, m: usize, n: usize) -> Matrix3x3 {
        if m >= 4 {
            panic!("m out of bounds");
        }            
        if n >= 4 {
            panic!("n out of bounds");
        }
        let mut sub = Matrix3x3::new();
        let mut i = 0;
        for row in 0..4 {
            if row != m {
                for col in 0..4 {
                    if col != n {
                        sub.values[i] = self.value_at(row, col);
                        i += 1;
                    }
                }
            }
        }
        sub
    }
    pub fn minor(&self, m: usize, n: usize) -> f32 {
        let sub = self.submatrix(m, n);
        sub.determinant()
    }
    pub fn cofactor(&self, m: usize, n: usize) -> f32 {
        let minor = self.minor(m, n);
        if (m + n) & 1  == 0 {
            minor
        } else {
            -minor
        }
    }
    pub fn determinant(&self) -> f32 {
        let mut determinant = 0.0;
        for m in 0..4 {
            determinant += self.value_at(m, 0) * self.cofactor(m, 0);
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
        for m in 0..4 {
            for n in 0..4 {
                let c = self.cofactor(m, n);
                inv.write_value(n, m, c/det)
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
        for m in 0..4 {
            for n in 0..4 {
                vector[m * 4 + n] = self.value_at(m, 0)*rhs.value_at(0, n) +
                                    self.value_at(m, 1)*rhs.value_at(1, n) +
                                    self.value_at(m, 2)*rhs.value_at(2, n) +
                                    self.value_at(m, 3)*rhs.value_at(3, n);
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
        for (x, element) in vector.iter_mut().enumerate() {
            *element =  self.value_at(x, 0)*rhs.x +
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
impl fmt::Display for Matrix4x4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:5}, {:5}, {:5}, {:5}\n{:5}, {:5}, {:5}, {:5}\n{:5}, {:5}, {:5}, {:5}\n{:5}, {:5}, {:5}, {:5}",
                self.values[0], self.values[1], self.values[2], self.values[3],
                self.values[4], self.values[5], self.values[6], self.values[7],
                self.values[8], self.values[9], self.values[10], self.values[11],
                self.values[12], self.values[13], self.values[14], self.values[15],)
    }
}
pub fn identity() -> Matrix4x4 {
    let mut ident = Matrix4x4::new();
    ident.write_value(0, 0, 1.0);
    ident.write_value(1, 1, 1.0);
    ident.write_value(2, 2, 1.0);
    ident.write_value(3, 3, 1.0);
    ident
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

    use super::{Matrix3x3,Matrix2x2};
    #[test]
    fn create_matrix3x3() {
        let mut m = Matrix3x3::new();

        m.fill([-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);

        assert_eq!(m.value_at(0, 0), -3.0);
        assert_eq!(m.value_at(1, 1), -2.0);
        assert_eq!(m.value_at(2, 2), 1.0);
    }    
    #[test]
    fn value_at_3x3() {
        let mut m = Matrix3x3::new();

        m.fill([-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);

        assert_eq!(m.value_at(1, 1), -2.0);
        assert_eq!(m.value_at(0, 1),  5.0);
        assert_eq!(m.value_at(0, 0), -3.0);
    }
    #[should_panic]
    #[test]
    fn value_at_oob3x3_() {
        let mut m = Matrix3x3::new();

        m.fill([-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);

        assert_eq!(m.value_at(5, 4), 13.5);
    }
    #[should_panic]
    #[test]
    fn value_at_oob3x3_2() {
        let mut m = Matrix3x3::new();

        m.fill([-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);

        m.value_at(1, 4);
    }
    #[test]
    fn almost_zero3x3() {
        let m = Matrix3x3::new();

        assert!(m.almost_zero());
    }
    #[should_panic]
    #[test]
    fn almost_zero_panic3x3() {
        let mut m = Matrix3x3::new();

        m.fill([-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);

        assert!(m.almost_zero());
    }
    #[test]
    fn almost_equal3x3() {
        let mut m = Matrix3x3::new();
        let mut n = Matrix3x3::new();
        
        m.fill([-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);
        n.fill([-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);

        assert!(m.almost_equals(n));
    }
    #[should_panic]
    #[test]
    fn almost_equal_panic3x3() {
        let mut m = Matrix3x3::new();
        let mut n = Matrix3x3::new();
        
        m.fill([-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);
        n.fill([-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.01]);

        assert!(m.almost_equals(n));
    }
    #[test]
    fn get_sub3x3() {
        let mut m = Matrix3x3::new();
        let mut n = Matrix2x2::new();
        
        m.fill([1.0, 5.0, 0.0,
                      -3.0, 2.0, 7.0,
                      0.0, 6.0, -3.0]);
        n.fill([-3.0, 2.0,
                      0.0, 6.0,]);
        print!("{}",m);

        assert!(m.submatrix(0, 2).almost_equals(n));
    }
    #[test]
    fn calc_minor3x3() {
        let mut m = Matrix3x3::new();
        let mut n = Matrix2x2::new();
        
        m.fill([3.0, 5.0, 0.0,
                      2.0, -1.0, -7.0,
                      6.0, -1.0, 5.0]);
        n.fill([-3.0, 2.0,
                      0.0, 6.0,]);

        assert!(m.minor(1, 0).almost_equals(25.0));
    }
    #[test]
    fn cofactor3x3() {
        let mut m = Matrix3x3::new();
        
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
        let mut m = Matrix3x3::new();
        
        m.fill([1.0, 2.0, 6.0,
                      -5.0, 8.0, -4.0,
                      2.0, 6.0, 4.0]);

        assert!(m.cofactor(0, 0).almost_equals(56.0));
        assert!(m.cofactor(0, 1).almost_equals(12.0));
        assert!(m.cofactor(0, 2).almost_equals(-46.0));
        assert!(m.determinant().almost_equals(-196.0));
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

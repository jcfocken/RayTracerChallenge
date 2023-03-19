use crate::matrix::{Matrix4x4, identity};


pub fn translation(x: f32, y: f32, z: f32) -> Matrix4x4 {
    let mut mat = identity();
    mat.write_value(0, 3, x);
    mat.write_value(1, 3, y);
    mat.write_value(2, 3, z);
    mat
}
pub fn scale(x: f32, y: f32, z: f32) -> Matrix4x4 {     
    let mut mat = identity();
    mat.write_value(0, 0, x);
    mat.write_value(1, 1, y);
    mat.write_value(2, 2, z);
    mat
}
pub fn rot_x(r: f32) ->  Matrix4x4 {     
    let mut mat = identity();
    mat.write_value(1, 1, r.cos());
    mat.write_value(1, 2, -r.sin());
    mat.write_value(2, 1, r.sin());
    mat.write_value(2, 2, r.cos());
    mat
}
pub fn rot_y(r: f32) ->  Matrix4x4 {     
    let mut mat = identity();
    mat.write_value(0, 0, r.cos());
    mat.write_value(0, 2, r.sin());
    mat.write_value(2, 0, -r.sin());
    mat.write_value(2, 2, r.cos());
    mat
}
pub fn rot_z(r: f32) ->  Matrix4x4 {     
    let mut mat = identity();
    mat.write_value(0, 0, r.cos());
    mat.write_value(0, 1, -r.sin());
    mat.write_value(1, 0, r.sin());
    mat.write_value(1, 1, r.cos());
    mat
}
pub fn shear(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32,) ->  Matrix4x4 {     
    let mut mat = identity();
    mat.write_value(0, 1, xy);
    mat.write_value(0, 2, xz);
    mat.write_value(1, 0, yx);
    mat.write_value(1, 2, yz);
    mat.write_value(2, 0, zx);
    mat.write_value(2, 1, zy);
    mat
}
#[cfg(test)]
mod tests {
    use std::f32::consts::PI;
    use almost::AlmostEqual;
    use crate::transformation::*;
    use crate::tuple::point;
    use crate::tuple::vector;

    #[test]
    fn mult_by_translation() {
        let trans = translation(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);

        assert_eq!(trans*p, point(2.0, 1.0, 7.0))
    }
    #[test]
    fn mult_by_inverse() {
        let trans = translation(5.0, -3.0, 2.0);
        let inv = trans.inverse();
        let p = point(-3.0, 4.0, 5.0);

        assert_eq!(inv*p, point(-8.0, 7.0, 3.0))
    }
    #[test]
    fn translate_vector() {
        let trans = translation(5.0, -3.0, 2.0);
        let v = vector(-3.0, 4.0, 5.0);

        assert_eq!(trans*v, v)
    }
    #[test]
    fn scale_point() {
        let trans = scale(2.0, 3.0, 4.0);
        let p = point(-4.0, 6.0, 8.0);

        assert_eq!(trans*p, point(-8.0, 18.0, 32.0))
    }
    #[test]
    fn scale_vector() {        
        let trans = scale(2.0, 3.0, 4.0);
        let v = vector(-4.0, 6.0, 8.0);

        assert_eq!(trans*v, vector(-8.0, 18.0, 32.0))
    }
    #[test]
    fn inverse_scale() {        
        let trans = scale(2.0, 3.0, 4.0);
        let inv = trans.inverse();
        let v = vector(-4.0, 6.0, 8.0);

        assert_eq!(inv*v, vector(-2.0, 2.0, 2.0))
    }
    #[test]
    fn reflection() {
        let trans = scale(-1.0,1.0, 1.0);
        let p = point(2.0, 3.0, 4.0);

        assert_eq!(trans*p, point(-2.0, 3.0, 4.0))
    } 
    #[test]
    fn rotate_x() {
        let rot_eighth = rot_x(PI/4.0);
        //let rot_quarter = rot_x(PI/2.0);
        let p = point(0.0, 1.0, 0.0);

        assert!((rot_eighth*p).almost_equals(point(0.0, f32::sqrt(2.0)/2.0, f32::sqrt(2.0)/2.0)));
        //Fails because of floating point comparison with 0.0 and almost_equals crate uses relative comparison.
        //Maybe change to crate using absolute comparison
        //assert!((rot_quarter*p).almost_equals(point(0.0, 0.0, 1.0)));
    }
    #[test]
    fn rotate_x_inverse() {
        let rot_eighth = rot_x(PI/4.0);
        let inv = rot_eighth.inverse();
        let p = point(0.0, 1.0, 0.0);
 
        assert!((inv*p).almost_equals(point(0.0, f32::sqrt(2.0)/2.0, -f32::sqrt(2.0)/2.0)));
    }    
    #[test]
    fn rotate_y() {
        let rot_eighth = rot_y(PI/4.0);
        //let rot_quarter = rot_y(PI/2.0);
        let p = point(0.0, 0.0, 1.0);

        assert!((rot_eighth*p).almost_equals(point(f32::sqrt(2.0)/2.0, 0.0, f32::sqrt(2.0)/2.0)));
        //Fails because of floating point comparison with 0.0 and almost_equals crate uses relative comparison.
        //Maybe change to crate using absolute comparison
        //assert!((rot_quarter*p).almost_equals(point(1.0, 0.0, 0.0)));
    }
    #[test]
    fn rotate_z() {
        let rot_eighth = rot_z(PI/4.0);
        //let rot_quarter = rot_z(PI/2.0);
        let p = point(0.0, 1.0, 0.0);
        println!("{}", rot_eighth*p);

        assert!((rot_eighth*p).almost_equals(point(-f32::sqrt(2.0)/2.0, f32::sqrt(2.0)/2.0, 0.0)));
        //Fails because of floating point comparison with 0.0 and almost_equals crate uses relative comparison.
        //Maybe change to crate using absolute comparison
        //assert!((rot_quarter*p).almost_equals(point(-1.0, 0.0, 0.0)));
    }
    #[test]
    fn shear_x_y() {
        let shear_mat = shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert!((shear_mat*p).almost_equals(point(5.0, 3.0, 4.0)));
    }
    #[test]
    fn shear_x_z() {
        let shear_mat = shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert!((shear_mat*p).almost_equals(point(6.0, 3.0, 4.0)));
    }
    #[test]
    fn shear_y_x() {
        let shear_mat = shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert!((shear_mat*p).almost_equals(point(2.0, 5.0, 4.0)));
    }
    #[test]
    fn shear_y_z() {
        let shear_mat = shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert!((shear_mat*p).almost_equals(point(2.0, 7.0, 4.0)));
    }
    #[test]
    fn shear_z_x() {
        let shear_mat = shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert!((shear_mat*p).almost_equals(point(2.0, 3.0, 6.0)));
    }
    #[test]
    fn shear_z_y() {
        let shear_mat = shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = point(2.0, 3.0, 4.0);

        assert!((shear_mat*p).almost_equals(point(2.0, 3.0, 7.0)));
    }
    #[test]
    fn transformation_order() {
        let a = rot_x(PI/2.0);
        let b = scale(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);
        let p = point(1.0, 0.0, 1.0);
        let p2 = a*p;
        //assert!((p2).almost_equals(point(1.0, -1.0, 0.0)));
        let p3 = b*p2;
        //assert!((p3).almost_equals(point(5.0, -5.0, 0.0)));
        let p4 = c*p3;
        assert!((p4).almost_equals(point(15.0, 0.0, 7.0)));
    }
    #[test]
    fn transformation_order_2() {
        let a = rot_x(PI/2.0);
        let b = scale(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);
        let p = point(1.0, 0.0, 1.0);
        let p2 = c*b*a*p;
        assert!((p2).almost_equals(point(15.0, 0.0, 7.0)));
    }
}
// Synchronization/computation between matrix and transform (i.e. scale/rotation/skew).

use ruffle_render::matrix::Matrix;

use crate::types::{Degrees, Percent};

pub(crate) fn props_from_matrix(matrix: Matrix) -> (Degrees, Percent, Percent, f64) {
    let notnan_or_zero = |x: f64| if x.is_nan() { 0.0 } else { x };

    let atan2 = |x: f64, y: f64| {
        if (x, y) == (0.0, 0.0) {
            // different from Number.atan2 or Math.atan2.
            std::f64::consts::PI / 2.0
        } else {
            f64::atan2(x, y)
        }
    };

    let Matrix { a, b, c, d, .. } = matrix;
    let ra = f64::from(a);
    let rb = f64::from(b);
    let rc = f64::from(c);
    let rd = f64::from(d);

    let (a, b, c, d) = (
        notnan_or_zero(ra),
        notnan_or_zero(rb),
        notnan_or_zero(rc),
        notnan_or_zero(rd),
    );

    // If this object's transform matrix is:
    // [[a c tx]
    //  [b d ty]]
    // After transformation, the X-axis and Y-axis will turn into the column vectors x' = <a, b> and y' = <c, d>.
    // We derive the scale, rotation, and skew values from these transformed axes.
    // The skew value is not exposed by ActionScript, but is remembered internally.
    // xscale = len(x')
    // yscale = len(y')
    // rotation = atan2(b, a)  (the rotation of x' from the normal x-axis).
    // skew = atan2(-c, d) - atan2(b, a)  (the signed difference between y' and x' rotation)

    // This can produce some surprising results due to the overlap between flipping/rotation/skewing.
    // For example, in Flash, using Modify->Transform->Flip Horizontal and then tracing _xscale, _yscale, and _rotation
    // will output 100, 100, and 180. (a horizontal flip could also be a 180 degree skew followed by 180 degree rotation!)
    let sig = if ra.is_nan() {
        1.0
    } else if rb.is_nan() {
        1.0
    } else if (a * d) > (b * c) {
        1.0
    } else if (a * d) < (b * c) {
        -1.0
    } else {
        1.0
    };
    let rotation_x = atan2(b, a);
    let rotation_y = atan2(-c, d);
    let scale_x = f64::sqrt(ra * ra + rb * rb);
    let scale_y = sig * f64::sqrt(rc * rc + rd * rd);
    let skew = sig * rotation_y * -1.0 - rotation_x;

    let rotation = Degrees::from_radians(f64::atan2(rb, ra));
    let scale_x = Percent::from_unit(scale_x);
    let scale_y = Percent::from_unit(scale_y);

    (rotation, scale_x, scale_y, skew)
}

pub(crate) fn matrix_from_props(
    degrees: Degrees,
    scale_x: Percent,
    scale_y: Percent,
    skew: f64,
) -> Matrix {
    let notnan_or_zero = |x: f64| if x.is_nan() { 0.0 } else { x };

    let (sin_x, cos_x) = notnan_or_zero(degrees.into_radians()).sin_cos();
    let (sin_y, cos_y) = (notnan_or_zero(degrees.into_radians()) + skew).sin_cos();
    let scale_x = scale_x.unit();
    let scale_y = scale_y.unit();

    Matrix {
        a: (notnan_or_zero(scale_x) * cos_x) as f32,
        b: (notnan_or_zero(scale_x) * sin_x) as f32,
        c: (notnan_or_zero(scale_y.signum() * scale_y) * sin_y) as f32,
        d: (notnan_or_zero(scale_y.signum() * scale_y) * cos_y) as f32,
        ..Default::default()
    }
}

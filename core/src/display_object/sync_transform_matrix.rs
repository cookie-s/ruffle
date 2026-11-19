// Synchronization/computation between matrix and transform (i.e. scale/rotation/skew).

use ruffle_render::matrix::Matrix;
use swf::Fixed16;

use crate::types::{Degrees, Percent};

pub(crate) fn props_from_matrix(matrix: Matrix) -> (Degrees, Percent, Percent, f64) {
    let atan2 = |x: f64, y: f64| {
        let nan_to_neginf = |x: f64| if x.is_nan() { f64::NEG_INFINITY } else { x };

        let x = nan_to_neginf(x);
        let y = nan_to_neginf(y);
        if (x, y) == (0.0, 0.0) {
            // different value from Number.atan2 or Math.atan2.
            return std::f64::consts::PI / 2.0;
        }
        f64::atan2(x, y)
    };

    let Matrix { a, b, c, d, .. } = matrix;
    let a = f64::from(a);
    let b = f64::from(b);
    let c = f64::from(c);
    let d = f64::from(d);

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
    if (a, b, c, d) == (-1.0, 0.0, 0.0, 1.0) {
        // early return for this specific case.
        // This is the only confirmed case where scale_x is negative.
        let scale_x = Percent::from_unit(-1.0);
        let scale_y = Percent::from_unit(1.0);
        let rotation = Degrees::from(0.0);
        let skew = 0.0;
        return (rotation, scale_x, scale_y, skew);
    }

    let ysignum = if (a * d) < (b * c) { -1.0 } else { 1.0 };

    let scale_x = f64::sqrt(a * a + b * b);
    let scale_y = ysignum * f64::sqrt(c * c + d * d);

    let rotation = if (nan_to_zero(a), b, c) == (0.0, 0.0, 0.0) && d > 0.0 {
        0.0
    } else {
        f64::atan2(b, a)
    };
    let skew = {
        let rotation_y = if (b, c, nan_to_zero(d)) == (0.0, 0.0, 0.0) && a > 0.0 {
            0.0
        } else {
            // `ysignum` multiplication is required here for pi difference.
            atan2(-ysignum * c, ysignum * d)
        };
        let rotation_x = if (nan_to_zero(a), b, c) == (0.0, 0.0, 0.0) && d > 0.0 {
            0.0
        } else {
            atan2(b, a)
        };
        (rotation_y - rotation_x).rem_euclid(2.0 * std::f64::consts::PI)
    };

    let scale_x = Percent::from_unit(scale_x);
    let scale_y = Percent::from_unit(scale_y);
    let rotation = Degrees::from_radians(rotation);

    (rotation, scale_x, scale_y, skew)
}

pub(crate) fn matrix_from_props(
    degrees: Degrees,
    scale_x: Percent,
    scale_y: Percent,
    skew: f64,
) -> Matrix {
    // Note - in order to match Flash's behavior, the 'scale_x'/`scale_y` field is set to NaN
    // (which gets reported back to ActionScript), but we treat it as 0 for the purposes of
    // updating the matrix
    let scale_x = nan_to_zero(scale_x.unit());
    let scale_y = nan_to_zero(scale_y.unit());

    // Similarly, a rotation of `NaN` can be reported to ActionScript, but we
    // treat it as 0.0 when calculating the matrix
    let degrees = nan_to_zero(degrees.into_radians());

    let (sin_x, cos_x) = degrees.sin_cos();
    let (sin_y, cos_y) = (degrees + skew).sin_cos();

    Matrix {
        a: (scale_x * cos_x) as f32,
        b: (scale_x * sin_x) as f32,
        c: (scale_y * -sin_y) as f32,
        d: (scale_y * cos_y) as f32,
        ..Default::default()
    }
}

/// from_swf_matrix includes tweaks specific to SWF PlaceObject family tags.
pub(crate) fn from_swf_matrix(swf_matrix: swf::Matrix) -> (Degrees, Percent, Percent, f64, Matrix) {
    let matrix: Matrix = swf_matrix.into();
    let (mut rotation, mut scale_x, mut scale_y, mut skew) = props_from_matrix(matrix);

    let negate_scale_x = swf_matrix.a < Fixed16::ZERO
        && (swf_matrix.b, swf_matrix.c) == (Fixed16::ZERO, Fixed16::ZERO)
        && swf_matrix.d > Fixed16::ZERO;
    let negate_scale_y =
        !negate_scale_x && swf_matrix.a * swf_matrix.d < swf_matrix.b * swf_matrix.c;

    if negate_scale_x {
        scale_x = Percent::from(-scale_x.percent());
        rotation = Degrees::from_radians(std::f64::consts::PI);
    }

    // invariant: matrix.
    // - `scale_y *= -1` reverses the sign of `c` and `d`.
    // - `skew += pi` also reverses the sign of `c` and `d`.
    if negate_scale_y {
        scale_y = Percent::from(-scale_y.percent());
        skew = (skew + std::f64::consts::PI).rem_euclid(2.0 * std::f64::consts::PI);
    }

    // atan2(0, 0) is generally arbitrary.
    if (swf_matrix.a, swf_matrix.b) == (Fixed16::ZERO, Fixed16::ZERO) {
        rotation = Degrees::from_radians(std::f64::consts::PI / 2.0);
    }

    // TODO: Refactoring
    // I think it's possible to simplify/merge those logics to props_from_matrix after moving to Fixed16?
    if (swf_matrix.b, swf_matrix.c, swf_matrix.d) == (Fixed16::ZERO, Fixed16::ZERO, Fixed16::ZERO)
        && swf_matrix.a > Fixed16::ZERO
    {
        skew = std::f64::consts::PI / 2.0 - f64::atan2(swf_matrix.b.into(), swf_matrix.a.into());
    }
    if (swf_matrix.a, swf_matrix.b, swf_matrix.c) == (Fixed16::ZERO, Fixed16::ZERO, Fixed16::ZERO)
        && swf_matrix.d > Fixed16::ZERO
    {
        skew = f64::atan2((-swf_matrix.c).into(), swf_matrix.d.into()) - std::f64::consts::PI / 2.0;
    }

    (rotation, scale_x, scale_y, skew, matrix)
}

pub(crate) fn to_skip_update(value: f64, matrix: &Matrix) -> bool {
    value.is_nan() && matrix.a == 0.0 && matrix.b == 0.0 && matrix.c == 0.0 && matrix.d >= 0.0
}

#[inline]
fn nan_to_zero(val: f64) -> f64 {
    if val.is_nan() { 0.0 } else { val }
}

#[cfg(test)]
mod tests {
    // Unit test cases for sync_transform_matrix with some test framework such as NaN or approximation.
    // Easy and quick checks with some samples from the test avm2/display_object_invalid_floats.
    // More comprehensive tests are in the avm2/display_object_invalid_floats test suite.

    use std::fmt::Debug;

    use crate::{display_object::sync_transform_matrix, types::Degrees};
    use ruffle_render::matrix::Matrix;
    use swf::Twips;

    struct DontCare(f64);
    impl PartialEq for DontCare {
        fn eq(&self, _other: &Self) -> bool {
            true
        }
    }
    impl Debug for DontCare {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.0.is_nan() {
                write!(f, "DC(_)")
            } else {
                write!(f, "DC({})", self.0)
            }
        }
    }
    const DC: DontCare = DontCare(f64::NAN);

    struct ApproxEq(f64);
    impl PartialEq for ApproxEq {
        fn eq(&self, other: &Self) -> bool {
            if (0.9999..=1.0001).contains(&(self.0 / other.0)) {
                return true;
            }
            if (self.0 - other.0).abs() < 1e-10 {
                return true;
            }
            if self.0.is_nan() && other.0.is_nan() {
                return true;
            }
            f64::eq(&self.0, &other.0)
        }
    }
    impl Debug for ApproxEq {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl From<ApproxEq> for DontCare {
        fn from(value: ApproxEq) -> Self {
            DontCare(value.0)
        }
    }

    #[rustfmt::skip]
        fn assign_nanmat_assign_rot( ra: f64, rb: f64, rc: f64, rd: f64, rot: f64) -> (
            ApproxEq, ApproxEq, ApproxEq, ApproxEq, ApproxEq, ApproxEq, ApproxEq,
        ) {
            // cache_scale_rotation
            let matrix = Matrix {
                a: ra as f32,
                b: rb as f32,
                c: rc as f32,
                d: rd as f32,
                tx: Twips::default(),
                ty: Twips::default(),
            };

            let (rotation, scale_x, scale_y, skew) = sync_transform_matrix::props_from_matrix(matrix);

            // set_rotation
            let set_degrees = Degrees::from_radians(rot);

            let Matrix { a, b, c, d, .. } =
                sync_transform_matrix::matrix_from_props(set_degrees, scale_x, scale_y, skew);

            (
                ApproxEq(a as f64),
                ApproxEq(b as f64),
                ApproxEq(c as f64),
                ApproxEq(d as f64),
                ApproxEq(scale_x.unit()),
                ApproxEq(scale_y.unit()),
                ApproxEq(rotation.into_radians()),
            )
        }

    #[rustfmt::skip]
        fn assign_nanmat_assign_rot0( ra: f64, rb: f64, rc: f64, rd: f64, ) -> (
            ApproxEq, ApproxEq, ApproxEq, ApproxEq, ApproxEq, ApproxEq, ApproxEq,
        ) {
            assign_nanmat_assign_rot(ra,rb,rc,rd, 0.0)
        }

    #[rustfmt::skip]
        fn assign_rotnan( ra: f64, rb: f64, rc: f64, rd: f64, ) -> (
            ApproxEq, ApproxEq, ApproxEq, ApproxEq, ApproxEq, ApproxEq, ApproxEq,
        ) {
            assign_nanmat_assign_rot(ra,rb,rc,rd, f64::NAN)
        }

    fn assign_nanmat_assign_rot0_prev_rot(a: f64, b: f64, c: f64, d: f64) -> ApproxEq {
        let (_a, _b, _c, _d, _sx, _sy, r) = assign_nanmat_assign_rot0(a, b, c, d);
        r
    }

    #[rustfmt::skip]
        fn assign_rotnan_x( a: f64, b: f64, c: f64, d: f64, ) -> (
            ApproxEq, ApproxEq, DontCare, DontCare, ApproxEq, DontCare, DontCare,
        ) {
            let (a, b, c, d, sx, sy, r) = assign_rotnan(a, b, c, d);
            (a, b, c.into(), d.into(), sx, sy.into(), r.into())
        }

    #[rustfmt::skip]
        fn assign_rotnan_y( a: f64, b: f64, c: f64, d: f64, ) -> (
            DontCare, DontCare, ApproxEq, ApproxEq, DontCare, ApproxEq, DontCare,
        ) {
            let (a, b, c, d, sx, sy, r) = assign_rotnan(a, b, c, d);
            (a.into(), b.into(), c, d, sx.into(), sy, r.into())
        }

    #[rustfmt::skip]
        fn assign_nanmat_assign_rot0_x( a: f64, b: f64, c: f64, d: f64, ) -> (
            ApproxEq, ApproxEq, DontCare, DontCare, ApproxEq, DontCare, DontCare,
        ) {
            let (a, b, c, d, sx, sy, r) = assign_nanmat_assign_rot0(a, b, c, d);
            (a, b, c.into(), d.into(), sx, sy.into(), r.into())
        }

    #[rustfmt::skip]
        fn assign_nanmat_assign_rot0_y( a: f64, b: f64, c: f64, d: f64, ) -> (
            DontCare, DontCare, ApproxEq, ApproxEq, DontCare, ApproxEq, DontCare,
        ) {
            let (a, b, c, d, sx, sy, r) = assign_nanmat_assign_rot0(a, b, c, d);
            (a.into(), b.into(), c, d, sx.into(), sy, r.into())
        }

    #[rustfmt::skip]
        fn approx_care_x( a: f64, b: f64, sx: f64, ) -> (
            ApproxEq, ApproxEq, DontCare, DontCare, ApproxEq, DontCare, DontCare,
        ) {
            (ApproxEq(a), ApproxEq(b), DC, DC, ApproxEq(sx), DC, DC)
        }

    #[rustfmt::skip]
        fn approx_care_y( c: f64, d: f64, sy: f64, ) -> (
            DontCare, DontCare, ApproxEq, ApproxEq, DontCare, ApproxEq, DontCare,
        ) {
            (DC, DC, ApproxEq(c), ApproxEq(d), DC, ApproxEq(sy), DC)
        }

    #[test]
    fn test_all_nan() {
        assert_eq!(
            assign_nanmat_assign_rot0_prev_rot(f64::NAN, f64::NAN, f64::NAN, f64::NAN),
            ApproxEq(f64::NAN),
            "nan, nan, nan, nan prev_rot",
        );
        assert_eq!(
            assign_nanmat_assign_rot0_x(f64::NAN, f64::NAN, f64::NAN, f64::NAN),
            approx_care_x(0.0, 0.0, f64::NAN),
            "nan, nan, nan, nan x",
        );
    }

    #[test]
    fn test_no_skew() {
        // no skew
        assert_eq!(
            assign_nanmat_assign_rot0_y(-1.0, 0.0, 0.0, -1.0),
            approx_care_y(0.0, 1.0, 1.0),
            "-1.0, 0.0, 0.0, -1.0 y",
        );
        assert_eq!(
            assign_nanmat_assign_rot0_y(0.0, 0.0, 0.0, -1.0),
            approx_care_y(-1.0, 0.0, 1.0),
            "0.0, 0.0, 0.0, -1.0 y",
        );
    }

    #[test]
    fn test_assign_nanmat_assign_rot0_prev_rot() {
        assert_eq!(
            assign_nanmat_assign_rot0_prev_rot(0.0, 0.0, 0.0, 0.0),
            ApproxEq(0.0),
            "0.0, 0.0, 0.0, 0.0 prev_rot", // zero
        );
        assert_eq!(
            assign_nanmat_assign_rot0_prev_rot(1.0, 0.0, 0.0, 1.0),
            ApproxEq(0.0),
            "1.0, 0.0, 0.0, 1.0 prev_rot", // id
        );
        assert_eq!(
            assign_nanmat_assign_rot0_prev_rot(0.0, 1.0, 1.0, 0.0),
            ApproxEq(std::f64::consts::PI / 2.0), // 90
            "0.0, 1.0, 1.0, 0.0 prev_rot",        // -id
        );
        assert_eq!(
            assign_nanmat_assign_rot0_prev_rot(1.0, 1.0, 1.0, 1.0),
            ApproxEq(std::f64::consts::PI / 4.0), // 45
            "1.0, 1.0, 1.0, 1.0 prev_rot",        // det=0
        );
        assert_eq!(
            assign_nanmat_assign_rot0_prev_rot(-0.0, -0.0, -0.0, 1.0),
            ApproxEq(0.0),
            "-0.0, -0.0, -0.0, 1.0 prev_rot",
        );
        assert_eq!(
            assign_nanmat_assign_rot0_prev_rot(-0.0, -0.0, -0.0, 0.0),
            ApproxEq(-std::f64::consts::PI), // -180
            "-0.0, -0.0, -0.0, 0.0 prev_rot",
        );
    }

    #[test]
    fn test_assign_nanmat_assign_rot0_x() {
        assert_eq!(
            assign_nanmat_assign_rot0_x(0.0, 0.0, 0.0, 0.0),
            approx_care_x(0.0, 0.0, 0.0),
            "0.0, 0.0, 0.0, 0.0 x", // zero
        );
        assert_eq!(
            assign_nanmat_assign_rot0_x(1.0, 0.0, 0.0, 1.0),
            approx_care_x(1.0, 0.0, 1.0),
            "1.0, 0.0, 0.0, 1.0 x", // id
        );
        assert_eq!(
            assign_nanmat_assign_rot0_x(0.0, 1.0, 1.0, 0.0),
            approx_care_x(1.0, 0.0, 1.0),
            "0.0, 1.0, 1.0, 0.0 x", // -id
        );
        assert_eq!(
            assign_nanmat_assign_rot0_x(1.0, 1.0, 1.0, 1.0),
            approx_care_x(f64::sqrt(2.0), 0.0, f64::sqrt(2.0)),
            "1.0, 1.0, 1.0, 1.0 x", // det=0
        );
        assert_eq!(
            assign_nanmat_assign_rot0_x(1.0, 1.0, f64::NAN, 1.0),
            approx_care_x(f64::sqrt(2.0), 0.0, f64::sqrt(2.0)),
            "1.0, 1.0, nan, 1.0 x",
        );
        assert_eq!(
            assign_nanmat_assign_rot0_x(1.0, 1.0, 1.0, f64::NAN),
            approx_care_x(f64::sqrt(2.0), 0.0, f64::sqrt(2.0)),
            "1.0, 1.0, 1.0, nan x",
        );
        assert_eq!(
            assign_nanmat_assign_rot0_x(1.0, 1.0, f64::NAN, f64::NAN),
            approx_care_x(f64::sqrt(2.0), 0.0, f64::sqrt(2.0)),
            "1.0, 1.0, nan, nan x",
        );
    }

    #[test]
    fn test_assign_nanmat_assign_rot0_y() {
        assert_eq!(
            assign_nanmat_assign_rot0_y(0.0, 0.0, 0.0, 0.0),
            approx_care_y(0.0, 0.0, 0.0),
            "0.0, 0.0, 0.0, 0.0 y", // zero
        );
        assert_eq!(
            assign_nanmat_assign_rot0_y(1.0, 0.0, 0.0, 1.0),
            approx_care_y(0.0, 1.0, 1.0),
            "1.0, 0.0, 0.0, 1.0 y", // id
        );
        assert_eq!(
            assign_nanmat_assign_rot0_y(0.0, 1.0, 1.0, 0.0),
            approx_care_y(0.0, -1.0, -1.0),
            "0.0, 1.0, 1.0, 0.0 y", // -id
        );
        assert_eq!(
            assign_nanmat_assign_rot0_y(1.0, 1.0, 1.0, 1.0),
            approx_care_y(f64::sqrt(2.0), 0.0, f64::sqrt(2.0)),
            "1.0, 1.0, 1.0, 1.0 y", // det=0
        );
        assert_eq!(
            assign_nanmat_assign_rot0_y(1.0, f64::NAN, 1.0, 1.0),
            approx_care_y(-1.0, 1.0, f64::sqrt(2.0)),
            "1.0, nan, 1.0, 1.0 y",
        );
        assert_eq!(
            assign_nanmat_assign_rot0_y(f64::NAN, 1.0, 1.0, 1.0),
            approx_care_y(-1.0, -1.0, f64::sqrt(2.0)),
            "nan, 1.0, 1.0, 1.0 y",
        );
        assert_eq!(
            assign_nanmat_assign_rot0_y(f64::NAN, f64::NAN, 1.0, 1.0),
            approx_care_y(-f64::sqrt(2.0), 0.0, f64::sqrt(2.0)),
            "nan, nan, 1.0, 1.0 y",
        );
    }

    #[test]
    fn test_assign_rotnan() {
        assert_eq!(
            assign_rotnan_x(1.0, 1.0, 1.0, 1.0),
            approx_care_x(f64::sqrt(2.0), 0.0, f64::sqrt(2.0)),
            "1.0, 1.0, 1.0, 1.0 x",
        );
        assert_eq!(
            assign_rotnan_y(1.0, 1.0, 1.0, 1.0),
            approx_care_y(f64::sqrt(2.0), 0.0, f64::sqrt(2.0)),
            "1.0, 1.0, 1.0, 1.0 y",
        );
        assert_eq!(
            assign_rotnan_x(f64::NAN, f64::NAN, f64::NAN, f64::NAN),
            approx_care_x(0.0, 0.0, f64::NAN),
            "nan, nan, nan, nan x",
        );
        assert_eq!(
            assign_rotnan_y(f64::NAN, f64::NAN, f64::NAN, f64::NAN),
            approx_care_y(0.0, 0.0, f64::NAN),
            "nan, nan, nan, nan y",
        );
        assert_eq!(
            assign_rotnan_x(2.0, 2.0, 2.0, 2.0),
            approx_care_x(f64::sqrt(8.0), 0.0, f64::sqrt(8.0)),
            "2.0, 2.0, 2.0, 2.0 x",
        );
        assert_eq!(
            assign_rotnan_x(-1.0, 0.0, -1.0, 0.0),
            approx_care_x(1.0, 0.0, 1.0),
            "-1.0, 0.0, -1.0, 0.0 x",
        );
    }
}

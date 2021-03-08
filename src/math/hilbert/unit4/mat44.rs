pub mod display;
pub mod op_linear;
pub mod op_index;
pub mod transform;
pub use self::display::*;
pub use self::op_linear::*;
pub use self::op_index::*;
pub use self::transform::*;
use crate::math::number::*;


#[derive(Clone, Copy)]
pub struct Mat44 {
    pub(crate) content: [Complex; 16],
}

impl Mat44 {
    #[inline]
    pub fn zeros() -> Self {
        Self { content: [Complex::zero(); 16] }
    }

    #[inline]
    pub fn new(m: [[Complex; 4]; 4]) -> Self {
        Self {
            content: [
                m[0][0], m[0][1], m[0][2], m[0][3],
                m[1][0], m[1][1], m[1][2], m[1][3],
                m[2][0], m[2][1], m[2][2], m[2][3],
                m[3][0], m[3][1], m[3][2], m[3][3],
            ],
        }
    }

    #[inline]
    pub fn eye() -> Self {
        Self {
            content: [
                1_f64.as_z(),   0.0.as_z(),   0.0.as_z(),   0.0.as_z(),
                  0.0.as_z(), 1_f64.as_z(),   0.0.as_z(),   0.0.as_z(),
                  0.0.as_z(),   0.0.as_z(), 1_f64.as_z(),   0.0.as_z(),
                  0.0.as_z(),   0.0.as_z(),   0.0.as_z(), 1_f64.as_z(),
            ]
        }
    }
}
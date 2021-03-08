use std::ops::Mul;
use crate::math::number::*;
use super::*;


impl Mul<Vec4> for Vec4 {
    type Output = Complex;

    #[inline]
    fn mul(self, rhs: Vec4) -> Self::Output {
        debug_assert!(
            self.basis == Bra && rhs.basis == Ket,
            "Operation Vec4 * Vec4 error: must be < bra | ket >!"
        );

        self[0] * rhs[0]
        + self[1] * rhs[1]
        + self[2] * rhs[2]
        + self[3] * rhs[3]
    }
}

impl Mul<&Vec4> for Vec4 {
    type Output = Complex;

    #[inline]
    fn mul(self, rhs: &Vec4) -> Self::Output {
        self * *rhs
    }
}

impl Mul<Vec4> for &Vec4 {
    type Output = Complex;

    #[inline]
    fn mul(self, rhs: Vec4) -> Self::Output {
        *self * rhs
    }
}

impl<'a, 'b> Mul<&'b Vec4> for &'a Vec4 {
    type Output = Complex;

    #[inline]
    fn mul(self, rhs: &'b Vec4) -> Self::Output {
        *self * *rhs
    }
}
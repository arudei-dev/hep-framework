use unroll::unroll_for_loops;
use std::ops::Mul;
use super::mat44::Mat44;
use super::vec4::Vec4;
use crate::math::hilbert::Braket::{Bra, Ket};
use crate::math::number::*;


impl Mul<Mat44> for Vec4 {
    type Output = Vec4;

    #[inline]
    #[unroll_for_loops]
    fn mul(self, rhs: Mat44) -> Self::Output {
        debug_assert!(self.basis == Bra, "Operation Vec4 * Mat44 error: Vec4 is not a Bra!");

        let mut new_content = [Complex::zero(); 4];

        for i in 0..4 {
            new_content[i] =   (self[0] * rhs[(0, i)])
                             + (self[1] * rhs[(1, i)])
                             + (self[2] * rhs[(2, i)])
                             + (self[3] * rhs[(3, i)]);
        }

        Self::Output {
            content: new_content,
            basis: Bra,
        }
    }
}

impl Mul<&Mat44> for Vec4 {
    type Output = Vec4;

    #[inline]
    fn mul(self, rhs: &Mat44) -> Self::Output {
        self.mul(*rhs)
    }
}

impl Mul<Mat44> for &Vec4 {
    type Output = Vec4;

    #[inline]
    fn mul(self, rhs: Mat44) -> Self::Output {
        (*self).mul(rhs)
    }
}

impl<'a, 'b> Mul<&'b Mat44> for &'a Vec4 {
    type Output = Vec4;

    #[inline]
    fn mul(self, rhs: &Mat44) -> Self::Output {
        (*self).mul(*rhs)
    }
}




impl Mul<Vec4> for Mat44 {
    type Output = Vec4;

    #[inline]
    #[unroll_for_loops]
    fn mul(self, rhs: Vec4) -> Self::Output {
        debug_assert!(rhs.basis == Ket, "Operation Mat44 * Vec4 error: Vec4 is not a Ket!");

        let mut new_content = [Complex::zero(); 4];

        for i in 0..4 {
            new_content[i] = 
                  self[(i, 0)] * rhs[0]
                + self[(i, 1)] * rhs[1]
                + self[(i, 2)] * rhs[2]
                + self[(i, 3)] * rhs[3];
        }

        Self::Output {
            content: new_content,
            basis: Ket
        }
    }
}

impl Mul<&Vec4> for Mat44 {
    type Output = Vec4;

    fn mul(self, rhs: &Vec4) -> Self::Output {
        self * (*rhs)
    }
}

impl Mul<Vec4> for &Mat44 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        (*self) * rhs
    }
}

impl<'a, 'b> Mul<&'b Vec4> for &'a Mat44 {
    type Output = Vec4;

    fn mul(self, rhs: &Vec4) -> Self::Output {
        (*self) * (*rhs)
    }
}
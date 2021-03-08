use std::ops::{Add, Sub, Mul};
use unroll::unroll_for_loops;
use crate::math::hilbert::mat44::*;
use crate::math::number::*;


impl Add for Mat44 {
    type Output = Mat44;

    #[unroll_for_loops]
    fn add(self, rhs: Mat44) -> Self::Output {
        let mut content = [Complex::zero(); 16];

        for i in 0..16 {
            content[i] = self.content[i] + rhs.content[i];
        }

        Self::Output { content }
    }
}

impl Sub for Mat44 {
    type Output = Mat44;

    #[unroll_for_loops]
    fn sub(self, rhs: Mat44) -> Self::Output {
        #[cfg(feature="simd_calc")] {
            todo!("SIMD not implemented yet!")
        }
        #[cfg(not(feature="simd_calc"))] {
            let mut content = [Complex::zero(); 16];

            for i in 0..16 {
                content[i] = self.content[i] - rhs.content[i];
            }

            Self::Output { content }
        }
    }
}

impl Mul for Mat44 {
    type Output = Mat44;

    #[inline]
    #[unroll_for_loops]
    fn mul(self, rhs: Mat44) -> Self::Output {
        let mut product = Mat44::zeros();

        for i in 0..4 {
            for j in 0..4 {
                product[(i, j)] =   self[(i, 0)] * rhs[(0, j)]
                                  + self[(i, 1)] * rhs[(1, j)]
                                  + self[(i, 2)] * rhs[(2, j)]
                                  + self[(i, 3)] * rhs[(3, j)];
            }
        }
        product
    }
}

macro_rules! fwd_binop_valref {
    (impl $imp:ident, $method:ident) => {
        impl $imp<&Mat44> for Mat44 {
            type Output = Mat44;

            #[inline]
            fn $method(self, rhs: &Mat44) -> Self::Output {
                self.$method(*rhs)
            }
        }
    };
}

macro_rules! fwd_binop_refval {
    (impl $imp:ident, $method:ident) => {
        impl $imp<Mat44> for &Mat44 {
            type Output = Mat44;

            #[inline]
            fn $method(self, rhs: Mat44) -> Self::Output {
                (*self).$method(rhs)
            }
        }
    };
}

macro_rules! fwd_binop_refref {
    (impl $imp:ident, $method:ident) => {
        impl<'a, 'b> $imp<&'b Mat44> for &'a Mat44 {
            type Output = Mat44;

            #[inline]
            fn $method(self, rhs: &'b Mat44) -> Self::Output {
                (*self).$method(*rhs)
            }
        }
    };
}

macro_rules! fwd_binop {
    (impl $imp:ident, $method:ident) => {
        fwd_binop_valref!(impl $imp, $method);
        fwd_binop_refval!(impl $imp, $method);
        fwd_binop_refref!(impl $imp, $method);
    };
}

fwd_binop!(impl Add, add);
fwd_binop!(impl Sub, sub);
fwd_binop!(impl Mul, mul);
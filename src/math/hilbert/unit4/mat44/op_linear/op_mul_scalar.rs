use std::ops::{Mul};
use unroll::unroll_for_loops;
use crate::math::hilbert::mat44::*;
use crate::math::number::*;

/////////////////////////////////////////////////
/////////////////////////////////////////////////
////
////  (Mat44 * T, T * Mat44) ... <T>
////  Mat44 multiplication Arithmetics Base
////
/////////////////////////////////////////////////
/////////////////////////////////////////////////

macro_rules! gen_mul_t {
    (T=$id:ident) => {
        impl Mul<$id> for Mat44
        {
            type Output = Mat44;

            #[inline]
            #[unroll_for_loops]
            fn mul(self, rhs: $id) -> Self::Output {
                let mut prod = Mat44::zeros();

                for i in 0..16 {
                    prod.content[i] = self.content[i] * rhs;
                }

                prod
            }
        }

        impl Mul<Mat44> for $id
        {
            type Output = Mat44;

            #[inline]
            #[unroll_for_loops]
            fn mul(self, rhs: Mat44) -> Self::Output {
                let mut prod = Mat44::zeros();

                for i in 0..16 {
                    prod.content[i] = self * rhs.content[i];
                }

                prod
            }
        }
    };
}

/////////////////////////////////////////////////
/////////////////////////////////////////////////
////
////  %T * %Mat44 and %Mat44 * %T ... <T>
////  where @ = (+, -, *, /) Operators
////        % = (&&, &v, v&) Ref / Val -type
////
/////////////////////////////////////////////////
/////////////////////////////////////////////////

macro_rules! fwd_mul_valref {
    (T=$id:ident) => {
        impl Mul<&$id> for Mat44
        {
            type Output = Mat44;

            #[inline]
            fn mul(self, rhs: &$id) -> Self::Output {
                self.mul(*rhs)
            }
        }

        impl Mul<&Mat44> for $id
        {
            type Output = Mat44;

            #[inline]
            fn mul(self, rhs: &Mat44) -> Self::Output {
                self.mul(*rhs)
            }
        }
    };
}

macro_rules! fwd_mul_refval {
    (T=$id:ident) => {
        impl Mul<$id> for &Mat44
        {
            type Output = Mat44;

            #[inline]
            fn mul(self, rhs: $id) -> Self::Output {
                (*self).mul(rhs)
            }
        }

        impl Mul<Mat44> for &$id
        {
            type Output = Mat44;

            #[inline]
            fn mul(self, rhs: Mat44) -> Self::Output {
                (*self).mul(rhs)
            }
        }
    };
}

macro_rules! fwd_mul_refref {
    (T=$id:ident) => {
        impl Mul<&$id> for &Mat44
        {
            type Output = Mat44;

            #[inline]
            fn mul(self, rhs: &$id) -> Self::Output {
                (*self).mul(*rhs)
            }
        }

        impl Mul<&Mat44> for &$id
        {
            type Output = Mat44;

            #[inline]
            fn mul(self, rhs: &Mat44) -> Self::Output {
                (*self).mul(*rhs)
            }
        }
    };
}

macro_rules! genfwd_mul {
    (T=$id:ident) => {
        gen_mul_t!(T=$id);
        fwd_mul_valref!(T=$id);
        fwd_mul_refval!(T=$id);
        fwd_mul_refref!(T=$id);
    };
}

/////////////////////////////////////////////////
/////////////////////////////////////////////////
////
////  Generation for supported scalar type
////  T = { Real, Complex }
////
/////////////////////////////////////////////////
/////////////////////////////////////////////////

genfwd_mul!(T=Real);
genfwd_mul!(T=Complex);
use std::ops::Mul;
use crate::math::number::{Number, Real, complex::*};
use super::*;


/////////////////////////////////////////////////
/////////////////////////////////////////////////
////
////  (V * s, s * V)
////  Lorentz4 Multiplication Base
////    where V := Lorentz4,  s := Scalar
////
/////////////////////////////////////////////////
/////////////////////////////////////////////////

pub mod gen_std {
    use super::*;

    impl<T> Mul<T> for Lorentz4<T> 
    where
        T: Number + Clone + Copy + Mul<Output=T>
    {
        type Output = Lorentz4<T>;

        #[inline]
        fn mul(self, rhs: T) -> Self::Output {
            Self::Output {
                content: [
                    self[0] * rhs,
                    self[1] * rhs,
                    self[2] * rhs,
                    self[3] * rhs,
                ],
                basis: self.basis,
            }
        }
    }

    impl Mul<Real> for Lorentz4<Complex> {
        type Output = Lorentz4<Complex>;

        #[inline]
        fn mul(self, rhs: Real) -> Self::Output {
            self * (rhs.as_z())
        }
    }

    impl Mul<Complex> for Lorentz4<Real> {
        type Output = Lorentz4<Complex>;

        #[inline]
        fn mul(self, rhs: Complex) -> Self::Output {
            self.to_z() * rhs
        }
    }



    macro_rules! gen_scalar_mul_v {
        (lhs=$lhs:ident, rhs=$rhs:ident, out=$out:ident) => {
            impl Mul<Lorentz4<$rhs>> for $lhs {
                type Output = Lorentz4<$out>;
        
                #[inline]
                fn mul(self, rhs: Lorentz4<$rhs>) -> Self::Output {
                    rhs * self
                }
            }
        };
    }

    gen_scalar_mul_v!(lhs=Real,    rhs=Real,    out=Real);
    gen_scalar_mul_v!(lhs=Complex, rhs=Complex, out=Complex);
    gen_scalar_mul_v!(lhs=Real,    rhs=Complex, out=Complex);
    gen_scalar_mul_v!(lhs=Complex, rhs=Real,    out=Complex);
}
pub use gen_std::*;

/////////////////////////////////////////////////
/////////////////////////////////////////////////
////
////  (&&, &V, V&) for Mul(V * s)
////  Operator forwarding
////
/////////////////////////////////////////////////
/////////////////////////////////////////////////

pub mod fwd_op_vs {
    use super::*;

    macro_rules! fwd_mul_vs_valref {
        (lhs=$lhs:ident, rhs=$rhs:ident, out=$out:ident) => {
            impl Mul<&$rhs> for Lorentz4<$lhs>
            {
                type Output = Lorentz4<$out>;

                #[inline]
                fn mul(self, rhs: &$rhs) -> Self::Output {
                    self.mul(*rhs)
                }
            }
        };
    }

    macro_rules! fwd_mul_vs_refval {
        (lhs=$lhs:ident, rhs=$rhs:ident, out=$out:ident) => {
            impl Mul<$rhs> for &Lorentz4<$lhs> {
                type Output = Lorentz4<$out>;

                #[inline]
                fn mul(self, rhs: $rhs) -> Self::Output {
                    (*self).mul(rhs)
                }
            }
        };
    }

    macro_rules! fwd_mul_vs_refref {
        (lhs=$lhs:ident, rhs=$rhs:ident, out=$out:ident) => {
            impl<'a, 'b> Mul<&'b $rhs> for &'a Lorentz4<$lhs> {
                type Output = Lorentz4<$out>;

                #[inline]
                fn mul(self, rhs: &'b $rhs) -> Self::Output {
                    (*self).mul(*rhs)
                }
            }
        };
    }

    macro_rules! fwd_mul_vs {
        (lhs=$lhs:ident, rhs=$rhs:ident, out=$out:ident) => {
            fwd_mul_vs_valref!(lhs=$lhs, rhs=$rhs, out=$out);
            fwd_mul_vs_refval!(lhs=$lhs, rhs=$rhs, out=$out);
            fwd_mul_vs_refref!(lhs=$lhs, rhs=$rhs, out=$out);
        };
    }

    fwd_mul_vs!(lhs=Real,    rhs=Real,    out=Real);
    fwd_mul_vs!(lhs=Complex, rhs=Complex, out=Complex);
    fwd_mul_vs!(lhs=Real,    rhs=Complex, out=Complex);
    fwd_mul_vs!(lhs=Complex, rhs=Real,    out=Complex);
}
pub use fwd_op_vs::*;


/////////////////////////////////////////////////
/////////////////////////////////////////////////
////
////  (&&, &V, V&) for Mul(s * V)
////  Operator forwarding
////
/////////////////////////////////////////////////
/////////////////////////////////////////////////

pub mod fwd_op_sv {
    use super::*;

    macro_rules! fwd_mul_sv_valref {
        (lhs=$lhs:ident, rhs=$rhs:ident, out=$out:ident) => {
            impl Mul<&Lorentz4<$rhs>> for $lhs
            {
                type Output = Lorentz4<$out>;

                #[inline]
                fn mul(self, rhs: &Lorentz4<$rhs>) -> Self::Output {
                    *rhs * self
                }
            }
        };
    }

    macro_rules! fwd_mul_sv_refval {
        (lhs=$lhs:ident, rhs=$rhs:ident, out=$out:ident) => {
            impl Mul<Lorentz4<$rhs>> for &$lhs {
                type Output = Lorentz4<$out>;

                #[inline]
                fn mul(self, rhs: Lorentz4<$rhs>) -> Self::Output {
                    rhs * *self
                }
            }
        };
    }

    macro_rules! fwd_mul_sv_refref {
        (lhs=$lhs:ident, rhs=$rhs:ident, out=$out:ident) => {
            impl<'a, 'b> Mul<&'b Lorentz4<$rhs>> for &'a $lhs {
                type Output = Lorentz4<$out>;

                #[inline]
                fn mul(self, rhs: &'b Lorentz4<$rhs>) -> Self::Output {
                    *rhs * *self
                }
            }
        };
    }

    macro_rules! fwd_mul_sv {
        (lhs=$lhs:ident, rhs=$rhs:ident, out=$out:ident) => {
            fwd_mul_sv_valref!(lhs=$lhs, rhs=$rhs, out=$out);
            fwd_mul_sv_refval!(lhs=$lhs, rhs=$rhs, out=$out);
            fwd_mul_sv_refref!(lhs=$lhs, rhs=$rhs, out=$out);
        };
    }

    fwd_mul_sv!(lhs=Real,    rhs=Real,    out=Real);
    fwd_mul_sv!(lhs=Complex, rhs=Complex, out=Complex);
    fwd_mul_sv!(lhs=Real,    rhs=Complex, out=Complex);
    fwd_mul_sv!(lhs=Complex, rhs=Real,    out=Complex);
}
pub use fwd_op_sv::*;
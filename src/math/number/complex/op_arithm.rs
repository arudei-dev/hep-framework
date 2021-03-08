use std::ops::{Add, Sub, Mul, Div};
use crate::math::number::complex::*;
use crate::math::number::Real;

/////////////////////////////////////////////////
/////////////////////////////////////////////////
////
////  (+, -, *, /)
////  Complex Arithmetics Base
////
/////////////////////////////////////////////////
/////////////////////////////////////////////////

pub mod base {
    use super::*;

    impl Add for Complex {
        type Output = Complex;

        #[inline]
        fn add(self, rhs: Complex) -> Self::Output {
            Self {
                real: self.real + rhs.real,
                imag: self.imag + rhs.imag,
            }
        }
    }

    impl Sub for Complex {
        type Output = Complex;

        #[inline]
        fn sub(self, rhs: Complex) -> Self::Output {
            Self {
                real: self.real - rhs.real,
                imag: self.imag - rhs.imag,
            }
        }
    }

    impl Mul for Complex {
        type Output = Complex;

        #[inline]
        fn mul(self, rhs: Complex) -> Self::Output {
            Self {
                real: (self.real * rhs.real) - (self.imag * rhs.imag),
                imag: (self.real * rhs.imag) + (self.imag * rhs.real),
            }
        }
    }

    impl Div for Complex {
        type Output = Complex;

        #[inline]
        fn div(self, rhs: Complex) -> Self::Output {
            let denom = rhs.norm_sq();

            Self {
                real: ((self.real * rhs.real) + (self.imag * rhs.imag)) / denom,
                imag: ((self.imag * rhs.real) - (self.real * rhs.imag)) / denom,
            }
        }
    }
}
pub use base::*;

/////////////////////////////////////////////////
/////////////////////////////////////////////////
////
////  R @ C, C @ R (+, -, *, /) Ops generator
////  where R := Real, C := Complex
////
/////////////////////////////////////////////////
/////////////////////////////////////////////////

pub mod gen_het {
    use super::*;

    macro_rules! gen_binop_rc {
        (impl $imp:ident, $method:ident) => {
            impl $imp<Real> for Complex {
                type Output = Complex;

                #[inline]
                fn $method(self, rhs: Real) -> Self::Output {
                    self.$method(Complex::from_real(rhs))
                }
            }
            impl $imp<Complex> for Real {
                type Output = Complex;

                #[inline]
                fn $method(self, rhs: Complex) -> Self::Output {
                    Complex::from_real(self).$method(rhs)
                }
            }
        };
    }

    gen_binop_rc!(impl Add, add);
    gen_binop_rc!(impl Sub, sub);
    gen_binop_rc!(impl Mul, mul);
    gen_binop_rc!(impl Div, div);
}
pub use gen_het::*;

/////////////////////////////////////////////////
/////////////////////////////////////////////////
////
////  (&&, &v, v&)
////  C @ C, R @ C, C @ R (+, -, *, /) Ops forwarding
////  where R := Real, C := Complex
////
/////////////////////////////////////////////////
/////////////////////////////////////////////////

pub mod fwd_rule {
    use super::*;
    macro_rules! fwd_arithm_val_ref {
        (impl $imp:ident, $method:ident, lhs=$lhs:ident, rhs=$rhs:ident, out=$out:ident) => {
            impl $imp<&$rhs> for $lhs {
                type Output = $out;

                #[inline]
                fn $method(self, rhs: &$rhs) -> Self::Output {
                    self.$method(*rhs)
                }
            }
        };
    }

    macro_rules! fwd_arithm_ref_val {
        (impl $imp:ident, $method:ident, lhs=$lhs:ident, rhs=$rhs:ident, out=$out:ident) => {
            impl $imp<$rhs> for &$lhs {
                type Output = $out;

                #[inline]
                fn $method(self, rhs: $rhs) -> Self::Output {
                    (*self).$method(rhs)
                }
            }
        };
    }

    macro_rules! fwd_arithm_ref_ref {
        (impl $imp:ident, $method:ident, lhs=$lhs:ident, rhs=$rhs:ident, out=$out:ident) => {
            impl<'a, 'b> $imp<&'b $rhs> for &'a $lhs {
                type Output = $out;

                #[inline]
                fn $method(self, rhs: &'b $rhs) -> Self::Output {
                    (*self).$method(*rhs)
                }
            }
        };
    }

    macro_rules! fwd_arithm {
        (impl $imp:ident, $method:ident) => {
            fwd_arithm_val_ref!(impl $imp, $method, lhs=Complex, rhs=Complex, out=Complex);
            fwd_arithm_val_ref!(impl $imp, $method, lhs=Real,    rhs=Complex, out=Complex);
            fwd_arithm_val_ref!(impl $imp, $method, lhs=Complex, rhs=Real,    out=Complex);

            fwd_arithm_ref_val!(impl $imp, $method, lhs=Complex, rhs=Complex, out=Complex);
            fwd_arithm_ref_val!(impl $imp, $method, lhs=Real,    rhs=Complex, out=Complex);
            fwd_arithm_ref_val!(impl $imp, $method, lhs=Complex, rhs=Real,    out=Complex);

            fwd_arithm_ref_ref!(impl $imp, $method, lhs=Complex, rhs=Complex, out=Complex);
            fwd_arithm_ref_ref!(impl $imp, $method, lhs=Real,    rhs=Complex, out=Complex);
            fwd_arithm_ref_ref!(impl $imp, $method, lhs=Complex, rhs=Real,    out=Complex);
        };
    }

    fwd_arithm!(impl Add, add);
    fwd_arithm!(impl Sub, sub);
    fwd_arithm!(impl Mul, mul);
    fwd_arithm!(impl Div, div);
}
pub use fwd_rule::*;
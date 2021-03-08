use std::ops::{Add, Sub};
use crate::math::number::{Number, Real, Complex};
use super::*;


/////////////////////////////////////////////////
/////////////////////////////////////////////////
////
////  Lorentz4 Arithmetic (+, -) Base
////
/////////////////////////////////////////////////
/////////////////////////////////////////////////

pub mod gen_std {
    use super::*;

    impl<T> Add for Lorentz4<T> 
    where
        T: Number + Clone + Copy + Add<Output=T>
    {
        type Output = Lorentz4<T>;

        #[inline]
        fn add(self, rhs: Lorentz4<T>) -> Self::Output {
            debug_assert!(
                self.basis == rhs.basis, 
                "Lorentz4 addition error: Basis mismatch!"
            );

            Self::Output {
                content: [
                    self[0] + rhs[0],
                    self[1] + rhs[1],
                    self[2] + rhs[2],
                    self[3] + rhs[3],
                ],
                basis: self.basis
            }
        }
    }

    impl<T> Sub for Lorentz4<T> 
    where
        T: Number + Clone + Copy + Sub<Output=T>
    {
        type Output = Lorentz4<T>;

        #[inline]
        fn sub(self, rhs: Lorentz4<T>) -> Self::Output {
            debug_assert!(
                self.basis == rhs.basis, 
                "Lorentz4 addition error: Basis mismatch!"
            );

            Self::Output {
                content: [
                    self[0] - rhs[0],
                    self[1] - rhs[1],
                    self[2] - rhs[2],
                    self[3] - rhs[3],
                ],
                basis: self.basis
            }
        }
    }

}
pub use gen_std::*;

/////////////////////////////////////////////////
/////////////////////////////////////////////////
////
////  (+, -) for V<permute(Real, Complex)>
////  Lorentz4 Arithmetics Base
////
/////////////////////////////////////////////////
/////////////////////////////////////////////////

pub mod gen_het {
    use super::*;

    macro_rules! gen_het_pm {
        (impl $imp:ident, $method:ident) => {
            impl $imp<Lorentz4<Real>> for Lorentz4<Complex> {
                type Output = Lorentz4<Complex>;

                #[inline]
                fn $method(self, rhs: Lorentz4<Real>) -> Self::Output {
                    self.$method(rhs.to_z())
                }
            } 
            impl $imp<Lorentz4<Complex>> for Lorentz4<Real> {
                type Output = Lorentz4<Complex>;

                #[inline]
                fn $method(self, rhs: Lorentz4<Complex>) -> Self::Output {
                    (self.to_z()).$method(rhs)
                }
            } 
        };
    }

    gen_het_pm!(impl Add, add);
    gen_het_pm!(impl Sub, sub);


}
pub use gen_het::*;

/////////////////////////////////////////////////
/////////////////////////////////////////////////
////
////  (&&, &V, V&) 
////  Operator forwarding
////
/////////////////////////////////////////////////
/////////////////////////////////////////////////

pub mod fwd_rule {
    use super::*;

    macro_rules! fwd_het_binop_valref {
        (impl $imp:ident, $method:ident, lhs=$lhs:ident, rhs=$rhs:ident, out=$out:ident) => {
            impl $imp<&Lorentz4<$rhs>> for Lorentz4<$lhs> {
                type Output = Lorentz4<$out>;

                fn $method(self, rhs: &Lorentz4<$rhs>) -> Self::Output {
                    self.$method(*rhs)
                }
            } 
        };
    }

    macro_rules! fwd_het_binop_refval {
        (impl $imp:ident, $method:ident, lhs=$lhs:ident, rhs=$rhs:ident, out=$out:ident) => {
            impl $imp<Lorentz4<$rhs>> for &Lorentz4<$lhs> {
                type Output = Lorentz4<$out>;

                fn $method(self, rhs: Lorentz4<$rhs>) -> Self::Output {
                    (*self).$method(rhs)
                }
            } 
        };
    }

    macro_rules! fwd_het_binop_refref {
        (impl $imp:ident, $method:ident, lhs=$lhs:ident, rhs=$rhs:ident, out=$out:ident) => {
            impl<'a, 'b> $imp<&'b Lorentz4<$rhs>> for &'a Lorentz4<$lhs> {
                type Output = Lorentz4<$out>;

                fn $method(self, rhs: &'b Lorentz4<$rhs>) -> Self::Output {
                    (*self).$method(*rhs)
                }
            } 
        };
    }

    macro_rules! fwd_het_binop {
        (impl $imp:ident, $method:ident) => {
            fwd_het_binop_valref!(impl $imp, $method, lhs=Real,    rhs=Real,    out=Real);
            fwd_het_binop_valref!(impl $imp, $method, lhs=Complex, rhs=Complex, out=Complex);
            fwd_het_binop_valref!(impl $imp, $method, lhs=Real,    rhs=Complex, out=Complex);
            fwd_het_binop_valref!(impl $imp, $method, lhs=Complex, rhs=Real,    out=Complex);

            fwd_het_binop_refval!(impl $imp, $method, lhs=Real,    rhs=Real,    out=Real);
            fwd_het_binop_refval!(impl $imp, $method, lhs=Complex, rhs=Complex, out=Complex);
            fwd_het_binop_refval!(impl $imp, $method, lhs=Real,    rhs=Complex, out=Complex);
            fwd_het_binop_refval!(impl $imp, $method, lhs=Complex, rhs=Real,    out=Complex);
            
            fwd_het_binop_refref!(impl $imp, $method, lhs=Real,    rhs=Real,    out=Real);
            fwd_het_binop_refref!(impl $imp, $method, lhs=Complex, rhs=Complex, out=Complex);
            fwd_het_binop_refref!(impl $imp, $method, lhs=Real,    rhs=Complex, out=Complex);
            fwd_het_binop_refref!(impl $imp, $method, lhs=Complex, rhs=Real,    out=Complex);
            
        };
    }

    fwd_het_binop!(impl Add, add);
    fwd_het_binop!(impl Sub, sub);
}
pub use fwd_rule::*;
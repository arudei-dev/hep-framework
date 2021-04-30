use std::ops::{Add, Sub, Mul};
use crate::math::number::*;
use super::*;


impl<T> Lorentz4<T>
where
    T: Number + Clone + Copy + Add<Output=T> + Sub<Output=T> + Mul<Output=T>
{
    #[inline]
    pub fn norm_sq(&self) -> T {
        self[0] * self[0]
        - self[1] * self[1]
        - self[2] * self[2]
        - self[3] * self[3]
    }
}

/////////////////////////////////////////////////
/////////////////////////////////////////////////

pub trait InnerProduct<Rhs = Self> {
    type Output;

    fn dot(self, rhs: Rhs) -> Self::Output;
}

/////////////////////////////////////////////////
/////////////////////////////////////////////////
////
////  V<T, B1>.dot(V<T, B2>)
////  Lorentz4 Inner Product (Homogen)
////    where V := Lorentz4,  
////     B1, B2 := Basis
////
/////////////////////////////////////////////////
/////////////////////////////////////////////////

pub mod gen_inner_hom {
    use super::*;

    impl<T> InnerProduct<Lorentz4<T>> for Lorentz4<T> 
    where
        T: Number + Clone + Copy + Add<Output=T> + Sub<Output=T> + Mul<Output=T>
    {
        type Output = T;

        #[inline]
        fn dot(self, rhs: Lorentz4<T>) -> Self::Output {
            if self.basis == rhs.basis {
                self[0] * rhs[0]
                - self[1] * rhs[1]
                - self[2] * rhs[2]
                - self[3] * rhs[3]
            }
            else {
                self[0] * rhs[0]
                + self[1] * rhs[1]
                + self[2] * rhs[2]
                + self[3] * rhs[3]
            }
        }
    }
}
pub use gen_inner_hom::*;

/////////////////////////////////////////////////
/////////////////////////////////////////////////
////
////  V<T, B1>.dot(V<T, B2>)
////  Lorentz4 Inner Product (Heterogen R, C)
////    where V := Lorentz4,  
////     B1, B2 := Basis
////
/////////////////////////////////////////////////
/////////////////////////////////////////////////

pub mod gen_inner_het {
    use super::*;

    impl InnerProduct<Lorentz4<Real>> for Lorentz4<Complex> {
        type Output = Complex;

        #[inline]
        fn dot(self, rhs: Lorentz4<Real>) -> Self::Output {
            self.dot(rhs.to_z())
        }
    }

    impl InnerProduct<Lorentz4<Complex>> for Lorentz4<Real> {
        type Output = Complex;

        #[inline]
        fn dot(self, rhs: Lorentz4<Complex>) -> Self::Output {
            (self.to_z()).dot(rhs)
        }
    }
}
pub use gen_inner_het::*;

/////////////////////////////////////////////////
/////////////////////////////////////////////////
////
////  &&, &v, v& Forwarding
////
/////////////////////////////////////////////////
/////////////////////////////////////////////////

pub mod fwd_ops {
    use super::*;
    
    // V.dot(&V)
    macro_rules! fwd_inner_valref {
        (lhs=$lhs:ident, rhs=$rhs:ident, out=$out:ident) => {   
            impl InnerProduct<&Lorentz4<$rhs>> for Lorentz4<$lhs> 
            {
                type Output = $out;

                #[inline]
                fn dot(self, rhs: &Lorentz4<$rhs>) -> Self::Output {
                    self.dot(*rhs)
                }
            }
        };
    }

    // (&V).dot(V)
    macro_rules! fwd_inner_refval {
        (lhs=$lhs:ident, rhs=$rhs:ident, out=$out:ident) => {   
            impl InnerProduct<Lorentz4<$rhs>> for &Lorentz4<$lhs> 
            {
                type Output = $out;

                #[inline]
                fn dot(self, rhs: Lorentz4<$rhs>) -> Self::Output {
                    (*self).dot(rhs)
                }
            }
        };
    }

    // (&V).dot(&V)
    macro_rules! fwd_inner_refref {
        (lhs=$lhs:ident, rhs=$rhs:ident, out=$out:ident) => {   
            impl<'a, 'b> InnerProduct<&'b Lorentz4<$rhs>> for &'a Lorentz4<$lhs> 
            {
                type Output = $out;

                #[inline]
                fn dot(self, rhs: &'b Lorentz4<$rhs>) -> Self::Output {
                    (*self).dot(*rhs)
                }
            }
        };
    }

    macro_rules! fwd_inner {
        (lhs=$lhs:ident, rhs=$rhs:ident, out=$out:ident) => {
            fwd_inner_valref!(lhs=$lhs, rhs=$rhs, out=$out);
            fwd_inner_refval!(lhs=$lhs, rhs=$rhs, out=$out);
            fwd_inner_refref!(lhs=$lhs, rhs=$rhs, out=$out);
        };
    }

    fwd_inner!(lhs=Real,    rhs=Real,    out=Real);
    fwd_inner!(lhs=Complex, rhs=Complex, out=Complex);
    fwd_inner!(lhs=Real,    rhs=Complex, out=Complex);
    fwd_inner!(lhs=Complex, rhs=Real,    out=Complex);
}
pub use fwd_ops::*;
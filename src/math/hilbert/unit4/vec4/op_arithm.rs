use std::ops::{Add, Sub};
use super::*;


impl Add for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: Vec4) -> Self::Output {
        debug_assert!(
            self.basis == rhs.basis,
            "Operation Vec4 + Vec4 error: must be the same basis!"
        );

        Self::Output {
            content: [
                self[0] + rhs[0],
                self[1] + rhs[1],
                self[2] + rhs[2],
                self[3] + rhs[3]
            ],
            basis: self.basis,
        }
    }
}

impl Sub for Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: Vec4) -> Self::Output {
        debug_assert!(
            self.basis == rhs.basis,
            "Operation Vec4 - Vec4 error: must be the same basis!"
        );

        Self::Output {
            content: [
                self[0] - rhs[0],
                self[1] - rhs[1],
                self[2] - rhs[2],
                self[3] - rhs[3]
            ],
            basis: self.basis,
        }
    }
}





macro_rules! fwd_binop_valref {
    (impl $imp:ident, $method:ident) => {
        impl $imp<&Vec4> for Vec4 {
            type Output = Vec4;

            #[inline]
            fn $method(self, rhs: &Vec4) -> Self::Output {
                self.$method(*rhs)
            }
        }
    };
}

macro_rules! fwd_binop_refval {
    (impl $imp:ident, $method:ident) => {
        impl $imp<Vec4> for &Vec4 {
            type Output = Vec4;

            #[inline]
            fn $method(self, rhs: Vec4) -> Self::Output {
                (*self).$method(rhs)
            }
        }
    };
}

macro_rules! fwd_binop_refref {
    (impl $imp:ident, $method:ident) => {
        impl<'a, 'b> $imp<&'b Vec4> for &'a Vec4 {
            type Output = Vec4;

            #[inline]
            fn $method(self, rhs: &'b Vec4) -> Self::Output {
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
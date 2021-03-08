use std::ops::{Add, Sub, Mul, Div};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};
use crate::math::number::complex::*;


macro_rules! fwd_assignop {
    (impl $imp:ident, $method:ident, $methodtarget:ident) => {
        impl $imp for Complex {
            fn $method(&mut self, rhs: Complex) {
                *self = (*self).$methodtarget(rhs)
            }
        }

        impl $imp<&Complex> for Complex {
            fn $method(&mut self, rhs: &Complex) {
                *self = (*self).$methodtarget(rhs)
            }
        }
    };
}

fwd_assignop!(impl AddAssign, add_assign, add);
fwd_assignop!(impl SubAssign, sub_assign, sub);
fwd_assignop!(impl MulAssign, mul_assign, mul);
fwd_assignop!(impl DivAssign, div_assign, div);

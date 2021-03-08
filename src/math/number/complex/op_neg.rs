use std::ops::Neg;
use super::*;


impl Neg for Complex {
    type Output = Complex;

    #[inline]
    fn neg(self) -> Self::Output {
        Complex {
            real: -self.real,
            imag: -self.imag,
        }
    }
}

impl Neg for &Complex {
    type Output = Complex;

    #[inline]
    fn neg(self) -> Self::Output {
        Complex {
            real: -self.real,
            imag: -self.imag,
        }
    }
}
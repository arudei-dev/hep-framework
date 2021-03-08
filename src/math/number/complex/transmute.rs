use crate::math::number::complex::*;

pub trait NumberComplexify {
    fn as_z(&self) -> Complex;
}

impl NumberComplexify for Complex {
    #[inline]
    fn as_z(&self) -> Complex { *self }
}

impl NumberComplexify for &Complex {
    #[inline]
    fn as_z(&self) -> Complex { **self }
}

impl NumberComplexify for f64 {
    #[inline]
    fn as_z(&self) -> Complex {
        Complex::from(*self, 0.)
    }
}
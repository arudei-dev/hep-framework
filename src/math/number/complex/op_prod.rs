use crate::math::number::*;
use super::*;


impl Complex {
    #[inline]
    pub fn abs(&self) -> Self {
        Self::from(self.real.abs(), self.imag.abs())
    }

    #[inline]
    pub fn norm_sq(&self) -> Real {
        self.real * self.real  +  self.imag * self.imag
    }

    #[inline]
    pub fn norm(&self) -> f64 {
        self.real.hypot(self.imag)
    }
}
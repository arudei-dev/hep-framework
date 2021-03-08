use crate::math::number::complex::*;


impl Complex {
    #[inline]
    pub fn from_polar(r: f64, theta: f64) -> Self {
        Self {
            real: r * theta.cos(),
            imag: r * theta.sin(),
        }
    }

    #[inline]
    pub fn arg(&self) -> f64 {
        self.imag.atan2(self.real)
    }

    #[inline]
    pub fn to_polar(&self) -> (f64, f64) {
        (self.norm(), self.arg())
    }

    #[inline]
    pub fn r(&self) -> f64 {
        self.norm()
    }

    #[inline]
    pub fn theta(&self) -> f64 {
        self.arg()
    }
}
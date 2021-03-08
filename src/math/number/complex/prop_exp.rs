use crate::math::number::complex::*;


impl Complex {
    #[inline]
    pub fn exp(&self) -> Self {
        // Compute e^(self), where e := natural exponent
        
        Self::from_polar(self.real.exp(), self.imag)
    }

    #[inline]
    pub fn powf(self, exp: f64) -> Self {
        // x^y = (r e^(iθ))^y = r^y e^(iθy)
        // where
        //  - x = self, y = exp,
        //  - r = (self as polar).r,
        //  - θ = (self as polar).theta

        Self::from_polar(self.r().powf(exp), self.theta() * exp)
    }

    #[inline]
    pub fn powi(self, exp: isize) -> Self {
        self.powf(exp as f64)
    }
}
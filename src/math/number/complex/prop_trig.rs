use crate::math::number::complex::*;


impl Complex {
    #[inline]
    pub fn sin(&self) -> Self {
        let a = &self.real;
        let b = &self.imag;

        Self {
            real: a.sin() * b.cosh(),
            imag: a.cos() * b.sinh(),
        }
    }

    #[inline]
    pub fn cos(&self) -> Self {
        let a = &self.real;
        let b = &self.imag;

        Self {
            real: a.cos() * b.cosh(),
            imag: -a.sin() * b.sinh(),
        }
    }
}
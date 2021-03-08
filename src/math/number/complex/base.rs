use crate::math::number::Number;

#[derive(Clone, Copy)]
pub struct Complex {
    pub(super) real: f64,
    pub(super) imag: f64,
}

impl Complex {
    #[inline]
    pub fn new() -> Self {
        Self { real: 0., imag: 0. }
    }

    #[inline]
    pub fn from(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }

    #[inline]
    pub fn from_real(real: f64) -> Self {
        Self { real, imag: 0. }
    }

    #[inline]
    pub fn conj(&self) -> Self {
        Self {
            real: self.real,
            imag: -self.imag,
        }
    }

    #[inline]
    pub fn real(&self) -> f64 {
        self.real
    }

    #[inline]
    pub fn imag(&self) -> f64 {
        self.imag
    }
}

impl Number for Complex {
    fn zero() -> Self { Complex { real: 0., imag: 0. } }
    fn one()  -> Self { Complex { real: 1., imag: 0. } }
}
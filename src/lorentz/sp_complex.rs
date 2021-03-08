use super::*;
use crate::math::number::*;

impl Lorentz4<Complex> {
    #[inline]
    pub fn from_real(v: &Lorentz4<Real>, basis: LorentzBasis) -> Self {
        Self::new([
            Complex::from_real(v[0]),
            Complex::from_real(v[1]),
            Complex::from_real(v[2]),
            Complex::from_real(v[3]),
        ], basis)
    }

    #[inline]
    pub fn conj(&self) -> Self {
        Self::new([
            self[0].conj(),
            self[1].conj(),
            self[2].conj(),
            self[3].conj(),
        ], self.basis)
    }
}
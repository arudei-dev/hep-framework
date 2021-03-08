use crate::math::number::*;
use super::*;

#[derive(Clone, Copy)]
pub struct Lorentz4<T: Number> {
    pub(crate) content: [T; 4],
    pub(crate) basis: LorentzBasis,
}

impl<T> Lorentz4<T>
where
    T: Number + Clone + Copy
{
    #[inline]
    pub fn new(v: [T; 4], basis: LorentzBasis) -> Self {
        Self { content: v, basis }
    }

    #[inline]
    pub fn zeros(basis: LorentzBasis) -> Self {
        Self { content: [T::zero(); 4], basis }
    }

    #[inline]
    pub fn basis(&self) -> LorentzBasis {
        self.basis
    }
}
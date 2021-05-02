use std::ops::Neg;
use crate::math::number::*;
use super::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LorentzBasis { 
    Covariant, 
    Contravariant 
}
pub use LorentzBasis::{Contravariant, Covariant};


impl<T> Lorentz4<T>
where
    T: Number + Clone + Copy + Neg<Output=T>
{
    #[inline]
    pub fn to_basis(&self, basis_to: LorentzBasis) -> Lorentz4<T> {
        if self.basis == basis_to {
            *self
        }
        else {
            Lorentz4::<T> {
                content: [
                     self.content[0],
                    -self.content[1],
                    -self.content[2],
                    -self.content[3],
                ],
                basis: basis_to,
            }
        }
    }
}
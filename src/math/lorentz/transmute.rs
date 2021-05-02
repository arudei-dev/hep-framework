use crate::math::number::{Real, complex::*};
use super::*;

impl Lorentz4<Real> {
    #[inline]
    pub fn to_z(&self) -> Lorentz4<Complex> {
        Lorentz4::<Complex> {
            content: [
                self[0].as_z(),
                self[1].as_z(),
                self[2].as_z(),
                self[3].as_z(),
            ],
            basis: self.basis
        }
    }
}
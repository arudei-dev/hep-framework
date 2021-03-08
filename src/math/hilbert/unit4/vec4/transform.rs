use crate::math::hilbert::{Bra, Ket};
use super::*;


impl Vec4 {
    #[inline]
    pub fn conj_transpose(&self) -> Vec4 {
        Self {
            content: [
                self[0].conj(),
                self[1].conj(),
                self[2].conj(),
                self[3].conj(),
            ],
            basis: match self.basis {
                Bra => Ket,
                Ket => Bra,
            }
        }
    } 
}

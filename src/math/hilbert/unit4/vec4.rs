pub mod op_arithm;
pub mod op_index;
pub mod op_prod;
pub mod transform;
pub use self::op_arithm::*;
pub use self::op_index::*;
pub use self::op_prod::*;
pub use self::transform::*;
pub use crate::math::hilbert::Braket::{Bra, Ket};


use crate::math::number::*;
use crate::math::hilbert::Braket;

#[derive(Clone, Copy)]
pub struct Vec4 {
    pub(crate) content: [Complex; 4],
    pub(crate) basis: Braket,
}

impl Vec4 {
    #[inline]
    pub fn zeros(basis: Braket) -> Self {
        Self {
            content: [Complex::zero(); 4],
            basis,
        }
    }

    #[inline]
    pub fn from(v: [Complex; 4], basis: Braket) -> Self {
        Self { content: v, basis, }
    }

    #[inline]
    pub const fn basis(&self) -> Braket {
        self.basis
    }
}
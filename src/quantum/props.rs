
use crate::math::*;
pub use crate::math::hilbert::{Braket, Bra, Ket};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Spin {
    Up   = 1,
    Down = 2,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Helicity {
    Left         = -1,
    Longitudinal =  0,
    Right        =  1
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Charge {
    Positive,
    Neutral,
    Negative,
    Of(f64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Parity {
    Positive = 1,
    Negative = -1,
}

// #[derive(Debug, Clone, Copy, PartialEq)]
// pub enum Reaction {
//     Neutral,
//     Charged(Charge)
// }

pub type Reaction = Charge;


pub type StateMatrix = hilbert::mat44::Mat44;
pub type StateVector = hilbert::vec4::Vec4;

use crate::math::number::*;
use crate::lorentz::*;


pub type Momentum4 = Lorentz4<Real>;

pub fn calc_mom3_norm(p4: &Momentum4) -> Real {
    (p4[1] * p4[1] 
        + p4[2] * p4[2] 
        + p4[3] * p4[3]
    ).sqrt()
}

pub struct Particle {
    pub p4: Momentum4,
    pub mass: Real,
}

impl Particle {
    #[inline]
    pub fn zero() -> Self {
        Self {
            p4: Momentum4::zeros(Contravariant),
            mass: 0.,
        }
    }

    #[inline]
    pub fn init_with_mass(mass: Real) -> Self {
        Self {
            p4: Momentum4::zeros(Contravariant),
            mass,
        }
    }
}
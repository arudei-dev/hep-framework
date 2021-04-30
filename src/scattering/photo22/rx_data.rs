use super::*;
use crate::quantum::observable::{Particle};
use crate::quantum::props::{Charge};


pub struct RxData {
    pub charge: Charge,
    pub p_inc_a: Particle,
    pub p_inc_b: Particle,
    pub p_out_c: Particle,
    pub p_out_d: Particle,
}

impl RxData {
    #[inline]
    pub fn init(charge: Charge) -> Self {
        Self {
            charge,
            p_inc_a: Particle::zero(),
            p_inc_b: Particle::zero(),
            p_out_c: Particle::zero(),
            p_out_d: Particle::zero(),
        }
    }
}
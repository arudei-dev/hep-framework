use super::*;
use crate::quantum::observable::{Particle};
use crate::quantum::props::{Reaction};


pub struct ReactionData {
    pub rx_type: Reaction,
    pub p_inc_a: Particle,
    pub p_inc_b: Particle,
    pub p_out_c: Particle,
    pub p_out_d: Particle,
}

impl ReactionData {
    #[inline]
    pub fn init(rx_type: Reaction) -> Self {
        Self {
            rx_type,
            p_inc_a: Particle::zero(),
            p_inc_b: Particle::zero(),
            p_out_c: Particle::zero(),
            p_out_d: Particle::zero(),
        }
    }
}
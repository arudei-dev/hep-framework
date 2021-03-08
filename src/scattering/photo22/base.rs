use crate::quantum::observable::*;
use crate::quantum::props::*;
use super::*;


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

pub trait PhotoproductionBase {
    fn set_energy(&mut self, energy: Energy);

    fn set_angle(&mut self, angle: Angle);

    fn set_rx_type(&mut self, rx_type: Reaction);

    fn get_reaction_data(&self) -> &ReactionData;

    fn calc_amplitude_avgsq(&self) -> Real;
}
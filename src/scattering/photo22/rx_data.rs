use super::*;
use crate::quantum::observable::{Particle};
use crate::quantum::props::{Reaction};


pub struct ReactionData {
    pub energy: Real,
    pub angle: Real,

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
            energy: 0.,
            angle: 0.,
            p_inc_a: Particle::zero(),
            p_inc_b: Particle::zero(),
            p_out_c: Particle::zero(),
            p_out_d: Particle::zero(),
        }
    }

    pub fn calculate_momentums(&mut self) {
        let xmn = &self.p_inc_b.mass;
        let xmk = &self.p_out_c.mass;
        let xmy = &self.p_out_d.mass;

        let angle = self.angle;

        if self.energy < (xmk + xmy) {
            self.energy = 0.;
            return;
        }

        let xki = (self.energy.powi(2) - xmn.powi(2)) / (2. * self.energy);

        self.p_inc_a.p4[0] = xki;
        self.p_inc_a.p4[3] = xki;

        self.p_inc_b.p4[0] = (xki.powi(2) + xmn.powi(2)).sqrt();
        self.p_inc_b.p4[3] = -xki;

        let const_a = self.energy.powi(2) - (xmk + xmy).powi(2);
        let const_b = self.energy.powi(2) - (xmk - xmy).powi(2);

        let xqi = (const_a * const_b).sqrt() / (2. * self.energy);

        self.p_out_c.p4[0] = (xqi.powi(2) + xmk.powi(2)).sqrt();
        self.p_out_c.p4[1] = xqi * angle.sin();
        self.p_out_c.p4[3] = xqi * angle.cos();

        self.p_out_d.p4[0] = (xqi.powi(2) + xmy.powi(2)).sqrt();
        self.p_out_d.p4[1] = -xqi * angle.sin();
        self.p_out_d.p4[3] = -xqi * angle.cos();
    } 
}
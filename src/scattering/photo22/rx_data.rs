use super::*;
use crate::quantum::observable::{Particle};
use crate::quantum::props::{Reaction};

pub struct RxParticles {
    pub inc_a: Particle,
    pub inc_b: Particle,
    pub out_c: Particle,
    pub out_d: Particle,
}

pub struct ReactionData {
    pub rx_type: Reaction,
    pub energy: Real,
    pub angle: Real,

    pub particles: RxParticles,
}

impl ReactionData {
    #[inline]
    pub fn init(rx_type: Reaction) -> Self {
        Self {
            rx_type,
            energy: 0.,
            angle: 0.,
            particles: RxParticles {
                inc_a: Particle::zero(),
                inc_b: Particle::zero(),
                out_c: Particle::zero(),
                out_d: Particle::zero(),
            },
        }
    }
    
    #[inline]
    pub fn init_with_particles(rx_type: Reaction, particles: RxParticles) -> Self {
        Self {
            rx_type,
            energy: 0.,
            angle: 0.,
            particles,
        }
    }

    fn calculate_momentums(&mut self) {
        let px = &mut self.particles;

        let xmn = &px.inc_b.mass;
        let xmk = &px.out_c.mass;
        let xmy = &px.out_d.mass;

        let angle = self.angle;

        if self.energy < (xmk + xmy) {
            self.energy = 0.;
            return;
        }

        let xki = (self.energy.powi(2) - xmn.powi(2)) / (2. * self.energy);

        px.inc_a.p4[0] = xki;
        px.inc_a.p4[3] = xki;

        px.inc_b.p4[0] = (xki.powi(2) + xmn.powi(2)).sqrt();
        px.inc_b.p4[3] = -xki;

        let const_a = self.energy.powi(2) - (xmk + xmy).powi(2);
        let const_b = self.energy.powi(2) - (xmk - xmy).powi(2);

        let xqi = (const_a * const_b).sqrt() / (2. * self.energy);

        px.out_c.p4[0] = (xqi.powi(2) + xmk.powi(2)).sqrt();
        px.out_c.p4[1] = xqi * angle.sin();
        px.out_c.p4[3] = xqi * angle.cos();

        px.out_d.p4[0] = (xqi.powi(2) + xmy.powi(2)).sqrt();
        px.out_d.p4[1] = -xqi * angle.sin();
        px.out_d.p4[3] = -xqi * angle.cos();
    }

    pub fn set_energy(&mut self, energy: Energy) {
        let px = &mut self.particles;

        self.energy = match energy {
            Energy::InCMFrame(e_cm) => e_cm,
            Energy::InLabFrame(e_lab) => {
                let mass_nucl = &px.inc_b.mass;
                (
                    (2. * mass_nucl * e_lab)
                    + mass_nucl.powi(2)
                ).sqrt()
            }
        };

        self.calculate_momentums();
    }

    pub fn set_angle(&mut self, angle: Angle) {
        use crate::math::PI;

        self.angle = match angle {
            Angle::InRadian(rad) => rad,
            Angle::InDegree(deg) => deg * (PI / 180.)
        };

        self.calculate_momentums();
    }
}
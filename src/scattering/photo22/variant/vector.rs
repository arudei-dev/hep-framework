use super::*;
use crate::math::number::*;
use crate::quantum::{*, relativistic::*};


pub struct VectorParticleInv {
    pub h_photon: Helicity,
    pub h_vector: Helicity,
}
impl ParticleInvariance for VectorParticleInv {}

pub struct AmplsConfig {
    pub spin_avg: Real,
    pub h_photon: Vec<Helicity>,
    pub s_inc_b:  Vec<Spin>,
    pub h_out_c:  Vec<Helicity>,
    pub s_out_d:  Vec<Spin>,
}
impl AmplsConfiguration for AmplsConfig {}



impl PhotoproductionBase for Photoproduction<VectorParticleInv, AmplsConfig> {
    #[inline]
    fn get_rx_data(&self) -> &RxData {
        &self.reaction
    }

    #[inline]
    fn calc_amplitude_avgsq(&self) -> Real {
        if self.energy_cm == 0. { return 0. }

        let ampl_sq = |h1: Helicity, s1: Spin, h2: Helicity, s2: Spin| -> Real {
            let mut sum = Complex::zero();

            let u: &StateVector     = &dirac_spinor(&self.reaction.p_inc_b, s1);
            let u_bar: &StateVector = &dirac_adjoint(&dirac_spinor(&self.reaction.p_out_d, s2));

            for xch in &self.exchanges {
                let m = xch.calc_invariant_amplitude(&self.reaction, VectorParticleInv { h_photon: h1, h_vector: h2 });

                // < f | M | i >
                sum += u_bar * m * u;
            }

            // | < f | M | i >_1 + < f | M | i >_2 + ... < f | M | i >_n |^2
            return sum.norm_sq();
        };

        let mut sum = 0.;

        // TODO: Parallelization and iter zip.
        for h_photon in &self.ampls_cfg.h_photon {
            for s_inc_b in &self.ampls_cfg.s_inc_b {
                for h_out_c in &self.ampls_cfg.h_out_c {
                    for s_out_d in &self.ampls_cfg.s_out_d {
                        sum += ampl_sq(*h_photon, *s_inc_b, *h_out_c, *s_out_d);
                    }
                }
            }
        }

        return sum / self.ampls_cfg.spin_avg;
    }
}

pub type VectorInvPhotoproduction = Photoproduction<VectorParticleInv, AmplsConfig>;
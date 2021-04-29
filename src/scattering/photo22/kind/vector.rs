use super::*;
use crate::math::number::*;
use crate::quantum::{*, relativistic::*};


pub trait XchBaseVector : XchBase {
    fn calc_invariant_amplitude(
        &self,
        rx: &ReactionData,
        h_photon: Helicity,
        h_vector: Helicity,
    ) -> StateMatrix;
}

pub struct QuantumStatesIters {
    pub h_photon: Vec<Helicity>,
    pub h_out_c:  Vec<Helicity>,
    pub sp_inc_b: Vec<Spin>,
    pub sp_out_d: Vec<Spin>,
}

pub fn calc_ampl_avgsq(
    rx: &ReactionData, 
    xchs: &Vec<Box<dyn XchBaseVector>>, 
    iter_states: QuantumStatesIters, 
    spin_avg: Real
) -> Real {

    if rx.energy == 0. { return 0. }

    let ampl_sq = |h1: Helicity, s1: Spin, h2: Helicity, s2: Spin| -> Real {
        let mut sum = Complex::zero();

        let u: &StateVector     = &dirac_spinor(&rx.particles.inc_b, s1);
        let u_bar: &StateVector = &dirac_adjoint(&dirac_spinor(&rx.particles.out_d, s2));

        for xch in xchs {
            let m = xch.calc_invariant_amplitude(rx, h1, h2);

            // < f | M | i >
            sum += u_bar * m * u;
        }

        // | < f | M | i >_1 + < f | M | i >_2 + ... < f | M | i >_n |^2
        return sum.norm_sq();
    };

    let mut sum = 0.;

    // TODO: Parallelization and iter zip.
    for h_photon in &iter_states.h_photon {
        for s_inc_b in &iter_states.sp_inc_b {
            for h_out_c in &iter_states.h_out_c {
                for s_out_d in &iter_states.sp_out_d {
                    sum += ampl_sq(*h_photon, *s_inc_b, *h_out_c, *s_out_d);
                }
            }
        }
    }

    return sum / spin_avg;
}
use crate::math::PI;
use crate::constants::fundamental::conversion::MEVMINSQ_TO_MUBARN;
use crate::quantum::observable::calc_mom3_norm;
use super::*;


pub fn differential<'a>(photo: &'a dyn PhotoproductionBase, u: DifferentialUnit) -> Real {
    let rx = photo.get_reaction_data();

    let p4_a = &rx.p_inc_a.p4;
    let p4_b = &rx.p_inc_b.p4;

    let s = (p4_a + p4_b).norm_sq();

    let p3_f = calc_mom3_norm(&rx.p_out_c.p4);
    let p3_i = calc_mom3_norm(&rx.p_inc_a.p4);

    let modifier = match u {
        DifferentialUnit::DifOmega    => MEVMINSQ_TO_MUBARN / (64. * PI.powi(2) * s) ,
        DifferentialUnit::DifCosTheta => MEVMINSQ_TO_MUBARN / (32. * PI * s),
    };

    let m2 = photo.calc_amplitude_avgsq();

    modifier * m2 * (p3_f / p3_i)
}
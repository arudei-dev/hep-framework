use super::*;
use crate::quantum::*;

pub trait ParticleInvariance {}
pub trait ExchangeBase<T: ParticleInvariance> {
    fn calc_invariant_amplitude(&self, rx: &RxData, states: T) -> StateMatrix;
}

pub trait PhotoproductionBase {
    fn get_rx_data(&self) -> &RxData;

    fn calc_amplitude_avgsq(&self) -> Real;
}

pub struct AmplsConfig {
    pub spin_avg: Real,
    pub h_photon: Vec<Helicity>,
    pub s_inc_b:  Vec<Spin>,
    pub h_out_c:  Vec<Helicity>,
    pub s_out_d:  Vec<Spin>,
}

pub struct Photoproduction<T: ParticleInvariance> {
    pub(crate) ampls_cfg: AmplsConfig,
    pub(crate) energy_cm: Real,
    pub(crate) angle_rad: Real,

    pub(crate) exchanges: Vec<Box<dyn ExchangeBase<T>>>,
    pub(crate) reaction: RxData,
}

impl<T: ParticleInvariance> Photoproduction<T> {
    #[inline]
    pub fn init(charge: Charge, ampls_cfg: AmplsConfig) -> Self {
        Self {
            ampls_cfg,
            energy_cm: 0., angle_rad: 0.,
            exchanges: vec![],
            reaction: RxData::init(charge),
        }
    }

    #[inline]
    pub fn init_with(rx : RxData, ampls_cfg: AmplsConfig) -> Self {
        Self {
            ampls_cfg,
            energy_cm: 0., angle_rad: 0.,
            exchanges: vec![],
            reaction: rx,
        }
    }

    pub fn set_energy(&mut self, energy: Energy) {
        self.energy_cm = match energy {
            Energy::InCMFrame(e_cm) => e_cm,
            Energy::InLabFrame(e_lab) => {
                let mass_b = &self.reaction.p_inc_b.mass;
                (
                    (2. * mass_b * e_lab)
                    + mass_b.powi(2)
                ).sqrt()
            }
        };

        self.calc_momentums();
    }

    pub fn set_angle(&mut self, angle: Angle) {
        use crate::math::PI;

        self.angle_rad = match angle {
            Angle::InRadian(rad) => rad,
            Angle::InDegree(deg) => deg * (PI / 180.)
        };

        self.calc_momentums();
    }

    
    #[inline]
    pub fn add_exchange(&mut self, xch: Box<dyn ExchangeBase<T>>) {
        self.exchanges.push(xch);
    }

    #[inline]
    pub fn clear_exchanges(&mut self) {
        self.exchanges.clear();
    }


    fn calc_momentums(&mut self) {
        let rx = &mut self.reaction;


        let xmn = &rx.p_inc_b.mass;
        let xmk = &rx.p_out_c.mass;
        let xmy = &rx.p_out_d.mass;

        let angle = self.angle_rad;

        if self.energy_cm < (xmk + xmy) {
            self.energy_cm = 0.;
            return;
        }

        let xki = (self.energy_cm.powi(2) - xmn.powi(2)) / (2. * self.energy_cm);

        rx.p_inc_a.p4[0] = xki;
        rx.p_inc_a.p4[3] = xki;

        rx.p_inc_b.p4[0] = (xki.powi(2) + xmn.powi(2)).sqrt();
        rx.p_inc_b.p4[3] = -xki;

        let const_a = self.energy_cm.powi(2) - (xmk + xmy).powi(2);
        let const_b = self.energy_cm.powi(2) - (xmk - xmy).powi(2);

        let xqi = (const_a * const_b).sqrt() / (2. * self.energy_cm);

        rx.p_out_c.p4[0] = (xqi.powi(2) + xmk.powi(2)).sqrt();
        rx.p_out_c.p4[1] = xqi * angle.sin();
        rx.p_out_c.p4[3] = xqi * angle.cos();

        rx.p_out_d.p4[0] = (xqi.powi(2) + xmy.powi(2)).sqrt();
        rx.p_out_d.p4[1] = -xqi * angle.sin();
        rx.p_out_d.p4[3] = -xqi * angle.cos();
    }
}
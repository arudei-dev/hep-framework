use super::*;
use crate::quantum::*;

pub trait InvariantAmplStates {}
pub trait ExchangeBase<T: InvariantAmplStates> {
    fn calc_invariant_amplitude(
        &self,
        rx: &ReactionData,
        states: T,
    ) -> StateMatrix;
}

pub trait PhotoproductionBase {
    fn get_reaction_data(&self) -> &ReactionData;

    fn calc_amplitude_avgsq(&self) -> Real;
}

pub struct AmplsConfig {
    pub spin_avg: Real,
    pub h_photon: Vec<Helicity>,
    pub s_inc_b:  Vec<Spin>,
    pub h_out_c:  Vec<Helicity>,
    pub s_out_d:  Vec<Spin>,
}

pub struct Photoproduction<T: InvariantAmplStates> {
    pub(crate) ampl_param: AmplsConfig,
    pub(crate) exchanges: Vec<Box<dyn ExchangeBase<T>>>,
    pub(crate) rx: ReactionData,
}

impl<T: InvariantAmplStates> Photoproduction<T> {
    #[inline]
    pub fn init(rx_type: Reaction, ampl_param: AmplsConfig) -> Self {
        Self {
            ampl_param,
            exchanges: vec![],
            rx: ReactionData::init(rx_type),
        }
    }

    #[inline]
    pub fn init_with(rx_type: Reaction, ampl_param: AmplsConfig, particles: RxParticles) -> Self {
        Self {
            ampl_param,
            exchanges: vec![],
            rx: ReactionData::init_with_particles(
                rx_type, particles
            ),
        }
    }
    
    #[inline]
    pub fn add_exchange(&mut self, xch: Box<dyn ExchangeBase<T>>) {
        self.exchanges.push(xch);
    }

    #[inline]
    pub fn clear_exchanges(&mut self) {
        self.exchanges.clear();
    }

    #[inline]
    pub fn set_energy(&mut self, energy: Energy) {
        self.rx.set_energy(energy);
    }

    #[inline]
    pub fn set_angle(&mut self, angle: Angle) {
        self.rx.set_angle(angle);
    }
}
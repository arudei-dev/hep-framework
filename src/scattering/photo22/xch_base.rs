use super::*;
use crate::quantum::{ Helicity, StateMatrix };


pub trait XchBase {
  fn get_xch_mass(&self, rx: &ReactionData) -> Real;

  fn calc_form_factor(&self, rx: &ReactionData) -> Real;
}

pub trait XchBasePseudoscalar : XchBase {
    fn calc_invariant_amplitude(
        &self,
        rx: &ReactionData,
        h_photon: Helicity,
    ) -> StateMatrix;
}

pub trait XchBaseVector : XchBase {
    fn calc_invariant_amplitude(
        &self,
        rx: &ReactionData,
        h_photon: Helicity,
        h_vector: Helicity,
    ) -> StateMatrix;
}
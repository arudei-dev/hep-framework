use super::*;


pub trait PhotoproductionBase {
    fn get_reaction_data(&self) -> &ReactionData;

    fn calc_amplitude_avgsq(&self) -> Real;
}

pub trait XchBase {
  fn get_xch_mass(&self, rx: &ReactionData) -> Real;

  fn calc_form_factor(&self, rx: &ReactionData) -> Real;
}
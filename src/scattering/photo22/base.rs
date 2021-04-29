use super::*;


pub trait PhotoproductionBase {
    fn add_exchange(&mut self, xch: Box<dyn xch_base::XchBase>);

    fn clear_exchanges(&mut self);

    fn get_reaction_data(&self) -> &ReactionData;

    fn calc_amplitude_avgsq(&self) -> Real;
}
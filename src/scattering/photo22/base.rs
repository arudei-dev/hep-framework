use super::*;


pub trait PhotoproductionBase {
    fn get_reaction_data(&self) -> &ReactionData;

    fn calc_amplitude_avgsq(&self) -> Real;
}
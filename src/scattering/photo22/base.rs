use crate::quantum::props::*;
use super::*;


pub trait PhotoproductionBase {
    fn set_energy(&mut self, energy: Energy);

    fn set_angle(&mut self, angle: Angle);

    fn set_rx_type(&mut self, rx_type: Reaction);

    fn get_reaction_data(&self) -> &ReactionData;

    fn calc_amplitude_avgsq(&self) -> Real;
}
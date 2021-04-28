pub mod base;
pub mod display;
pub mod transmute;
pub mod op_assign;
pub mod op_arithm;
pub mod op_neg;
pub mod op_prod;
pub mod prop_exp;
pub mod prop_polar;
pub mod prop_trig;


pub use base::*;
pub use display::*;
pub use transmute::*;
pub use op_assign::*;
pub use op_arithm::*;
pub use op_prod::*;
pub use prop_exp::*;
pub use prop_polar::*;
pub use prop_trig::*;


pub const J: Complex = Complex { real: 0., imag: 1. };
pub const R: Complex = Complex { real: 1., imag: 0. };

#[cfg(test)]
mod tests;
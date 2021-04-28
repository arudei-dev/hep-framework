//! Mathematical tools and framework to assist computations for (most) High Energy Physics calculations.
//! The goal of this library is to help you write a computational code with an almost 1:1 comparison
//! to your final invariant amplitude derivation(s), mainly eliminating the need to do a dimensional
//! reduction from a 4x4 Dirac matrices into a 2x2 Pauli matrices, while simultaneously producing a
//! maintainable, realiable, and performant code. 
//! 
//! 
//! Currently, this library mainly contains several abstraction to help you achieve that goal, such as:
//! - Common mathematical objects (Lorentz four-vectors and Bra-Ket-compliant unit-4 matrix);
//! - Quantum Field Theory-related functions (Gamma matrices, Dirac spinor, Feynman slash, etc);
//! - Scattering-specific functions (currently for 2 -> 2 Photoproduction);
//! - and common fundamental physical constants.
//! 
//! 

pub mod constants;
pub mod math;
pub mod lorentz;
pub mod quantum;
pub mod scattering;
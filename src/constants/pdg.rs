//! Containing fundamental and commonly used particle descriptions.
//! See <https://pdg.lbl.gov/> for more information.

use crate::quantum::props::{Parity, Charge};

pub mod baryon {
    pub mod nucl {
        pub mod proton {
            use crate::constants::pdg::*;

            pub const MASS_MEV: f64         = 938.28;
            pub const MASS_AMU: f64         = 1.0072764666;
            pub const MASS_KG_POWMIN27: f64 = 1.672621924;

            pub const CHARGE: f64    = 1.;
            pub const SPIN: f64      = 1./2.;
            pub const ISOSPIN: f64   = 1./2.;
            pub const PARITY: Parity = Parity::Positive;  
        }

        pub mod neutron {
            use crate::constants::pdg::*;

            pub const MASS_MEV: f64         = 939.57;
            pub const MASS_AMU: f64         = 1.008664916;
            pub const MASS_KG_POWMIN27: f64 = 1.674927498;
            
            pub const CHARGE: f64    = -1.;
            pub const SPIN: f64      =  1./2.;
            pub const ISOSPIN: f64   =  1./2.;
            pub const PARITY: Parity = Parity::Positive;  
        }
    }

    pub mod hyperon {
        pub mod lambda {
            use crate::constants::pdg::*;

            pub const MASS_MEV: f64 = 1116.;

            pub const CHARGE: Charge = Charge::Neutral;
            pub const PARITY: Parity = Parity::Positive; 
            pub const SPIN: f64      = 1./2.;
            pub const ISOSPIN: f64   = 0.; 
        }

        pub mod sigma_p {
            use crate::constants::pdg::*;

            pub const MASS_MEV: f64 = 1189.;

            pub const CHARGE: Charge = Charge::Positive;
            pub const PARITY: Parity = Parity::Positive;
            pub const SPIN: f64      = 1.;
            pub const ISOSPIN: f64   = 1./2.;  
        }

        pub mod sigma_0 {
            use crate::constants::pdg::*;

            pub const MASS_MEV: f64 = 1192.;

            pub const CHARGE: Charge = Charge::Neutral;
            pub const PARITY: Parity = Parity::Positive;  
            pub const SPIN: f64      = 1.;
            pub const ISOSPIN: f64   = 1./2.;
        }

        pub mod sigma_m {
            use crate::constants::pdg::*;

            pub const MASS_MEV: f64 = 1197.;

            pub const CHARGE: Charge = Charge::Negative;
            pub const PARITY: Parity = Parity::Positive;
            pub const SPIN: f64      = 1.;
            pub const ISOSPIN: f64   = 1./2.;
        }
    }

    // pub const MEV_LAMBDA: f64 = 1116.;
    // pub const MEV_SIGMA: f64 = 1192.;
}

pub mod meson {
    pub mod pseudoscalar {
        pub mod eta {
            use crate::constants::pdg::*;

            pub const MASS_MEV: f64 = 547.853;

            pub const CHARGE: Charge = Charge::Neutral;
            pub const PARITY: Parity = Parity::Negative;
            pub const SPIN: f64      = 0.;
            pub const ISOSPIN: f64   = 0.;  
        }

        pub mod eta_prime {
            use crate::constants::pdg::*;

            pub const MASS_MEV: f64 = 957.66;

            pub const CHARGE: Charge = Charge::Neutral;
            pub const PARITY: Parity = Parity::Negative;
            pub const SPIN: f64      = 0.;
            pub const ISOSPIN: f64   = 0.;  
        }
        
        pub mod kaon {
            use crate::constants::pdg::*;

            pub const MASS_MEV: f64 = 493.677;
            
            pub const PARITY: Parity = Parity::Negative;
            pub const SPIN: f64      = 0.;
            pub const ISOSPIN: f64   = 1./2.;  

            pub const CHARGE: Charge   = Charge::Positive;
            pub const STRANGENESS: f64 = 1.; 

            pub mod anti {
                use super::*;

                pub const CHARGE: Charge   = Charge::Negative;
                pub const STRANGENESS: f64 = -1.; 
            }
        }

        pub mod kaon_0 {
            use crate::constants::pdg::*;

            pub const MASS_MEV: f64 = 497.614;
            
            pub const PARITY: Parity = Parity::Negative;
            pub const SPIN: f64      = 0.;
            pub const ISOSPIN: f64   = 1./2.;

            pub const CHARGE: Charge   = Charge::Neutral;
            pub const STRANGENESS: f64 = 1.; 

            pub mod anti {
                pub const STRANGENESS: f64 = -1.; 
            }
        }

        pub mod pion {
            use crate::constants::pdg::*;

            pub const MASS_MEV: f64 = 139.570;
            
            pub const PARITY: Parity = Parity::Negative;
            pub const SPIN: f64      = 0.;
            pub const ISOSPIN: f64   = 1.;

            pub const CHARGE: Charge   = Charge::Positive;

            pub mod anti {
                use super::*;

                pub const CHARGE: Charge   = Charge::Negative;
            }
        }

        pub mod pion_0 {
            use crate::constants::pdg::*;

            pub const MASS_MEV: f64 = 134.9766;
            
            pub const PARITY: Parity = Parity::Negative;
            pub const SPIN: f64      = 0.;
            pub const ISOSPIN: f64   = 1.;

            pub const CHARGE: Charge   = Charge::Negative;

        }
    }


    pub mod vector {
        pub mod kaonstar {
            pub mod charged {
                use crate::constants::pdg::*;
    
                pub const MASS_MEV: f64 = 892.;
                
                pub const PARITY: Parity       = Parity::Negative;
                pub const CHARGES: [Charge; 2] = [Charge::Positive, Charge::Negative];
                pub const SPIN: f64            = 1./2.;
                pub const ISOSPIN: f64         = 1.;

            }

            pub mod neutral {
                use crate::constants::pdg::*;
    
                pub const MASS_MEV: f64 = 700.;
                
                pub const PARITY: Parity = Parity::Positive;
                pub const CHARGE: Charge = Charge::Neutral;
                pub const SPIN: f64      = 1./2.;
                pub const ISOSPIN: f64   = 0.;

            }
        }
    }
}
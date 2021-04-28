//! Containing fundamental physical constants, units, and conversions

pub mod dimensionless {
    //! Contains dimensionless physical constants (independent from any
    //! dimensional units)

    /// The Fine-structure constant (this dictates the strength of 
    /// the electromagnetic interaction between elementary charged particles.)
    pub const FINE_STRUCTURE: f64 = 1. / 137.;
    
    pub mod g_factor {
        //! A dimensionless quantity that characterizes the magnetic moment 
        //! and angular momentum of an atom, a particle or the nucleus.

        /// The experimental value g-factor of an electron.
        pub const OF_ELECTRON: f64 = -2.00231930436256;
        
        /// The experimental value g-factor of a neutron.
        pub const OF_NEUTRON: f64 = -3.82608545;

        /// The experimental value g-factor of a proton.
        pub const OF_PROTON: f64 = 5.5856946893;
    }

    pub mod anomal_magnetic_moments {
        pub const OF_PROTON: f64 = 1.79;
        pub const OF_NEUTRON: f64 = -1.91;
    }
}

pub mod hep_units {
    //! Contains some physical constants used in High Energy Physics
    //! relative to the natural units (c = h_bar = 1).

    /// The unit elementary charge (Q).
    pub const UNIT_ELEMENTARY_CHARGE: f64 = 0.30282212088;
}

pub mod conversion {
    //! Contains some constant(s) for dimensional unit conversion(s)

    /// Conversion from MeV^(-2) to μb.
    pub const MEVMINSQ_TO_MUBARN: f64 = 389379000.;
}
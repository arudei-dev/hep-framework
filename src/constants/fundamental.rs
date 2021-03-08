

pub mod dimensionless {
    pub const FINE_STRUCTURE: f64 = 1. / 137.;
    
    pub mod g_factor {
        pub const OF_ELECTRON: f64 = -2.00231930436256;
        pub const OF_NEUTRON: f64 = -3.82608545;
        pub const OF_PROTON: f64 = 5.5856946893;
    }

    pub mod anomal_magnetic_moments {
        pub const OF_PROTON: f64 = 1.79;
        pub const OF_NEUTRON: f64 = -1.91;
    }
}

pub mod hep_units {
    pub const UNIT_ELEMENTARY_CHARGE: f64 = 0.30282212088;
}

pub mod conversion {
    // Conversion from MeV^(-2) to μb.
    pub const MEVMINSQ_TO_MUBARN: f64 = 389379000.;
}
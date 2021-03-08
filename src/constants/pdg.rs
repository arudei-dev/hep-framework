

pub mod baryons {
    pub mod nucls {
        pub const MEV_PROTON: f64 = 938.28;
        pub const MEV_NEUTRON: f64 = 939.57;
    }

    pub const MEV_LAMBDA: f64 = 1116.;
    pub const MEV_SIGMA: f64 = 1192.;
    pub const MEV_SIGMA1385: f64 = 1385.;
    pub const MEV_SIGMA1780: f64 = 1780.;
}

pub mod mesons {
    pub mod scalar {
        // const double MEV_KAON_PLUS = 493.677;
        pub const MEV_KAON_PLUS: f64 = 495.;
        pub const MEV_KAON_ZERO: f64 = 497.648;

        pub const MEV_KAPPA900: f64 = 900.;
        pub const MEV_KAPPA800: f64 = 800.;
    }

    pub mod vector {
        pub const MEV_KAONSTAR_PLUS: f64 = 892.;
        pub const MEV_KAONSTAR_ZERO: f64 = 895.81;
    }
}
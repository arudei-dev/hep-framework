use crate::math::number::Real;

pub mod photo22;

pub enum Energy {
    InCMFrame(Real),
    InLabFrame(Real),
}

pub enum Angle {
    InDegree(Real),
    InRadian(Real),
}

pub enum DifferentialUnit {
    DifOmega,
    DifCosTheta,
}
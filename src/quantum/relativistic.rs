use std::ops::{Add, Sub, Mul, Neg};
use crate::math::number::*;
use crate::lorentz::*;
use super::*;


pub fn dirac_gamma(idx: usize) -> StateMatrix {
    let mut gamma_m = StateMatrix::zeros();

    match idx {
        0 => {
            gamma_m[(0, 0)] =  1.*R;
            gamma_m[(1, 1)] =  1.*R;
            gamma_m[(2, 2)] = -1.*R;
            gamma_m[(3, 3)] = -1.*R;
        },
        1 => {
            gamma_m[(0, 3)] =  1.*R;
            gamma_m[(1, 2)] =  1.*R;
            gamma_m[(2, 1)] = -1.*R;
            gamma_m[(3, 0)] = -1.*R;
        },
        2 => {
            gamma_m[(0, 3)] = -J;
            gamma_m[(1, 2)] =  J;
            gamma_m[(2, 1)] =  J;
            gamma_m[(3, 0)] = -J;
        },
        3 => {
            gamma_m[(0, 2)] =  1.*R;
            gamma_m[(1, 3)] = -1.*R;
            gamma_m[(2, 0)] = -1.*R;
            gamma_m[(3, 1)] =  1.*R;
        },
        5 => {
            gamma_m[(0, 2)] =  1.*R;
            gamma_m[(1, 3)] =  1.*R;
            gamma_m[(2, 0)] =  1.*R;
            gamma_m[(3, 1)] =  1.*R;
        },
        _ => {
            panic!("Dirac gamma matrices request error: Invalid index!");
        }

    }

    gamma_m
}

pub fn dirac_spinor(p: &Particle, s: Spin) -> StateVector {
    let mass   = &p.mass;
    let energy = &p.p4[0];
    let p1     = &p.p4[1];
    let p2     = &p.p4[2];
    let p3     = &p.p4[3];

    let mut spinor = StateVector::zeros(Ket);

    let e_mass_sqrt = (energy + mass).sqrt().as_z();

    match s {
        Spin::Up => {
            spinor[0] = e_mass_sqrt;
            spinor[1] = 0_f64.as_z();
            spinor[2] = e_mass_sqrt * (p3          / (energy + mass));
            spinor[3] = e_mass_sqrt * ((p1 + J*p2) / (energy + mass));
        },
        Spin::Down => {
            spinor[0] = 0_f64.as_z();
            spinor[1] = e_mass_sqrt;
            spinor[2] = e_mass_sqrt * ((p1 - J*p2) / (energy + mass));
            spinor[3] = e_mass_sqrt * (-p3         / (energy + mass));
        }
    }

    spinor
}

/// DEFINITION: Ⱥ := γ^μ A_μ (A_μ -- Covariant Lorentz-4 of A)
pub fn feynman_slash<T>(target: &Lorentz4<T>) -> StateMatrix
where
    T: Number + NumberComplexify + Clone + Copy + Add<Output=T> + Sub<Output=T>
       + Mul<Output=T> + Neg<Output=T>
{
    let co_sgn = if target.basis() == Covariant { 1. } else { -1. };

    let p0 = target[0].as_z();
    let p1 = co_sgn * target[1].as_z();
    let p2 = co_sgn * target[2].as_z();
    let p3 = co_sgn * target[3].as_z();

    let mut slashed = StateMatrix::zeros();

    slashed[(0, 0)] =  p0;
    slashed[(1, 1)] =  p0;
    slashed[(2, 2)] = -p0;
    slashed[(3, 3)] = -p0;

    slashed[(2, 0)] = -p3;
    slashed[(3, 1)] =  p3;
    slashed[(0, 2)] =  p3;
    slashed[(1, 3)] = -p3;

    slashed[(0, 3)] =  p1 - (J * p2);
    slashed[(1, 2)] =  p1 + (J * p2);
    slashed[(2, 1)] = -p1 + (J * p2);
    slashed[(3, 0)] = -p1 - (J * p2);

    // slashed[(0, 0)] =  p0;
    // slashed[(1, 1)] =  p0;
    // slashed[(2, 2)] = -p0;
    // slashed[(3, 3)] = -p0;

    // slashed[(2, 0)] = -p3;
    // slashed[(3, 1)] =  p3;
    // slashed[(0, 2)] =  p3;
    // slashed[(1, 3)] = -p3;

    // slashed[(0, 3)] =  p1 - (J * p2);
    // slashed[(1, 2)] =  p1 + (J * p2);
    // slashed[(2, 1)] = -p1 + (J * p2);
    // slashed[(3, 0)] = -p1 - (J * p2);

    slashed
}

pub fn polarization_state(p: &Particle, h: Helicity) -> Lorentz4<Complex> {
    match h {
        Helicity::Longitudinal => {
            if p.mass == 0. {
                return Lorentz4::<Complex>::zeros(Contravariant);
            }
            else {
                let mass   = &p.mass;
                let energy = &p.p4[0];
                let p3     = &p.p4[3];

                Lorentz4::<Complex>::new([
                    (p3 / mass).as_z(),
                    0_f64.as_z(),
                    0_f64.as_z(),
                    (energy / mass).as_z(),
                ], Contravariant)
            }
        },
        Helicity::Left => {
            Lorentz4::<Complex>::new([
                0_f64.as_z(),
                (1. / 2_f64.sqrt()).as_z(),
                -J  / 2_f64.sqrt(),
                0_f64.as_z(),
            ], Contravariant)
        },
        Helicity::Right => {
            Lorentz4::<Complex>::new([
                0_f64.as_z(),
                (-1. / 2_f64.sqrt()).as_z(),
                 -J  / 2_f64.sqrt(),
                0_f64.as_z(),
            ], Contravariant)
        }
    }
}

#[inline]
pub fn dirac_adjoint(v: &StateVector) -> StateVector {
    debug_assert!(v.basis() == Ket, "Dirac adjoint error: Input must be a Ket vec4!");

    v.conj_transpose() * dirac_gamma(0)
}
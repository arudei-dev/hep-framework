
pub trait Number {
    fn zero() -> Self;
    fn one()  -> Self;
}

pub type Real = f64;

impl Number for Real {
    fn zero() -> Self { 0. }
    fn one()  -> Self { 1. }
}
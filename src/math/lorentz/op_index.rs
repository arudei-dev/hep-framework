use std::ops::{Index, IndexMut};
use crate::math::number::Number;
use super::*;

impl<T> Index<usize> for Lorentz4<T>
where
    T: Number + Clone + Copy
{
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        debug_assert!(idx < 4, "Lorentz-4: out of bounds index access!");

        &self.content[idx]
    }
}

impl<T> Index<usize> for &Lorentz4<T>
where
    T: Number + Clone + Copy
{
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        debug_assert!(idx < 4, "Lorentz-4: out of bounds index access!");

        &self.content[idx]
    }
}

impl<T> IndexMut<usize> for Lorentz4<T>
where
    T: Number + Clone + Copy
{
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        debug_assert!(idx < 4, "Lorentz-4: out of bounds index access!");

        &mut self.content[idx]
    }
}
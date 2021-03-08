use std::ops::{Index, IndexMut};
use crate::math::number::*;
use super::*;

impl Index<usize> for Vec4 {
    type Output = Complex;

    #[inline]
    fn index(&self, idx: usize) -> &Self::Output {
        debug_assert!(idx < 4, "Hilbert Vec4: out of bounds index access!");

        &self.content[idx]
    }
}

impl IndexMut<usize> for Vec4 {
    #[inline]
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        debug_assert!(idx < 4, "Hilbert Vec4: out of bounds mut index access!");

        &mut self.content[idx]
    }
}
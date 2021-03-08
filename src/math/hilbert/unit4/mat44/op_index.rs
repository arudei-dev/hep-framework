use std::ops::{Index, IndexMut};
use crate::math::number::*;
use super::*;

impl Index<(usize, usize)> for Mat44 {
    type Output = Complex;

    #[inline]
    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        debug_assert!(idx.0 < 4 || idx.1 < 4, "Hilbert Mat44: out of bounds mut index access!");

        let row = &idx.0;
        let col = &idx.1;

        &self.content[(row * 4) + col]
    }
}

impl IndexMut<(usize, usize)> for Mat44 {
    #[inline]
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        debug_assert!(idx.0 < 4 || idx.1 < 4, "Hilbert Mat44: out of bounds mut index access!");
        
        let row = &idx.0;
        let col = &idx.1;

        &mut self.content[(row * 4) + col]
    }
}
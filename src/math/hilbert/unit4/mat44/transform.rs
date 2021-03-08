use unroll::unroll_for_loops;
use super::*;


impl Mat44 {
    #[inline]
    #[unroll_for_loops]
    pub fn conj_transpose(&self) -> Mat44 {
        let mut transpose = Mat44::zeros();

        for i in 0..4 {
            for j in 0..4 {
                transpose[(j, i)] = self[(i, j)].conj();
            }
        }

        transpose
    }
}
//! Contains trait implementations (mainly for foreign data type)
//! for augmenting transmutation to our complex data type.


use crate::math::number::complex::*;

pub trait NumberComplexify {
    fn as_z(&self) -> Complex;
}

impl NumberComplexify for Complex {
    #[inline]
    fn as_z(&self) -> Complex { *self }
}

impl NumberComplexify for &Complex {
    #[inline]
    fn as_z(&self) -> Complex { **self }
}

impl NumberComplexify for f64 {
    /// Transmute the current f64 data type to our complex data type.
    /// This has the same effect of calling `Complex::from_real(num1)`
    /// where `num1` is our original variable with type f64.
    /// 
    /// # Example
    /// ```
    /// use <...>::complex::transmute::*;
    /// 
    /// let num1: f64 = 1.;
    /// let z = num1.as_z();
    /// 
    /// println!("z = {}", z);
    /// ```
    /// 
    /// Result:
    /// ```
    ///     z = Complex {
    ///         real: 1.,
    ///         imag: 0.,
    ///     }
    /// ```
    #[inline]
    fn as_z(&self) -> Complex {
        Complex::from(*self, 0.)
    }
}
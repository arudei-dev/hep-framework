//! Base module for the complex number implementations.
//! Contains the minimum requirements (main struct and
//! basic impls) to create an instance of a complex number.

use crate::math::number::Number;

#[derive(Clone, Copy)]
pub struct Complex {
    pub(super) real: f64,
    pub(super) imag: f64,
}

impl Complex {

    /// Create a new instance of a complex number of zeros.
    /// 
    /// # Examples
    /// ```
    /// let z = Complex::new();
    /// 
    /// println!("z = {}", z);
    /// ```
    /// 
    /// Result:
    /// ```
    ///     z = Complex {
    ///         real: 0.,
    ///         imag: 0.,
    ///     }
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self { real: 0., imag: 0. }
    }


    /// Create a new instance of a complex number from a pair of numbers.
    /// 
    /// # Examples
    /// ```
    /// let z = Complex::from(1., -1);
    /// 
    /// println!("z = {}", z);
    /// ```
    /// 
    /// Result:
    /// ```
    ///     z = Complex {
    ///         real:  1.,
    ///         imag: -1.,
    ///     }
    /// ```
    #[inline]
    pub fn from(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }


    /// Create a new instance of a complex number from a real number.
    /// 
    /// # Examples
    /// ```
    /// let z = Complex::from_real(1.);
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
    pub fn from_real(real: f64) -> Self {
        Self { real, imag: 0. }
    }


    /// Takes the current complex number as a reference and conjugates it.
    /// 
    /// # Examples
    /// ```
    /// // Initialize a complex number variable
    /// let z = Complex::from(1., 1.);
    /// 
    /// // Conjugate it
    /// let z_conj = z.conj();
    /// ```
    /// 
    /// Result: 
    /// ```
    ///     z_conj { 
    ///         real: 1., 
    ///         imag: -1. // Before: 1.
    ///     };
    /// ```
    #[inline]
    pub fn conj(&self) -> Self {
        Self {
            real: self.real,
            imag: -self.imag,
        }
    }


    /// Takes the real part of the current complex number.
    /// 
    /// # Examples
    /// ```
    /// // Initialize a complex number variable
    /// let z = Complex::from(1., -1.);
    /// 
    /// // Takes the real part
    /// let z_re = z.real();
    /// 
    /// println!("z_re = {}", z_re);
    /// ```
    /// 
    /// Result: 
    /// ```
    ///     z_re = 1.
    /// ```
    #[inline]
    pub fn real(&self) -> f64 {
        self.real
    }


    /// Takes the imaginary part of the current complex number.
    /// 
    /// # Examples
    /// ```
    /// // Initialize a complex number variable
    /// let z = Complex::from(1., -1.);
    /// 
    /// // Takes the imaginary part
    /// let z_im = z.imag();
    /// 
    /// println!("z_im = {}", z_im);
    /// ```
    /// 
    /// Result: 
    /// ```
    ///     z_im = -1.
    /// ```
    #[inline]
    pub fn imag(&self) -> f64 {
        self.imag
    }
}

impl Number for Complex {
    fn zero() -> Self { Complex { real: 0., imag: 0. } }
    fn one()  -> Self { Complex { real: 1., imag: 0. } }
}
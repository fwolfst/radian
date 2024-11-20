#![cfg_attr(not(feature = "std"), no_std)]
#![deny(clippy::std_instead_of_core)]

#[cfg(all(feature = "std", feature = "libm"))]
compile_error!("The `std` and `libm` features may not both be enabled simultaneously.");

use core::{f64::consts::PI, fmt::Display};

mod ops;

/// A wrapper around a `f64` angle that is guaranteed to always be normalized.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle {
    value: f64,
}

impl Angle {
    pub const ZERO: Self = Self { value: 0.0 };
    pub const PI: Self = Self { value: PI };
    pub const NEG_PI: Self = Self { value: -PI };

    /// Create a new [`Angle`] by normalizing the given angle in radians.
    ///
    /// ```
    /// # use radian::Angle;
    /// # use core::f64::consts::PI;
    /// let angle = Angle::new(PI * 5.0);
    /// assert_eq!(angle.radians(), PI);
    /// ```
    pub fn new(radians: f64) -> Self {
        let two_pi = 2.0 * PI;
        let mut normalized = radians % two_pi;
        if normalized > PI {
            normalized -= two_pi;
        } else if normalized < -PI {
            normalized += two_pi;
        }
        Angle { value: normalized }
    }

    /// Create a new [`Angle`] from degrees.
    ///
    /// ```
    /// # use radian::Angle;
    /// # use core::f64::consts::PI;
    /// let angle = Angle::from_degrees(180.0);
    /// assert_eq!(angle.radians(), PI);
    /// ```
    pub fn from_degrees(degrees: f64) -> Angle {
        Angle::new(degrees.to_radians())
    }

    /// Get the angle in radians.
    ///
    /// ```
    /// # use radian::Angle;
    /// # use core::f64::consts::PI;
    /// let angle = Angle::new(PI);
    /// assert_eq!(angle.radians(), PI);
    /// ```
    #[inline(always)]
    pub fn radians(&self) -> f64 {
        self.value
    }

    /// Get the angle in degrees.
    ///
    /// ```
    /// # use radian::Angle;
    /// # use core::f64::consts::PI;
    /// let angle = Angle::new(PI);
    /// assert_eq!(angle.degrees(), 180.0);
    /// ```
    #[inline(always)]
    pub fn degrees(&self) -> f64 {
        self.value.to_degrees()
    }

    /// Get the absolute of this angle.
    ///
    /// ```
    /// # use radian::Angle;
    /// let angle = Angle::new(-1.0);
    /// assert_eq!(angle.abs().radians(), 1.0);
    /// ```
    #[inline(always)]
    pub fn abs(&self) -> Angle {
        #[cfg(feature = "std")]
        return Angle {
            value: self.value.abs(),
        };

        #[cfg(feature = "libm")]
        return Angle {
            value: libm::fabs(self.value),
        };

        #[cfg(all(not(feature = "std"), not(feature = "libm")))]
        if self.value.is_sign_positive() {
            *self
        } else {
            -*self
        }
    }

    /// Get the difference between this angle and another one.
    ///
    /// ```
    /// # use radian::Angle;
    /// # use core::f64::consts::PI;
    /// let a = Angle::new(-PI / 2.0);
    /// let b = Angle::new(PI / 2.0);
    /// assert_eq!(a.difference(&b).radians(), PI);
    /// ```
    #[inline(always)]
    pub fn difference(&self, other: &Self) -> Self {
        Angle::new(other.value - self.value)
    }

    /// Get the absolute distance between this angle and another one.
    ///
    /// ```
    /// # use radian::Angle;
    /// # use core::f64::consts::PI;
    /// let a = Angle::new(-PI);
    /// let b = Angle::new(PI / 2.0);
    /// assert_eq!(a.distance(&b).radians(), PI / 2.0);
    /// ```
    #[inline(always)]
    pub fn distance(&self, other: &Self) -> Self {
        Angle::new(self.value - other.value).abs()
    }

    /// Check if this angle is in a clockwise driection from another one.
    ///
    /// ```
    /// # use radian::Angle;
    /// # use core::f64::consts::PI;
    /// let a = Angle::new(PI);
    /// let b = Angle::new(PI / 2.0);
    /// assert!(a.is_clockwise_to(&b));
    /// ```
    pub fn is_clockwise_to(&self, other: &Self) -> bool {
        self.difference(other).value < 0.0
    }

    /// Check if this angle is in a counterclockwise driection from another one.
    ///
    /// ```
    /// # use radian::Angle;
    /// # use core::f64::consts::PI;
    /// let a = Angle::new(PI / 2.0);
    /// let b = Angle::new(PI);
    /// assert!(a.is_counterclockwise_to(&b));
    /// ```
    pub fn is_counterclockwise_to(&self, other: &Self) -> bool {
        self.difference(other).value > 0.0
    }

    /// Clamp the angle to a given range.
    ///
    /// ```
    /// # use radian::Angle;
    /// # use core::f64::consts::PI;
    /// let angle = Angle::new(PI);
    /// let clamped = angle.clamp(Angle::ZERO, Angle::new(PI / 2.0));
    /// assert_eq!(clamped.radians(), PI / 2.0);
    /// ```
    pub fn clamp(&self, min: Angle, max: Angle) -> Angle {
        if self.value < min.value {
            min
        } else if self.value > max.value {
            max
        } else {
            *self
        }
    }

    /// Interpolate between this angle and another by a given factor (t).
    ///
    /// ```
    /// # use radian::Angle;
    /// # use core::f64::consts::PI;
    /// let a = Angle::new(0.0);
    /// let b = Angle::new(PI);
    /// let interpolated = a.lerp(&b, 0.5);
    /// assert_eq!(interpolated.radians(), PI / 2.0);
    /// ```
    pub fn lerp(&self, other: &Self, t: f64) -> Angle {
        assert!((0.0..=1.0).contains(&t), "t must be between 0.0 and 1.0");
        let difference = self.difference(other);
        Angle::new(self.value + difference.value * t)
    }

    /// Get the opposite angle (add π).
    ///
    /// ```
    /// # use radian::Angle;
    /// # use core::f64::consts::PI;
    /// let angle = Angle::new(-PI / 2.0);
    /// assert_eq!(angle.opposite().radians(), PI / 2.0);
    /// ```
    pub fn opposite(&self) -> Angle {
        Angle::new(self.value + PI)
    }

    /// Find the midpoint angle between this angle and another one.
    ///
    /// ```
    /// # use radian::Angle;
    /// # use core::f64::consts::PI;
    /// let a = Angle::new(0.0);
    /// let b = Angle::new(PI);
    /// assert_eq!(a.midpoint(&b).radians(), PI / 2.0);
    /// ```
    pub fn midpoint(&self, other: &Self) -> Self {
        let diff = self.difference(other).value;
        Angle::new(self.value + diff / 2.0)
    }

    /// Get the unit vector of this angle defined as (cos(θ), sin(θ)).
    ///
    /// ```
    /// # use radian::Angle;
    /// # use core::f64::consts::{PI, SQRT_2};
    /// # use approx::assert_relative_eq;
    /// let angle = Angle::new(PI / 4.0);
    /// let (x, y) = angle.to_unit_vector();
    /// assert_relative_eq!(x,  SQRT_2 / 2.0);
    /// assert_relative_eq!(y,  SQRT_2 / 2.0);
    /// ```
    #[cfg(any(feature = "std", feature = "libm"))]
    pub fn to_unit_vector(&self) -> (f64, f64) {
        #[cfg(feature = "std")]
        return (self.value.cos(), self.value.sin());
        #[cfg(feature = "libm")]
        return (libm::cos(self.value), libm::sin(self.value));
    }

    /// Create an angle from a unit vector (x, y).
    ///
    /// ```
    /// # use radian::Angle;
    /// # use core::f64::consts::{PI, SQRT_2};
    /// # use approx::assert_relative_eq;
    /// let angle = Angle::from_unit_vector(SQRT_2 / 2.0, SQRT_2 / 2.0);
    /// assert_relative_eq!(angle.radians(), PI / 4.0);
    /// ```
    #[cfg(any(feature = "std", feature = "libm"))]
    pub fn from_unit_vector(x: f64, y: f64) -> Angle {
        #[cfg(feature = "std")]
        return Angle::new(y.atan2(x));
        #[cfg(feature = "libm")]
        return Angle::new(libm::atan2(y, x));
    }
}

impl Display for Angle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} radians", self.value)
    }
}

#[cfg(feature = "ufmt")]
impl ufmt::uDisplay for Angle {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        ufmt::uwrite!(f, "{} radians", ufmt_float::uFmt_f64::Five(self.value))
    }
}

#[cfg(test)]
mod tests {
    use super::Angle;
    use core::f64::consts::PI;

    #[test]
    fn test_in_range() {
        assert_eq!(Angle::new(PI).radians(), PI);
        assert_eq!(Angle::new(-PI).radians(), -PI);
        assert_eq!(Angle::new(PI / 2.0).radians(), PI / 2.0);
        assert_eq!(Angle::new(-PI / 2.0).radians(), -PI / 2.0);
        assert_eq!(Angle::new(0.0).radians(), 0.0);
    }

    #[test]
    fn test_out_of_range() {
        assert_eq!(Angle::new(PI + 0.1).radians(), -PI + 0.1);
        assert_eq!(Angle::new(-PI - 0.1).radians(), PI - 0.1);
    }

    #[test]
    fn test_math_ops() {
        assert_eq!((Angle::PI - Angle::PI / 2.0).radians(), PI / 2.0);
        assert_eq!((Angle::PI * 2.0).radians(), 0.0);
    }
}

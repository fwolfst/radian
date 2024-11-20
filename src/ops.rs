use super::Angle;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

impl Add for Angle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.radians() + rhs.radians())
    }
}

impl AddAssign for Angle {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::new(self.radians() + rhs.radians());
    }
}

impl Sub for Angle {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.radians() - rhs.radians())
    }
}

impl SubAssign for Angle {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self::new(self.radians() - rhs.radians());
    }
}

impl Mul for Angle {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.radians() * rhs.radians())
    }
}

impl MulAssign for Angle {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self::new(self.radians() * rhs.radians())
    }
}

impl Mul<f64> for Angle {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.radians() * rhs)
    }
}

impl MulAssign<f64> for Angle {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self::new(self.radians() * rhs)
    }
}

impl Div for Angle {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.radians() / rhs.radians())
    }
}

impl DivAssign for Angle {
    fn div_assign(&mut self, rhs: Self) {
        *self = Self::new(self.radians() / rhs.radians())
    }
}

impl Div<f64> for Angle {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.radians() / rhs)
    }
}

impl DivAssign<f64> for Angle {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self::new(self.radians() / rhs)
    }
}

impl Neg for Angle {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Angle { value: -self.value }
    }
}

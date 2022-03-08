use crate::{util, Hsva, SrgbRgba};
use en::Num;
use std::ops::{Add, Div, Mul, Sub};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LinRgba([f32; 4]);

impl LinRgba {
    impl_ctors!();

    pub fn map_hsv(self, f: impl FnOnce(Hsva) -> Hsva) -> Self {
        f(Hsva::from_rgba(self)).to_rgba()
    }

    /// Change the alpha channel.
    ///
    /// Note that alpha is always pre-multiplied, meaning the RGB channels will
    /// be multiplied by this new alpha value. Therefore, calling this more than
    /// once may be destructive.
    pub fn a(self, alpha: f32) -> Self {
        Self(util::map_color(util::map_alpha(self.0, |_| alpha), |ch| {
            ch * alpha
        }))
    }

    pub fn to_srgb(self) -> SrgbRgba {
        util::map_color(self.0, util::linear_to_srgb).into()
    }
}

impl_from_into!(LinRgba);

impl Add for LinRgba {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
            self.0[3] + rhs.0[3],
        ])
    }
}

impl<T: en::Float> Add<T> for LinRgba {
    type Output = Self;

    fn add(self, rhs: T) -> Self {
        util::map_all(self.0, |lhs| (lhs.cast::<T>() + rhs).to_f32()).into()
    }
}

impl Sub for LinRgba {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
            self.0[3] - rhs.0[3],
        ])
    }
}

impl<T: en::Float> Sub<T> for LinRgba {
    type Output = Self;

    fn sub(self, rhs: T) -> Self {
        util::map_all(self.0, |lhs| (lhs.cast::<T>() - rhs).to_f32()).into()
    }
}

impl Mul for LinRgba {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self([
            self.0[0] * rhs.0[0],
            self.0[1] * rhs.0[1],
            self.0[2] * rhs.0[2],
            self.0[3] * rhs.0[3],
        ])
    }
}

impl<T: en::Float> Mul<T> for LinRgba {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        util::map_all(self.0, |lhs| (lhs.cast::<T>() * rhs).to_f32()).into()
    }
}

impl Div for LinRgba {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self([
            self.0[0] / rhs.0[0],
            self.0[1] / rhs.0[1],
            self.0[2] / rhs.0[2],
            self.0[3] / rhs.0[3],
        ])
    }
}

impl<T: en::Float> Div<T> for LinRgba {
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        util::map_all(self.0, |lhs| (lhs.cast::<T>() / rhs).to_f32()).into()
    }
}

pub fn lerp<F: en::Float>(start: LinRgba, end: LinRgba, factor: F) -> LinRgba {
    start * (F::one() - factor) + end * factor
}

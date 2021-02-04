macro_rules! impl_ctors {
    () => {
        pub fn zero() -> Self {
            [0.0; 4].into()
        }

        pub fn one() -> Self {
            [1.0; 4].into()
        }

        pub const fn from_f32(r: f32, g: f32, b: f32, a: f32) -> Self {
            Self::from_f32_array([r, g, b, a])
        }

        // We avoid the use of `From`/`Into` in these implementations to make it
        // easier to mark things `const` down the road...
        pub fn from_unorm8(r: uno::Unorm8, g: uno::Unorm8, b: uno::Unorm8, a: uno::Unorm8) -> Self {
            Self::from_unorm8_array([r, g, b, a])
        }

        pub fn from_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
            Self::from_u8_array([r, g, b, a])
        }

        pub const fn from_f32_array(color: [f32; 4]) -> Self {
            Self(color)
        }

        pub fn from_unorm8_array(color: [uno::Unorm8; 4]) -> Self {
            Self::from_f32_array(crate::util::map_all(color, uno::Unorm8::to_float::<f32>))
        }

        pub fn from_u8_array(color: [u8; 4]) -> Self {
            Self::from_unorm8_array(crate::util::map_all(color, uno::Unorm8::from_inner))
        }

        pub const fn into_f32_array(self) -> [f32; 4] {
            self.0
        }

        pub fn into_unorm8_array(self) -> [uno::Unorm8; 4] {
            crate::util::map_all(self.0, uno::Unorm8::from_float_clamped)
        }

        pub fn into_u8_array(self) -> [u8; 4] {
            crate::util::map_all(self.into(), uno::Unorm8::to_inner)
        }
    };
}

macro_rules! impl_from_into {
    ($ty:ty) => {
        impl From<[f32; 4]> for $ty {
            fn from(color: [f32; 4]) -> Self {
                Self::from_f32_array(color)
            }
        }

        impl From<[uno::Unorm8; 4]> for $ty {
            fn from(color: [uno::Unorm8; 4]) -> Self {
                Self::from_unorm8_array(color)
            }
        }

        impl From<[u8; 4]> for $ty {
            fn from(color: [u8; 4]) -> Self {
                Self::from_u8_array(color)
            }
        }

        impl Into<[f32; 4]> for $ty {
            fn into(self) -> [f32; 4] {
                self.into_f32_array()
            }
        }

        impl Into<[uno::Unorm8; 4]> for $ty {
            fn into(self) -> [uno::Unorm8; 4] {
                self.into_unorm8_array()
            }
        }

        impl Into<[u8; 4]> for $ty {
            fn into(self) -> [u8; 4] {
                self.into_u8_array()
            }
        }
    };
}

pub mod colors;
mod lin_rgba;
mod srgb_rgba;

pub use crate::{lin_rgba::LinRgba, srgb_rgba::SrgbRgba};
pub use chromatose_macros::{rgb, rgba};
pub use chromatose_shared as util;

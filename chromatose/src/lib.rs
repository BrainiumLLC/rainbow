pub mod colors;
mod lin_rgba;
mod srgb_rgba;

pub use crate::{lin_rgba::LinRgba, srgb_rgba::SrgbRgba};
pub use chromatose_macros::color;
pub use chromatose_shared as util;

use crate::{util, SrgbRgba};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LinRgba([f32; 4]);

impl LinRgba {
    impl_ctors!();

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

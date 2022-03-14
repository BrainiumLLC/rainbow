use crate::{util, Hsva, LinRgba};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SrgbRgba([f32; 4]);

impl SrgbRgba {
    impl_ctors!();

    pub fn map_hsv(self, f: impl FnOnce(Hsva) -> Hsva) -> Self {
        f(Hsva::from_rgba(self.to_linear())).to_rgba().to_srgb()
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

    /// Convert to pre-multiplied alpha.
    /// This should only be called when consuming data that isn't pre-multiplied.
    pub fn pma(self) -> Self {
        self.a(self.0[3])
    }

    pub fn to_linear(self) -> LinRgba {
        util::map_color(self.0, util::srgb_to_linear).into()
    }
}

impl_from_into!(SrgbRgba);

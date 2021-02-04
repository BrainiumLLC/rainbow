use crate::{util, LinRgba};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SrgbRgba([f32; 4]);

impl SrgbRgba {
    impl_ctors!();

    /// Change the alpha channel.
    ///
    /// Note that alpha is always pre-multiplied, meaning the RGB channels will
    /// be multiplied by this new alpha value. Therefore, calling this more than
    /// once may be destructive.
    ///
    /// This internally converts to linear color and back, so if you're going to
    /// convert to linear as a final step anyway, you should call this after that.
    pub fn a(self, alpha: f32) -> Self {
        self.to_linear().a(alpha).to_srgb()
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

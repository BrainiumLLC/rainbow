use crate::{util, LinRgba};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SrgbRgba(pub [f32; 4]);

impl SrgbRgba {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self([r, g, b, a])
    }

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
        LinRgba(util::map_color(self.0, util::srgb_to_linear))
    }
}

impl Into<[u8; 4]> for SrgbRgba {
    fn into(self) -> [u8; 4] {
        util::map_all(self.0, |ch| {
            if ch <= 0.0 {
                0x00
            } else if ch >= 1.0 {
                0xFF
            } else {
                let mul = (0xFF as f32 * ch).round();
                debug_assert!(mul >= 0x00 as f32);
                debug_assert!(mul <= 0xFF as f32);
                mul as u8
            }
        })
    }
}

impl From<[u8; 4]> for SrgbRgba {
    fn from(rgba: [u8; 4]) -> Self {
        Self(util::map_all(rgba, |ch| ch as f32 / 0xFF as f32))
    }
}

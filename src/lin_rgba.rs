use crate::{util, SrgbRgba};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LinRgba(pub [f32; 4]);

impl LinRgba {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self([r, g, b, a])
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
        SrgbRgba(util::map_color(self.0, util::linear_to_srgb))
    }
}

#[cfg(feature = "gee")]
impl Into<gee::Vec4<f32>> for LinRgba {
    fn into(self) -> gee::Vec4<f32> {
        gee::Vec4::new(self.0[0], self.0[1], self.0[2], self.0[3])
    }
}

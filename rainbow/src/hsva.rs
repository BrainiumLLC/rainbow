use crate::LinRgba;

// TODO: this entire module is worthy of much scrutiny
// (wrt pma in particular)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hsva {
    hue: f32,
    saturation: f32,
    value: f32,
    alpha: f32,
}

impl Hsva {
    pub const fn new(hue: f32, saturation: f32, value: f32, alpha: f32) -> Self {
        Self {
            hue,
            saturation,
            value,
            alpha,
        }
    }

    pub fn from_rgba(color: LinRgba) -> Self {
        let [r, g, b, alpha] = color.into_f32_array();
        let min_channel = r.min(g).min(b);
        let max_channel = r.max(g).max(b);
        let delta = max_channel - min_channel;
        let value = max_channel;
        let (hue, saturation) = if max_channel > 0.0 {
            let saturation = delta / max_channel;
            let hue = {
                let hue = if r == max_channel {
                    (g - b) / delta
                } else if g == max_channel {
                    2.0 + (b - r) / delta
                } else {
                    4.0 + (r - g) / delta
                } * 60.0;
                if hue < 0.0 {
                    hue + 360.0
                } else {
                    hue
                }
            };
            (hue, saturation)
        } else {
            // zero saturation = lossy conversion; the choice of hue here is
            // completely arbitrary
            (0.0, 0.0)
        };
        Self {
            hue,
            saturation,
            value,
            alpha,
        }
    }

    pub fn to_rgba(self) -> LinRgba {
        if self.saturation == 0.0 {
            LinRgba::from_f32(self.value, self.value, self.value, self.alpha)
        } else {
            let (i, fract) = {
                let hh = if self.hue >= 360.0 {
                    0.0
                } else {
                    self.hue / 60.0
                };
                (hh.trunc() as usize, hh.fract())
            };
            let p = self.value * (1.0 - self.saturation);
            let q = self.value * (1.0 - self.saturation * fract);
            let t = self.value * (1.0 - self.saturation * (1.0 - fract));
            match i {
                0 => LinRgba::from_f32(self.value, t, p, self.alpha),
                1 => LinRgba::from_f32(q, self.value, p, self.alpha),
                2 => LinRgba::from_f32(p, self.value, t, self.alpha),
                3 => LinRgba::from_f32(p, q, self.value, self.alpha),
                4 => LinRgba::from_f32(t, p, self.value, self.alpha),
                _ => LinRgba::from_f32(self.value, p, q, self.alpha),
            }
        }
    }

    pub fn map_saturation(mut self, f: impl FnOnce(f32) -> f32) -> Self {
        self.saturation = f(self.saturation).clamp(0.0, 1.0);
        self
    }

    pub fn map_value(mut self, f: impl FnOnce(f32) -> f32) -> Self {
        self.value = f(self.value).clamp(0.0, 1.0);
        self
    }

    pub fn invert(mut self) -> Self {
        self.hue = (self.hue + 180.0) % 360.0;
        self
    }
}

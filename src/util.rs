pub fn linear_to_srgb(l: f32) -> f32 {
    if l < 0.0031308 {
        12.92 * l
    } else {
        1.055f32.mul_add(l.powf(0.416666657 /* 2.4.recip() */), -0.055)
    }
}

// https://github.com/apitrace/dxsdk/blob/4978eb035ef0de7385f865ebde8f77b37236fd4c/Include/d3dx_dxgiformatconvert.inl#L287
pub fn srgb_to_linear(l: f32) -> f32 {
    if l < 0.04045 {
        l * 0.0773993805 // = 12.92.recip()
    } else {
        l.mul_add(0.947867333, 0.0521327034).powf(2.4)
    }
}

pub fn map_all<T: Copy, U>(color: [T; 4], f: impl Fn(T) -> U) -> [U; 4] {
    [f(color[0]), f(color[1]), f(color[2]), f(color[3])]
}

/// Applies a function to the RGB components of a color.
/// The alpha component isn't changed.
pub fn map_color(color: [f32; 4], f: impl Fn(f32) -> f32) -> [f32; 4] {
    [f(color[0]), f(color[1]), f(color[2]), color[3]]
}

pub fn map_alpha(color: [f32; 4], f: impl Fn(f32) -> f32) -> [f32; 4] {
    [color[0], color[1], color[2], f(color[3])]
}

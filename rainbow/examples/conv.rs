use rainbow::SrgbRgba;

fn main() {
    // You can create colors using floats...
    let color = dbg!(SrgbRgba::from_f32(0.0, 1.0, 0.5, 1.0));
    // or using integers. These colors are approximately identical, being only
    // slightly different due to `u8` affording less precision.
    dbg!(SrgbRgba::from_u8(0x00, 0xFF, 0x80, 0xFF));
    // Naturally, you can also convert to arrays of either type:
    dbg!(color.into_f32_array());
    dbg!(color.into_u8_array());
    // If you want to convert to linear, that's easy too:
    dbg!(color.to_linear());
    // `LinRgba` has all of the same conversion methods as `SrgbRgba`, save for
    // providing `to_srgb` instead of `to_linear`.
}

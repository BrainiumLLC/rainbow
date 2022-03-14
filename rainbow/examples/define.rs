use rainbow::{rgb, LinRgba, SrgbRgba};

// You can define colors using hexadecimal at compile-time! Note that this value
// is interpreted as sRGB, since that's what color-pickers / artists / etc. tend
// to work in.
rgb!(AN_AUSPICIOUS_COLOR, 0x00FF80);

fn main() {
    // The unsuffixed color name gives you the color in an sRGB encoding. This
    // makes sense, since you originally defined the color with an sRGB value.
    println!("srgb: {:?}", AN_AUSPICIOUS_COLOR);
    // If you suffix with `_LINEAR`, you get the same color encoded as linear.
    println!("linear:  {:?}", AN_AUSPICIOUS_COLOR_LINEAR);
}

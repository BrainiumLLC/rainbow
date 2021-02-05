use rainbow::{rgb, LinRgba, SrgbRgba};

// You can define colors using hexadecimal at compile-time! Note that this value
// is interpreted as sRGB, since that's what color-pickers / artists / etc. tend
// to work in.
rgb!(AN_AUSPICIOUS_COLOR, 0x00FF80);

fn main() {
    // The unsuffixed color name gives you the color in a linear encoding. This
    // might seem counter-intuitive, since you originally defined this name with
    // an sRGB value... however, this reflects the general intention when
    // working with colors: sRGB is the usual way they're defined, and linear
    // is the usual way they're operated on.
    println!("linear: {:?}", AN_AUSPICIOUS_COLOR);
    // If you suffix with `_SRGB`, you get the same color encoded as sRGB.
    println!("srgb:  {:?}", AN_AUSPICIOUS_COLOR_SRGB);
}

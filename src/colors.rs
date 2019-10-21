use crate::SrgbRgba;

#[macro_export]
macro_rules! color {
    ($name:ident, $rgba:expr) => {
        pub const $name: SrgbRgba = SrgbRgba($rgba);
    };
}

color!(BLACK, [0.0, 0.0, 0.0, 1.0]);
color!(WHITE, [1.0, 1.0, 1.0, 1.0]);
color!(RED, [1.0, 0.0, 0.0, 1.0]);
color!(GREEN, [0.0, 1.0, 0.0, 1.0]);
color!(BLUE, [0.0, 0.0, 1.0, 1.0]);
color!(MAGIC_PINK, [1.0, 0.0, 1.0, 1.0]);
color!(INVISIBLE, [0.0, 0.0, 0.0, 0.0]);

color!(FRAN_PINK, [1.0, 0.5, 0.7, 1.0]);
color!(FUSCHIA, [0.9, 0.3, 0.6, 1.0]);
color!(ELECTRIC_MINT, [0.5, 1.0, 0.7, 1.0]);
color!(HONEY_MUSTARD, [0.9, 0.8, 0.5, 1.0]);
color!(SKY_BLUE, [0.7, 0.9, 1.0, 1.0]);
color!(PRINCE, [0.7, 0.3, 1.0, 1.0]);

color!(UNICORN_HAIR, [0.87, 0.82, 0.95, 1.0]);

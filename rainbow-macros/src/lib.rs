extern crate proc_macro;

use self::proc_macro::TokenStream;
use quote::quote;
use rainbow_shared as util;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, parse_str, Ident, LitInt, Token};

struct Rgb {
    name: Ident,
    rgb: LitInt,
}

impl Parse for Rgb {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let rgb: LitInt = input.parse()?;
        Ok(Self { name, rgb })
    }
}

impl Rgb {
    fn name_srgb(&self) -> Ident {
        parse_str::<Ident>(&format!("{}_SRGB", self.name)).expect("suffixed identifier invalid")
    }

    fn packed(&self) -> u32 {
        self.rgb.base10_parse::<u32>().expect("color invalid")
    }

    fn srgb(&self) -> [f32; 4] {
        let bytes = self.packed().to_be_bytes();
        let divisor = u8::max_value() as f32;
        [
            bytes[1] as f32 / divisor,
            bytes[2] as f32 / divisor,
            bytes[3] as f32 / divisor,
            1.0,
        ]
    }
}

struct Rgba {
    rgb: Rgb,
    a: LitInt,
}

impl Parse for Rgba {
    fn parse(input: ParseStream) -> Result<Self> {
        let rgb: Rgb = input.parse()?;
        input.parse::<Token![,]>()?;
        let a: LitInt = input.parse()?;
        Ok(Self { rgb, a })
    }
}

impl Rgba {
    fn srgb(&self) -> [f32; 4] {
        let [r, g, b, _] = self.rgb.srgb();
        let divisor = u8::max_value() as f32;
        let a = self.a.base10_parse::<u8>().expect("alpha invalid") as f32 / divisor;
        [r * a, g * a, b * a, a]
    }
}

fn expand(name: &Ident, name_srgb: Ident, srgb: [f32; 4]) -> TokenStream {
    let lin = util::map_color(srgb, util::srgb_to_linear);
    let (lr, lg, lb, la) = (lin[0], lin[1], lin[2], lin[3]);
    let (sr, sg, sb, sa) = (srgb[0], srgb[1], srgb[2], srgb[3]);
    let expanded = quote! {
        pub const #name: LinRgba = LinRgba::from_f32(#lr, #lg, #lb, #la);
        pub const #name_srgb: SrgbRgba = SrgbRgba::from_f32(#sr, #sg, #sb, #sa);
    };
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn rgb(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Rgb);
    expand(&input.name, input.name_srgb(), input.srgb())
}

#[proc_macro]
pub fn rgba(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Rgba);
    expand(&input.rgb.name, input.rgb.name_srgb(), input.srgb())
}

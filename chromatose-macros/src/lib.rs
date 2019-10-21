extern crate proc_macro;

use self::proc_macro::TokenStream;
use chromatose_shared as util;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, parse_str, Ident, LitInt, Token};

struct Color {
    name: Ident,
    raw: LitInt,
}

impl Parse for Color {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let raw: LitInt = input.parse()?;
        Ok(Self { name, raw })
    }
}

#[proc_macro]
pub fn color(input: TokenStream) -> TokenStream {
    let Color { name, raw } = parse_macro_input!(input as Color);
    let name_srgb =
        parse_str::<Ident>(&format!("{}_SRGB", name)).expect("suffixed identifier invalid");
    let packed = raw.base10_parse::<u32>().expect("color invalid");
    let bytes = packed.to_be_bytes();
    let divisor = u8::max_value() as f32;
    let srgb = if packed > 0xFFFFFF {
        [
            bytes[0] as f32 / divisor,
            bytes[1] as f32 / divisor,
            bytes[2] as f32 / divisor,
            bytes[3] as f32 / divisor,
        ]
    } else {
        [
            bytes[1] as f32 / divisor,
            bytes[2] as f32 / divisor,
            bytes[3] as f32 / divisor,
            1.0,
        ]
    };
    let lin = util::map_color(srgb, util::srgb_to_linear);
    let (lr, lg, lb, la) = (lin[0], lin[1], lin[2], lin[3]);
    let (sr, sg, sb, sa) = (srgb[0], srgb[1], srgb[2], srgb[3]);
    let expanded = quote! {
        pub const #name: LinRgba = LinRgba([#lr, #lg, #lb, #la]);
        pub const #name_srgb: SrgbRgba = SrgbRgba([#sr, #sg, #sb, #sa]);
    };
    TokenStream::from(expanded)
}

use std::fs;

use heck::ToPascalCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::array_to_tokenstream;

pub(crate) fn build() -> TokenStream {
    println!("cargo:rerun-if-changed=../assets/sounds.json");

    let sound: Vec<String> =
        serde_json::from_str(&fs::read_to_string("../assets/sounds.json").unwrap())
            .expect("Failed to parse sounds.json");
    let variants = array_to_tokenstream(&sound);

    let type_from_name = &sound
        .iter()
        .map(|sound| {
            let id = &sound;
            let name = format_ident!("{}", sound.to_pascal_case());

            quote! {
                #id => Some(Self::#name),
            }
        })
        .collect::<TokenStream>();

    let type_to_name = &sound
        .iter()
        .map(|sound| {
            let id = &sound;
            let name = format_ident!("{}", sound.to_pascal_case());

            quote! {
                Self::#name => #id,
            }
        })
        .collect::<TokenStream>();

    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(u16)]
        pub enum Sound {
            #variants
        }

        impl Sound {
            pub fn from_name(name: &str) -> Option<Self> {
                match name {
                    #type_from_name
                    _ => None
                }
            }

            pub const fn to_name(&self) -> &'static str {
                match self {
                    #type_to_name
                }
            }
        }
    }
}

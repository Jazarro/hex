use proc_macro::TokenStream;

use quote::{format_ident, quote, quote_spanned};
use syn::{Data, DeriveInput};
use syn::spanned::Spanned;

#[proc_macro_derive(SfxId)]
pub fn derive_sfx_id(input: TokenStream) -> TokenStream {
    derive_id(input, "SfxId")
}
#[proc_macro_derive(MusicId)]
pub fn derive_music_id(input: TokenStream) -> TokenStream {
    derive_id(input, "MusicId")
}
#[proc_macro_derive(InputAction)]
pub fn derive_input_action(input: TokenStream) -> TokenStream {
    derive_id(input, "InputAction")
}

/// See: https://doc.rust-lang.org/book/ch19-06-macros.html#how-to-write-a-custom-derive-macro
fn derive_id(input: TokenStream, trait_name: &str) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let enum_item_literal = match &ast.data {
        Data::Enum(data_enum) => {
            let variants = data_enum.variants.iter().map(|v| {
                if matches!(v.fields, syn::Fields::Unit) {
                    let mut path = syn::Path::from(name.clone());
                    path.segments.push(v.ident.clone().into());
                    let literal = v.ident.to_string();
                    quote! { #path { .. } => #literal }
                } else {
                    quote_spanned! {
                        v.fields.span() => _ => { compile_error!("InputActions can only be derived for simple unit-like enums."); }
                    }
                }
            });
            quote! {
                match self {
                    #(#variants),*
                }
            }
        }
        _ => {
            return quote_spanned! {
                name.span() => compile_error!("InputActions can only be derived for simple unit-like enums.");
            }
                .into();
        }
    };
    let enum_name_literal = name.to_string();
    let trait_ident = format_ident!("{}", trait_name);
    let output = quote! {
        impl #trait_ident for #name {
            fn group_id(&self) -> &'static str {
                #enum_name_literal
            }
            fn item_id(&self) -> &'static str {
                #enum_item_literal
            }
        }
    };
    output.into()
}

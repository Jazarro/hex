use proc_macro::TokenStream;

use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Data, DeriveInput};

/// This is a custom derive macro for the InputAction type.
///
/// See: https://doc.rust-lang.org/book/ch19-06-macros.html#how-to-write-a-custom-derive-macro
#[proc_macro_derive(InputAction)]
pub fn derive_input_action(input: TokenStream) -> TokenStream {
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
    let output = quote! {
        impl InputAction for #name {
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

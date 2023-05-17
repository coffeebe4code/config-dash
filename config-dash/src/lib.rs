use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_attribute]
pub fn check_config(
    attribute: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    println!("attribute {}", attribute.to_string());
    println!("input {}", input.to_string());
    input
}

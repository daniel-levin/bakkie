use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input, parse_quote};

#[proc_macro_derive(Argument)]
pub fn payload(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let expanded = quote! {
        const _: () = {
            #[derive(bakkie::serde::Serialize, bakkie::serde::Deserialize, bakkie::schemars::JsonSchema)]
            #[schemars(crate = "bakkie::schemars")]
            #[serde(crate = "bakkie::serde")]
            #input
        };
    };
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn input(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    // Add serde derives to the existing derives
    input
        .attrs
        .push(parse_quote!(#[derive(bakkie::serde::Serialize, bakkie::serde::Deserialize, bakkie::schemars::JsonSchema)]));

    input
        .attrs
        .push(parse_quote!(#[serde(crate = "bakkie::serde")]));

    input
        .attrs
        .push(parse_quote!(#[schemars(crate = "bakkie::schemars")]));

    let expanded = quote! {
        #input
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn tool(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let output: proc_macro2::TokenStream = quote! {};

    proc_macro::TokenStream::from(output)
}

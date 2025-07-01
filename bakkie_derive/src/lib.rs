use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_attribute]
pub fn tool(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{attr}\"");
    println!("item: \"{item}\"");
    item
}

#[proc_macro_attribute]
pub fn prompt(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{attr}\"");
    println!("item: \"{item}\"");
    item
}

#[proc_macro_derive(Argument)]
pub fn payload(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let expanded = quote! {
    const _: () = {        #[derive(bakkie::serde::Serialize, bakkie::serde::Deserialize, bakkie::schemars::JsonSchema)]
            #[schemars(crate = "bakkie::schemars")]
            #[serde(crate = "bakkie::serde")]
            #input
        };
        };
    TokenStream::from(expanded)
}

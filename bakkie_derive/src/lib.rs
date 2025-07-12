use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input, parse_quote};

#[proc_macro_attribute]
pub fn structured(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

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
    let input = parse_macro_input!(input as syn::ItemFn);

    let fn_name = &input.sig.ident;
    let fn_vis = &input.vis;
    let fn_attrs = &input.attrs;
    let fn_output = &input.sig.output;
    let fn_body = &input.block;
    let fn_asyncness = &input.sig.asyncness;

    // Generate struct name from function name
    let struct_name = syn::Ident::new(
        &format!("{}Args", fn_name.to_string().to_case(Case::Pascal)),
        fn_name.span(),
    );

    // Extract function parameters and create struct fields
    let mut struct_fields = Vec::new();
    let mut field_names = Vec::new();

    for input_param in input.sig.inputs.iter() {
        if let syn::FnArg::Typed(pat_type) = input_param {
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                let field_name = &pat_ident.ident;
                let field_type = &pat_type.ty;

                struct_fields.push(quote! {
                    pub #field_name: #field_type
                });

                field_names.push(field_name);
            }
        }
    }

    let output = quote! {
        #[derive(bakkie::serde::Serialize, bakkie::serde::Deserialize, bakkie::schemars::JsonSchema)]
        #[serde(crate = "bakkie::serde")]
        #[schemars(crate = "bakkie::schemars")]
        pub struct #struct_name {
            #(#struct_fields),*
        }

        #(#fn_attrs)*
        #fn_vis #fn_asyncness fn #fn_name(args: #struct_name) #fn_output {
            let #struct_name { #(#field_names),* } = args;
            #fn_body
        }
    };

    TokenStream::from(output)
}

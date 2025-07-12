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

    // Check if this function has a receiver parameter (self, &self, &mut self)
    // which would indicate it's a method
    for param in &input.sig.inputs {
        if let syn::FnArg::Receiver(_) = param {
            return syn::Error::new_spanned(
                &input.sig.ident,
                "#[tool] can only be used on bare functions, not methods",
            )
            .to_compile_error()
            .into();
        }
    }

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

    // Build the tool particulars constructor name, e.g. count_letters_particulars
    let particulars_fn = syn::Ident::new(
        &format!("{}_particulars", fn_name),
        fn_name.span(),
    );
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

        // Constructor for this tool's static particulars
        #fn_vis fn #particulars_fn() -> bakkie::provisions::tools::ToolParticulars {
            bakkie::provisions::tools::ToolParticulars {
                name: stringify!(#fn_name).to_string(),
                title: todo!(),
                description: todo!(),
                input_schema: bakkie::schemars::schema_for!(#struct_name),
                output_schema: todo!(),
            }
        }
    };

    TokenStream::from(output)
}

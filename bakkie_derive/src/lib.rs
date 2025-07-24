use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, parse_macro_input, parse_quote};

#[proc_macro_attribute]
pub fn structured(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    input
        .attrs
        .push(parse_quote!(#[derive(Debug, Clone, bakkie::serde::Serialize, bakkie::serde::Deserialize, bakkie::schemars::JsonSchema)]));

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
pub fn tool(args: TokenStream, input: TokenStream) -> TokenStream {
    // parse named arguments: name, title, description as name = "value" pairs
    let args = parse_macro_input!(args with syn::punctuated::Punctuated::<syn::MetaNameValue, syn::Token![,]>::parse_terminated);
    let mut name_lit: Option<syn::LitStr> = None;
    let mut title_lit: Option<syn::LitStr> = None;
    let mut description_lit: Option<syn::LitStr> = None;
    for nv in args {
        let ident = nv.path.get_ident().map(|i| i.to_string());
        let lit = match nv.value {
            syn::Expr::Lit(expr_lit) => match expr_lit.lit {
                syn::Lit::Str(l) => l,
                other => {
                    return syn::Error::new_spanned(other, "expected string literal")
                        .to_compile_error()
                        .into();
                }
            },
            other => {
                return syn::Error::new_spanned(other, "expected string literal")
                    .to_compile_error()
                    .into();
            }
        };
        match ident.as_deref() {
            Some("name") => {
                if name_lit.is_some() {
                    return syn::Error::new_spanned(nv.path, "duplicate 'name' attribute")
                        .to_compile_error()
                        .into();
                }
                name_lit = Some(lit);
            }
            Some("title") => {
                if title_lit.is_some() {
                    return syn::Error::new_spanned(nv.path, "duplicate 'title' attribute")
                        .to_compile_error()
                        .into();
                }
                title_lit = Some(lit);
            }
            Some("description") => {
                if description_lit.is_some() {
                    return syn::Error::new_spanned(nv.path, "duplicate 'description' attribute")
                        .to_compile_error()
                        .into();
                }
                description_lit = Some(lit);
            }
            _ => {
                return syn::Error::new_spanned(nv.path, "unknown tool attribute")
                    .to_compile_error()
                    .into();
            }
        }
    }
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

    // Check if this function is async
    if input.sig.asyncness.is_none() {
        return syn::Error::new_spanned(
            &input.sig.ident,
            "#[tool] can only be used on async functions",
        )
        .to_compile_error()
        .into();
    }

    let mut doc_strings = vec![];

    for att in &input.attrs {
        if let syn::Attribute {
            meta:
                syn::Meta::NameValue(syn::MetaNameValue {
                    path: syn::Path { segments, .. },
                    value:
                        syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(s),
                            ..
                        }),
                    ..
                }),
            ..
        } = att
        {
            let syn::PathSegment { ident, .. } = &segments[0];
            if ident == "doc" {
                doc_strings.push(s.value());
            }
        }
    }

    let fn_name = &input.sig.ident;
    let fn_vis = &input.vis;
    let fn_attrs = &input.attrs;
    let fn_output = &input.sig.output;
    let fn_body = &input.block;

    // Generate struct name from function name
    let struct_name = format_ident!("{}Args", fn_name);

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

    // Build function names: swap so tool constructor gets original name
    let impl_fn_name = format_ident!("{}_impl", fn_name); // Original function renamed
    let particulars_fn = format_ident!("{}_particulars", fn_name);
    // Tool constructor gets the original function name
    let tool_fn_name = fn_name.clone();
    // Prepare expressions for name, title, and description based on attribute args or defaults
    let name_expr = if let Some(lit) = name_lit {
        quote! { #lit.to_string() }
    } else {
        quote! { stringify!(#fn_name).to_string() }
    };
    let title_expr = if let Some(lit) = title_lit {
        quote! { Some(#lit.to_string()) }
    } else {
        quote! { None }
    };

    let description_expr = if let Some(lit) = description_lit {
        quote! { Some(#lit.to_string()) }
    } else if !doc_strings.is_empty() {
        let doc_string_description = doc_strings.join("\n");
        quote! { Some(#doc_string_description.to_string()) }
    } else {
        quote! { None }
    };

    let output = quote! {
        #[derive(bakkie::serde::Serialize, bakkie::serde::Deserialize, bakkie::schemars::JsonSchema)]
        #[serde(crate = "bakkie::serde")]
        #[schemars(crate = "bakkie::schemars")]
        #[allow(non_camel_case_types)]
        pub struct #struct_name {
            #(#struct_fields),*
        }

        #(#fn_attrs)*
        #[allow(non_snake_case)]
        #fn_vis async fn #impl_fn_name(args: #struct_name) #fn_output {
            let #struct_name { #(#field_names),* } = args;
            #fn_body
        }

        // Constructor for this tool's static particulars
        #[allow(non_snake_case)]
        #fn_vis fn #particulars_fn() -> bakkie::provisions::tools::ToolParticulars {

            use ::bakkie::schemars::JsonSchema;
            use ::bakkie::schemars::SchemaGenerator;
            use ::bakkie::schemars::generate::SchemaSettings;

            let mut set = SchemaSettings::openapi3();

            set.inline_subschemas = true;

            let mut g = SchemaGenerator::new(set);

            bakkie::provisions::tools::ToolParticulars {
                name: #name_expr,
                title: #title_expr,
                description: #description_expr,
                input_schema: #struct_name :: json_schema(&mut g),
                output_schema: None,
            }
        }

        // Constructor for the complete tool
        #[allow(non_snake_case)]
        #fn_vis fn #tool_fn_name() -> bakkie::provisions::tools::Tool {
            bakkie::provisions::tools::Tool {
                particulars: #particulars_fn(),
                tool_fn: Box::new(|tool_input: bakkie::provisions::tools::ToolInput| {
                    Box::pin(async move {
                        // Parse the input parameters from JSON
                        let args: #struct_name = match serde_json::from_value(
                            serde_json::Value::Object(tool_input.params)
                        ) {
                            Ok(args) => args,
                            Err(e) => return Err(bakkie::provisions::tools::ToolError::InvalidInput(e.to_string())),
                        };

                        // Call the actual tool function
                        match #impl_fn_name(args).await {
                            Ok(result) => Ok(Box::new(result) as Box<dyn bakkie::provisions::tools::AsToolOutput>),
                            Err(e) => Err(e),
                        }
                    })
                }),
            }
        }
    };

    TokenStream::from(output)
}

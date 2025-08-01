use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, parse_macro_input, parse_quote};

struct ToolMetadata {
    name_expr: proc_macro2::TokenStream,
    title_expr: proc_macro2::TokenStream,
    description_expr: proc_macro2::TokenStream,
}

fn parse_tool_attributes(
    args: syn::punctuated::Punctuated<syn::MetaNameValue, syn::Token![,]>,
    fn_name: &syn::Ident,
    doc_strings: &[String],
) -> Result<ToolMetadata, TokenStream> {
    // parse named arguments: name, title, description as name = "value" pairs
    let mut name_lit: Option<syn::LitStr> = None;
    let mut title_lit: Option<syn::LitStr> = None;
    let mut description_lit: Option<syn::LitStr> = None;

    for nv in args {
        let ident = nv.path.get_ident().map(|i| i.to_string());
        let lit = match nv.value {
            syn::Expr::Lit(expr_lit) => match expr_lit.lit {
                syn::Lit::Str(l) => l,
                other => {
                    return Err(syn::Error::new_spanned(other, "expected string literal")
                        .to_compile_error()
                        .into());
                }
            },
            other => {
                return Err(syn::Error::new_spanned(other, "expected string literal")
                    .to_compile_error()
                    .into());
            }
        };
        match ident.as_deref() {
            Some("name") => {
                if name_lit.is_some() {
                    return Err(
                        syn::Error::new_spanned(nv.path, "duplicate 'name' attribute")
                            .to_compile_error()
                            .into(),
                    );
                }
                name_lit = Some(lit);
            }
            Some("title") => {
                if title_lit.is_some() {
                    return Err(
                        syn::Error::new_spanned(nv.path, "duplicate 'title' attribute")
                            .to_compile_error()
                            .into(),
                    );
                }
                title_lit = Some(lit);
            }
            Some("description") => {
                if description_lit.is_some() {
                    return Err(syn::Error::new_spanned(
                        nv.path,
                        "duplicate 'description' attribute",
                    )
                    .to_compile_error()
                    .into());
                }
                description_lit = Some(lit);
            }
            _ => {
                return Err(syn::Error::new_spanned(nv.path, "unknown tool attribute")
                    .to_compile_error()
                    .into());
            }
        }
    }

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

    Ok(ToolMetadata {
        name_expr,
        title_expr,
        description_expr,
    })
}

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
    let args = parse_macro_input!(args with syn::punctuated::Punctuated::<syn::MetaNameValue, syn::Token![,]>::parse_terminated);
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

    // Parse tool attributes and get the processed metadata
    let metadata = match parse_tool_attributes(args, fn_name, &doc_strings) {
        Ok(metadata) => metadata,
        Err(error) => return error,
    };

    // Generate struct name from function name
    let struct_name = format_ident!("{}Args", fn_name);

    // Extract function parameters and create struct fields
    let mut struct_fields = Vec::new();
    let mut field_names = Vec::new();
    let mut app_param: Option<syn::FnArg> = None;

    for input_param in input.sig.inputs.iter() {
        if let syn::FnArg::Typed(pat_type) = input_param {
            if !pat_type.attrs.is_empty() {
                // Check if this parameter has an #[app] attribute
                let has_app_attr = pat_type.attrs.iter().any(|attr| {
                    if let syn::Meta::Path(path) = &attr.meta {
                        path.is_ident("app")
                    } else {
                        false
                    }
                });

                if has_app_attr {
                    // Check if we already found an app parameter
                    if app_param.is_some() {
                        return syn::Error::new_spanned(
                            pat_type,
                            "only one parameter can have the #[app] attribute",
                        )
                        .to_compile_error()
                        .into();
                    }

                    // Store the app parameter name
                    if let syn::Pat::Ident(_) = &*pat_type.pat {
                        app_param = Some(input_param.clone());
                    } else {
                        return syn::Error::new_spanned(
                            pat_type,
                            "#[app] attribute can only be used on simple identifiers",
                        )
                        .to_compile_error()
                        .into();
                    }
                }
            } else if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
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

    let output_schema = if let syn::ReturnType::Type(_, rt) = &fn_output {
        quote! { Some( <#rt as InnerSchema> :: inner_schema(&mut g) ) }
    } else {
        quote! { None }
    };

    let name_expr = &metadata.name_expr;
    let title_expr = &metadata.title_expr;
    let description_expr = &metadata.description_expr;

    // Extract the generic type T from App<T> if app parameter exists
    let (app_param_actual, app_generic_type) = if let Some(syn::FnArg::Typed(pat_type)) = &app_param
    {
        // Extract the type from the parameter
        if let syn::Type::Path(type_path) = &*pat_type.ty {
            // Check if this is App<T> and extract T
            if let Some(last_segment) = type_path.path.segments.last() {
                if last_segment.ident == "App" {
                    if let syn::PathArguments::AngleBracketed(args) = &last_segment.arguments {
                        if args.args.len() == 1 {
                            if let syn::GenericArgument::Type(generic_type) = &args.args[0] {
                                // Successfully extracted T from App<T>
                                // Create a clean parameter without the #[app] attribute
                                if let syn::FnArg::Typed(pat_type) = &app_param.clone().unwrap() {
                                    let clean_pat = &pat_type.pat;
                                    let clean_ty = &pat_type.ty;
                                    (quote! { #clean_pat: #clean_ty }, Some(generic_type.clone()))
                                } else {
                                    unreachable!("app_param was verified to be Typed above")
                                }
                            } else {
                                return syn::Error::new_spanned(
                                    &args.args[0],
                                    "App parameter must have a type argument, not a lifetime or const"
                                ).to_compile_error().into();
                            }
                        } else {
                            return syn::Error::new_spanned(
                                &last_segment.arguments,
                                "App must have exactly one generic parameter",
                            )
                            .to_compile_error()
                            .into();
                        }
                    } else {
                        return syn::Error::new_spanned(
                            &last_segment,
                            "App parameter must be App<T> with a generic type parameter",
                        )
                        .to_compile_error()
                        .into();
                    }
                } else {
                    return syn::Error::new_spanned(
                        &last_segment.ident,
                        "Parameter with #[app] attribute must be of type App<T>",
                    )
                    .to_compile_error()
                    .into();
                }
            } else {
                return syn::Error::new_spanned(
                    &type_path.path,
                    "Parameter with #[app] attribute must be of type App<T>",
                )
                .to_compile_error()
                .into();
            }
        } else {
            return syn::Error::new_spanned(
                &pat_type.ty,
                "Parameter with #[app] attribute must be of type App<T>",
            )
            .to_compile_error()
            .into();
        }
    } else {
        (
            quote! { _app: bakkie::proto::V20250618::App<A> },
            Some(syn::parse_quote!(A)),
        )
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
        #fn_vis async fn #impl_fn_name<A: Send + Sync + 'static>(
                #app_param_actual,
                _def_no_conflict_name_args_123: #struct_name) #fn_output {
            let #struct_name { #(#field_names),* } = _def_no_conflict_name_args_123;
            #fn_body
        }

        // Constructor for this tool's static particulars
        #[allow(non_snake_case)]
        #fn_vis fn #particulars_fn() -> bakkie::provisions::tools::ToolParticulars {

            use ::bakkie::schemars::JsonSchema;
            use ::bakkie::schemars::SchemaGenerator;
            use ::bakkie::schemars::generate::SchemaSettings;
            use ::bakkie::InnerSchema;

            let mut set = SchemaSettings::openapi3();

            set.inline_subschemas = true;

            let mut g = SchemaGenerator::new(set);

            bakkie::provisions::tools::ToolParticulars {
                name: #name_expr,
                title: #title_expr,
                description: #description_expr,
                input_schema: #struct_name :: json_schema(&mut g),
                output_schema: #output_schema,
            }
        }

        // Constructor for the complete tool
        #[allow(non_snake_case)]
        #fn_vis fn #tool_fn_name<A: Send + Sync + 'static>() -> bakkie::provisions::tools::Tool<#app_generic_type> {
            bakkie::provisions::tools::Tool {
                particulars: #particulars_fn(),
                tool_fn: Box::new(|tool_input: bakkie::provisions::tools::ToolInput<#app_generic_type>| {
                    Box::pin(async move {
                        // Parse the input parameters from JSON
                        let args: #struct_name = match serde_json::from_value(
                            serde_json::Value::Object(tool_input.params)
                        ) {
                            Ok(args) => args,
                            Err(e) => Err(ToolError::Json(e))?,
                        };

                        // Call the actual tool function
                        match #impl_fn_name(tool_input.app.clone(), args).await {
                            Ok(result) => Ok(Box::new(result) as Box<dyn bakkie::provisions::tools::AsToolOutput>),
                            Err(e) => Err(ToolError::Internal(Box::new(e))),
                        }
                    })
                }),
            }
        }
    };

    TokenStream::from(output)
}

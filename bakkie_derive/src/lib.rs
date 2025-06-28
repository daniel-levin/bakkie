use proc_macro::TokenStream;

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

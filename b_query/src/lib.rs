use proc_macro_hack::proc_macro_hack;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_hack]
pub fn query(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_hack]
pub fn query2(input: TokenStream) -> TokenStream {
    let q = quote! {
        query!(#input)
    };

    q.into()
}
extern crate proc_macro;

mod imp;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn auto_vec(_: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemFn);

    let compile_err_or_not = imp::auto_vec(&ast).unwrap_or_else(|err| err.to_compile_error());

    let tokens = quote! {
        #ast
        #compile_err_or_not
    };
    tokens.into()
}

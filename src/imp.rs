use proc_macro2::TokenStream;
use syn::{ItemFn, Result};

pub fn auto_vec(_: &ItemFn) -> Result<TokenStream> {
    // let _ = match input.clone() {
    //     Item::Enum(e) => Ok(e.variants),
    //     _ => Err(Error::new(
    //         Span::call_site(),
    //         "expected function expression",
    //     )),
    // }?;

    // let mut previous: Option<syn::Variant> = None;
    // for (i, curr) in variants.iter().enumerate() {
    //     if let Some(ref prev) = previous.replace(curr.clone()) {
    //         if prev.ident.to_string() > curr.ident.to_string() {
    //             let mut least_upper_idx = 0;
    //             for j in i - 1..=0 {
    //                 if variants[j].ident.to_string() < curr.ident.to_string() {
    //                     least_upper_idx = j + 1;
    //                 }
    //             }
    //             return Err(Error::new(
    //                 curr.ident.span(),
    //                 format!(
    //                     "{} should sort before {}",
    //                     curr.ident, variants[least_upper_idx].ident
    //                 ),
    //             ));
    //         }
    //     };
    // }
    Ok(TokenStream::new())
}

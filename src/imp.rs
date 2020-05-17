use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_quote, Block, FnArg, ItemFn, Result, ReturnType};

pub fn auto_vec(item_fn: &ItemFn) -> Result<TokenStream> {
    // eprintln!("item_fn: {:#?}", item_fn);
    let item_fn = item_fn.clone();

    let ident = item_fn.sig.ident;
    let ident_vec = format_ident!("{}_vec", ident);
    let inputs: Punctuated<FnArg, Comma> = item_fn.sig.inputs.iter().map(vectorize_arg).collect();
    let output = vectorized_return_type(&item_fn.sig.output);
    let block = vectorized_body(&ident, &item_fn.sig.inputs);
    let tokens = quote! {
        fn #ident_vec (#inputs) #output #block
    };

    // eprintln!("tokens: {}", tokens);
    Ok(tokens)
}

fn vectorize_arg(arg: &FnArg) -> FnArg {
    if let FnArg::Typed(pat_type) = arg {
        let pat = pat_type.pat.clone();
        let ty = *pat_type.ty.clone();
        parse_quote! {
            #pat: Vec<#ty>
        }
    } else {
        arg.clone()
    }
}
fn vectorized_return_type(return_type: &ReturnType) -> ReturnType {
    if let ReturnType::Type(_, ty) = return_type {
        parse_quote! {
            -> Vec<#ty>
        }
    } else {
        panic!("cannot handle default return type")
    }
}

fn vectorized_body(fn_ident: &proc_macro2::Ident, args: &Punctuated<FnArg, Comma>) -> Block {
    let arg_list: Vec<proc_macro2::TokenStream> = args
        .iter()
        .map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                let pat = pat_type.pat.clone();
                quote! {
                    #pat[_i]
                }
            } else {
                quote! {}
            }
        })
        .collect();
    let arg1_pat = if let FnArg::Typed(ref pat_type) = args[0] {
        pat_type.pat.clone()
    } else {
        panic!("cannot handle fn args")
    };
    parse_quote! {
        {
            let len = #arg1_pat.len();
            let mut result = Vec::with_capacity(len);
            for _i in 0..len {
                let returned = #fn_ident(#(#arg_list),*);
                result.push(returned);
            }
            result
        }
    }
}

//! # Hints
//!
//! - Parse the `item` token stream into an `ItemFn` AST node using `syn`
//! - Check `quote`'s documentation to learn its macro syntax
use proc_macro::TokenStream as TkStream;
// use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Attribute, ItemFn};

#[proc_macro_attribute]
pub fn vanilla_test(_args: TkStream, input: TkStream) -> TkStream {
    let test_fn: ItemFn = syn::parse(input).unwrap();

    if test_fn
        .attrs
        .iter()
        .find(|a| is_test_attribute(a))
        .is_some()
    {
        test_fn.to_token_stream()
    } else {

        // apply test attribute
        let func_tokens = test_fn.into_token_stream();
        // let test_attrib = syn::Attribute::
        quote::quote! {            
            #[test]
            #func_tokens
        }
        //todo!("Use quote");
    }
    .into()
}

fn is_test_attribute(attr: &Attribute) -> bool {
    attr.path().get_ident().map_or(false, |id| id.eq("test"))
}

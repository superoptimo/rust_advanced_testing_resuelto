use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Attribute, ItemFn, Token};

#[proc_macro_attribute]
pub fn test(args: TokenStream, input: TokenStream) -> TokenStream {
    let test_fn: ItemFn = syn::parse_macro_input!(input as ItemFn);
    let Args { before, after } = syn::parse_macro_input!(args as RawArgs).validate();

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = test_fn;

    let block_stmts = block.stmts;

    let mut output = quote::quote! {
        #(#attrs)*
        #vis #sig
        {
            // before callings
            #(
                #before();
            )*

            // original block
            #(#block_stmts)*

            // after callings
            #(
                #after();
            )*
        }
    };

    if !attrs.iter().any(|a| is_test_attribute(a)) {
        output = {
            quote! {
                #[::core::prelude::v1::test]
                #output
            }
        };
    }
    output.into()
}

struct RawArgs {
    vars: Vec<RawHook>,
}

/// Argument parsing goes through two phases:
///
/// 1. Parse the raw arguments into a struct, which is syntactically what we expect
/// 2. Validate the arguments and convert them into the form we want (semantic validation)
struct Args {
    before: Vec<syn::Path>,
    after: Vec<syn::Path>,
}

impl RawArgs {
    pub fn validate(self) -> Args {
        let mut before_args:Vec<syn::Path> = Vec::with_capacity(1);
        let mut after_args:Vec<syn::Path> = Vec::with_capacity(1);

        for varg in self.vars.iter() {
            if varg.type_.eq("before") {
                before_args.push(varg.fn_path.clone());
            }
            else if varg.type_.eq("after")
            {
                after_args.push(varg.fn_path.clone());
            }
        }

        Args{
            before: before_args, after: after_args
        }
    }
}

struct RawHook {
    type_: syn::Ident,
    _equals: Token![=],
    fn_path: syn::Path,
}

impl Parse for RawArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let vars = Punctuated::<RawHook, Token![,]>::parse_terminated(input)?;
        Ok(RawArgs {
            vars: vars.into_iter().collect(),
        })
    }
}

impl Parse for RawHook {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed_type = input.parse()?;
        let sep = input.parse()?;
        let parsed_path = input.parse()?;
        Ok(Self{type_: parsed_type, _equals: sep, fn_path:parsed_path})
    }
}

fn is_test_attribute(attr: &Attribute) -> bool {
    let last_segment = match attr.path().segments.last() {
        Some(last_segment) => last_segment,
        None => return false,
    };
    last_segment.ident == "test"
}

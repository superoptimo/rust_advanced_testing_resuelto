
use proc_macro::TokenStream as TkStream;

#[proc_macro_attribute]
pub fn vanilla_test(_attribs:TkStream, item:TkStream) -> TkStream
{
    // pass thru no-op
    item
}
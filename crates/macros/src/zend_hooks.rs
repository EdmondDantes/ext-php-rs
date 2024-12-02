use proc_macro2::{Span, TokenStream};
use syn::{AttributeArgs, ItemFn};
use crate::function;
use crate::function::Function;

pub fn parse_function_hook(args: AttributeArgs, input: ItemFn) -> TokenStream {
    let (token_stream, zend_function) = match function::parser(args, input, None) {
        Ok((parsed, zend_function)) => (parsed, Some(zend_function)),
        Err(e) => (syn::Error::new(Span::call_site(), e).to_compile_error(), None),
    };

    if zend_function.is_none() {
        return token_stream;
    };

    token_stream
}
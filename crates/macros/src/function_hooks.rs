use std::collections::HashMap;
use anyhow::{bail, Result};
use darling::FromMeta;
use proc_macro2::{Span, TokenStream};
use syn::{AttributeArgs, ItemFn, Lit};
use crate::function;
use quote::quote;

#[derive(Default, Debug, FromMeta)]
#[darling(default)]
pub struct AttrArgs {
    optional: Option<String>,
    ignore_module: bool,
    defaults: HashMap<String, Lit>,
    name: Option<String>,
}

pub fn parse_function_hook(args: AttributeArgs, input: ItemFn) -> Result<TokenStream> {

    let attr_args = match AttrArgs::from_list(&args) {
        Ok(args) => args,
        Err(e) => bail!("Unable to parse attribute arguments: {:?}", e),
    };

    // Default hooked function name is the name of the function in the source code
    // This can be overridden by the `name` attribute argument
    let default_function_name = input.sig.ident.to_string();

    match function::parser(args, input, None) {
        Ok((token_stream, zend_function)) => {

            let hooked_function_name = attr_args.name.unwrap_or_else(|| default_function_name);

            let ident = syn::Ident::new(&zend_function.ident, Span::call_site());

            let add_hook_code = quote! {

                #token_stream

                ::ext_php_rs::hooks::add_function_hook(::ext_php_rs::hooks::ZendFunctionHook {
                    hooked_function_name: #hooked_function_name,
                    handler: #ident,
                    previous_handler: None,
                });
            };

            Ok(add_hook_code)
        }
        Err(e) => Err(e),
    }.into()
}
use std::collections::HashMap;
use anyhow::{bail, Result};
use darling::FromMeta;
use proc_macro2::{Span, TokenStream};
use syn::{AttributeArgs, ItemFn, Lit, Signature};
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

pub fn parse_function_hook(args: AttributeArgs, input: ItemFn) -> Result<(TokenStream)> {

    let attr_args = match AttrArgs::from_list(&args) {
        Ok(args) => args,
        Err(e) => bail!("Unable to parse attribute arguments: {:?}", e),
    };

    let ItemFn { sig, .. } = &input;
    let Signature {
        ident,
        output,
        inputs,
        ..
    } = &sig;

    match function::parser(args, input, None) {
        Ok((token_stream, zend_function)) => {

            let hooked_function_name = match attr_args.name {
                None => ident.to_string(),
                Some(name) => name
            };

            let add_hook_code = quote! {

                #token_stream

                ::ext_php_rs::hooks::add_function_hook(::ext_php_rs::hooks::ZendFunctionHook {
                    hooked_function_name: #hooked_function_name,
                    handler: #zend_function.ident,
                    previous_handler: None,
                });
            };

            Ok(add_hook_code)
        }
        Err(e) => Err(e),
    }.into()
}
use std::collections::HashMap;
use anyhow::{bail, Result};
use darling::FromMeta;
use proc_macro2::{Span, TokenStream};
use syn::{AttributeArgs, ItemFn, Lit};
use crate::function;
use quote::quote;
use std::sync::RwLock;
use lazy_static::lazy_static;

#[derive(Default, Debug, FromMeta)]
#[darling(default)]
pub struct AttrArgs {
    optional: Option<String>,
    ignore_module: bool,
    defaults: HashMap<String, Lit>,
    name: Option<String>,
}

lazy_static! {
    static ref FUNCTION_HOOKS: RwLock<HashMap<String, ZendFunctionHook>> = RwLock::new(HashMap::new());
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
            let previous_name = format!("PREVIOUS_{}", hooked_function_name.to_uppercase());

            // This static variable will store the value of the previous Zend function handler.
            // You will be able to access this variable
            // from the hook function by name: `PREVIOUS_` + `hooked_function_name`
            let previous_ident = syn::Ident::new(&previous_name, Span::call_site());

            let add_hook_code = quote! {

                #token_stream

                thread_local! {
                    pub static #previous_ident: RefCell<Option<FunctionHandler>> = RefCell::new(None);
                }
            };

            /*
                ::ext_php_rs::hooks::add_function_hook(::ext_php_rs::hooks::ZendFunctionHook {
                    hooked_function_name: #hooked_function_name,
                    handler: #ident,
                    previous_handler: None,
                });
            */

            println!("add_hook_code: {:?}", add_hook_code.to_string());

            Ok(add_hook_code)
        }
        Err(e) => Err(e),
    }.into()
}
#![allow(dead_code)]

use std::cell::RefCell;
use crate::flags::FunctionType;
use crate::zend::{ExecuteData, Function};
use crate::ffi::{zif_handler};
use crate::types::Zval;

/// Function representation in Rust.
#[cfg(not(windows))]
pub type FunctionHandler = extern "C" fn(execute_data: &mut ExecuteData, retval: &mut Zval);
#[cfg(windows)]
pub type FunctionHandler =
extern "vectorcall" fn(execute_data: &mut ExecuteData, retval: &mut Zval);

fn from_zif_handler(handler: zif_handler) -> Result<FunctionHandler, &'static str> {
    if let Some(zif) = handler {
        Ok(unsafe { std::mem::transmute(zif) })
    } else {
        Err("zif_handler is None")
    }
}

fn to_zif_handler(handler: FunctionHandler) -> zif_handler {
    Some(unsafe { std::mem::transmute(handler) })
}

#[derive(Clone)]
pub struct ZendFunctionHook {
    pub hooked_function_name: String,
    pub handler: FunctionHandler,
    pub previous_handler: FunctionHandler,
}

thread_local! {
    static FUNCTION_HOOKS: RefCell<Vec<ZendFunctionHook>> = RefCell::new(Vec::new());
}

///
/// Add a hook to the list of hooks
///
pub fn add_function_hook(hook: ZendFunctionHook) {
    FUNCTION_HOOKS.with(|hooks| {
        hooks.borrow_mut().push(hook);
    });
}

///
/// Get the list of hooks
///
pub fn get_function_hooks() -> Vec<ZendFunctionHook> {
    FUNCTION_HOOKS.with(|hooks| {
        hooks.borrow().clone()
    })
}

///
/// Remove all hooks
///
pub fn remove_all_function_hooks() {
    FUNCTION_HOOKS.with(|hooks| {
        hooks.borrow_mut().clear();
    });
}

pub fn setup_function_hooks() {
    FUNCTION_HOOKS.with(|hooks| {
        for hook in hooks.borrow_mut().iter_mut() {
            if let Ok(Some(previous_handler)) = hook_function(hook.handler, &hook.hooked_function_name) {
                hook.previous_handler = previous_handler;
            }
        }
    });
}

pub fn remove_function_hooks() {
    FUNCTION_HOOKS.with(|hooks| {
        for hook in hooks.borrow().iter() {
            hook_function(hook.previous_handler, &hook.hooked_function_name).unwrap();
        }
    });
}

fn hook_function(handler: FunctionHandler, func_name: &str) -> Result<Option<FunctionHandler>, Box<dyn std::error::Error>> {

    let mut zend_function = Function::try_from_function(func_name)
        .ok_or_else(|| format!("The function '{}' was not found in Zend.", func_name))?;

    if zend_function.function_type() != FunctionType::Internal {
        return Err(format!("Function '{}' is not an internal function.", func_name).into());
    }

    zend_function.internal_function.handler = to_zif_handler(handler);

    Ok(unsafe {
        zend_function.internal_function.handler
            .map(|existing_handler| {
                from_zif_handler(Some(existing_handler)).expect("Failed to convert handler")
            })
    }
    )
}

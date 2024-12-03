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

#[derive(Clone)]
pub struct ZendFunctionHook {
    pub hooked_function_name: String,
    pub handler: zif_handler,
    pub previous_handler: zif_handler,
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
        for hook in hooks.borrow().iter() {
            hook_function(hook.handler, &hook.hooked_function_name).unwrap();
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

fn hook_function(handler: zif_handler, func_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Ищем функцию в глобальной таблице
    let mut zend_function = Function::try_from_function(func_name)
        .ok_or_else(|| format!("The function '{}' was not found in Zend.", func_name))?;

    if zend_function.function_type() != FunctionType::Internal {
        return Err(format!("Function '{}' is not an internal function.", func_name).into());
    }

    unsafe {
        // Сохраняем оригинальный обработчик, если нужно
        let _original_handler = zend_function.internal_function.handler;

        // Подменяем обработчик на новый
        zend_function.internal_function.handler = handler;
    }

    Ok(())
}

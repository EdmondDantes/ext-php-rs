use std::cell::RefCell;
use std::ffi::c_void;
use crate::{types, zend};
use crate::zend::{Function, ExecutorGlobals};

#[derive(Clone)]
pub struct ZendFunctionHook {
    pub hooked_function_name: String,
    pub handler: extern "C" fn(
        ex: &mut zend::ExecuteData,
        return_value: &mut types::Zval,
    ),
    pub previous_handler: Option<*const c_void>,
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
            hook_function(hook.handler as *const c_void, &hook.hooked_function_name).unwrap();
        }
    });
}

pub fn remove_function_hooks() {
    FUNCTION_HOOKS.with(|hooks| {
        for hook in hooks.borrow().iter() {
            hook_function(hook.previous_handler.unwrap(), &hook.hooked_function_name).unwrap();
        }
    });
}

fn hook_function(func_ptr: *const c_void, func_name: &str) -> Result<(), Box<dyn std::error::Error>> {

    // Получаем глобальную таблицу функций
    let function_table = ExecutorGlobals::get()
        .function_table()
        .expect("Error: function table is not available");

    if let Some(zval_function) = function_table.get(func_name) {
        let mut zend_function = Function::from(zval_function.as_ptr());

        unsafe {
            zend_function.internal_function.handler = Some(func_ptr);
        }

    } else {
        return Err(print!("The function '{}' was not found in Zend.", func_name).into());
    }

    Ok(())
}

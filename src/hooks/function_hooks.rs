use std::cell::RefCell;
use std::ffi::c_void;
use crate::{types, zend};

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
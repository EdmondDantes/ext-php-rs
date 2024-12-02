use std::ffi::c_void;

pub struct ZendFunctionHook {
    pub hooked_function_name: String,
    pub handler: extern "C" fn(
        ex: &mut ::ext_php_rs::zend::ExecuteData,
        return_value: &mut ::ext_php_rs::types::Zval,
    ),
    pub previous_handler: Option<*const c_void>,
}

thread_local! {
    static FUNCTION_HOOKS: RefCell<Vec<ZendFunctionHook>> = RefCell::new(Vec::new());
}

///
/// Add a hook to the list of hooks
///
pub fn add_hook(hook: ZendFunctionHook) {
    FUNCTION_HOOKS.with(|hooks| {
        hooks.borrow_mut().push(hook);
    });
}

///
/// Get the list of hooks
///
pub fn get_hooks() -> Vec<ZendFunctionHook> {
    FUNCTION_HOOKS.with(|hooks| {
        hooks.borrow().clone()
    })
}

///
/// Remove all hooks
///
pub fn remove_all_hooks() {
    FUNCTION_HOOKS.with(|hooks| {
        hooks.borrow_mut().clear();
    });
}
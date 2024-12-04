//!
//! The module implements interaction with ZEND HOOKS
//! and allows modifying the behavior of functions.
//!

pub mod function_hooks;
pub use function_hooks::FunctionHandler;
pub use function_hooks::ZendFunctionHook;
pub use function_hooks::add_function_hook;
pub use function_hooks::remove_function_hooks;
pub use function_hooks::get_function_hook;
pub use function_hooks::remove_function_hook;
pub use function_hooks::remove_all_function_hooks;
pub use function_hooks::setup_function_hooks;
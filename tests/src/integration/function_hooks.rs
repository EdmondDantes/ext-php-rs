#[test]
fn hooks_works() {
    assert!(crate::integration::run_php("function_hooks.php"));
}

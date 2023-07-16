rust
fn foo() { /* do something */ }

fn pure_function() {
    // Look, I constructed an instance of `foo` from nothing, just its name.
    let my_fn_instance = foo;
    my_fn_instance();
}

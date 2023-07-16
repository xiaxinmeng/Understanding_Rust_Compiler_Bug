rust
// Say I want to use `extern crate foo` in this function without declaring it anywhere outside.
fn f() {
    extern crate foo; // I could declare it here, but I can't `use` it (yet...)
    mod foo {
        extern crate foo; // If I want `use`, I need to declare it in a module.
    }
}

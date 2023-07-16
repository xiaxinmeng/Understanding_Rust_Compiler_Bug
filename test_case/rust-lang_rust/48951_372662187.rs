rust
// Whoops, we changed the type signature between versions
fn function_that_returns_bar() {
}

let my_thing: Foo = function_that_returns_bar().into();

 rust
#[lua_safety]
fn foo() {
    //...
}
// expands to...
fn foo() {
    fn inner_foo() {
        //...
    }
    // LUA setup
    inner_foo();
    // LUA cleanup
}

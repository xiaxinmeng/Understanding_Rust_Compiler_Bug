 Rust
struct Foo;

fn foo(handle: &mut Foo) {
    unsafe {
        let temp: *mut Foo = &mut *handle;
        *temp;
    }
}

// This also causes the ICE.
/*
fn foo_immut(handle: & Foo) {
    unsafe {
        let temp: *const Foo = &*handle;
        *temp;
    }
}
*/
fn main() {

}

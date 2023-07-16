rust
#![feature(bindings_after_at)]

fn foo() {
    let ref mut _z @ ref _a = &mut 1;
    **_z = 0;
}

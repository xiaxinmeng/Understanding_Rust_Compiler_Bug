rust
#![feature(const_fn)]

const fn foo() {
    match 0 { // error[E0019]: constant function contains unimplemented expression type
        1 => {}
        _ => {}
    };
}

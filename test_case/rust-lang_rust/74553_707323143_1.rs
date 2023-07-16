rust
#![feature(const_fn_pointer)]
#![feature(const_fn_fn_ptr_basics)]

const fn foo() { }
const fn bar() { }
fn baz() { }

const fn bazz(f: const fn()) {
    f();
}


fn main() {
    let x = match 2 {
        2 => foo,
        3 => bar,
        _ => baz
    };
}


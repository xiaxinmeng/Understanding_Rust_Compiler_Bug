rust
#![feature(generators)]

fn foo() {
    let x = static || {
        let mut s = String::new();
        s += { yield; "" };
    };
}

fn main() {
    foo()
}

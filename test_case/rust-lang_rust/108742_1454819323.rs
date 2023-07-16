
#![feature(non_lifetime_binders)]

fn foo() where for<T> T: Copy {}

fn main() {
    foo();
}

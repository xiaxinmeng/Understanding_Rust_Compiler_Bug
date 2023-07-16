rust
#![allow(dead_code)]

#[must_use]
struct A;

trait T {}

impl T for A {}

fn f() -> impl T { A }

fn g() {
    f();
}

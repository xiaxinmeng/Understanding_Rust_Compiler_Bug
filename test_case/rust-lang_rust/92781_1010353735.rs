
$ cat a.rs 
mod machine {
        pub struct A {
            pub b: B,
        }
        pub struct B {}
        impl B {
            pub fn f(&self) {}
        }
}

pub struct Context {
    pub a: machine::A,
}

pub fn ctx() -> Context {
    todo!();
}
$ cat b.rs 
fn main() {
    a::ctx().a.b.f();
}
$ rustc +stage1 --edition=2018 --crate-type=lib -O a.rs
warning: associated function is never used: `f`
 --> a.rs:7:20
  |
7 |             pub fn f(&self) {}
  |                    ^
  |
  = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted

$ rustc +stage1 --edition=2018 --crate-type=bin -O b.rs --extern a -L.
error: internal compiler error: compiler/rustc_monomorphize/src/collector.rs:966:9: no MIR available for DefId(20:8 ~ a[c3e7]::machine::{impl#0}::f)

rust
#![feature(type_alias_impl_trait)]

type Foo = impl for<'a> FnOnce(&'a str) -> usize;
type Bar = impl FnOnce(&'static str) -> usize;

// these two work
fn foo() -> Foo { if true { |s| s.len() } else { panic!() } }
fn bar() -> Bar { if true { |s| s.len() } else { panic!() } }

// these two don't work
fn foo2() -> impl for<'a> FnOnce(&'a str) -> usize { if true { |s| s.len() } else { panic!() } }
fn bar2() -> impl FnOnce(&'static str) -> usize { if true { |s| s.len() } else { panic!() } }

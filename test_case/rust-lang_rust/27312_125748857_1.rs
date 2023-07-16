 rust
#![feature(default_type_parameter_fallback)]
fn foo<T=u64>(t: T) { println!("{}", std::mem::size_of_val(&t)); }
fn main() { foo(22); }

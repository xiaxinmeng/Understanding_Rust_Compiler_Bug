 rust
#![allow(dead_code)]
struct A<'l> { _a: &'l i32 }

fn call<'m, T>(_functions: &'m Vec<for <'n> fn(&'n T)>) { }

#[cfg(wont_compile_due_to_type_mismatch)]
fn caller1<'o>(vec: &'o Vec<for <'s> fn(&'s A<'s>)>) { call(vec); }

fn caller2<'t>(vec: &'t Vec<for <'u,'v> fn(&'u A<'v>)>) { call(vec); }

fn main() { }

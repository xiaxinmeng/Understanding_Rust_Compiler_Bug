
#![feature(staged_api)]
#![staged_api]

#[deprecated(since = "1.6.0", reason = "")]
#[stable(since = "1.6.0", feature="lol")]
fn foo() { }

#[deprecated(since = "1.6.0", reason = "")]
#[stable(since = "1.6.0", feature="lol")]
fn bar() { foo() }

fn main() {}

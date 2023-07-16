rust
#![feature(type_alias_impl_trait)]
trait TA {}
impl TA for () {} 
type A = impl TA;
fn foo() -> A { } // defining use
fn foo1() -> A { foo() } // not a defining use

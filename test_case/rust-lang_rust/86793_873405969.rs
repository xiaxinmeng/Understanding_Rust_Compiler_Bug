rust
#![feature(min_type_alias_impl_trait)]

trait Trait {}
struct S;
impl Trait for S {}
fn foo() -> impl Trait { S }

type Concrete = impl Trait;
fn bar() -> Concrete { foo() }

fn main() {
    let mut v = Vec::<Concrete>::new();
    v.push(foo());
}

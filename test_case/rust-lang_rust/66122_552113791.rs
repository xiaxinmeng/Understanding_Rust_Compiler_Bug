rust
#![feature(optin_builtin_traits)]

trait Trait {
    fn foo(&self) where Self: Auto {}
}
impl Trait for () {}

auto trait Auto {}
trait Nobody {}
impl<T> Auto for T where T: Nobody {}
impl Auto for dyn Trait {}

fn main() {
    <dyn Trait>::foo(&())
}

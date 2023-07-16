rust
trait Tr {}
struct S {}

impl Tr for &S {}
impl Tr for &mut S {}

fn foo<T: Tr>(t: T) {}

fn main() {
    let s = S {};
    foo(s);
}

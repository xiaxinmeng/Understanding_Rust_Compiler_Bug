rust
    trait Goo {}
impl Goo for () {}
fn main() -> impl Goo {}

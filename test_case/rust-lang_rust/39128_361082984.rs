rust
trait Trait {}

impl PartialEq for Box<Trait> {
    fn eq(&self, _: &Box<Trait>) -> bool {
        false
    }
}

#[derive(PartialEq)]
struct S(Box<Trait>);

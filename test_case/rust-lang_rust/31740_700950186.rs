 rust
pub trait Trait {}

/* This impl is not sufficient and causes "the cannot move out of `*__self_1_0` which is behind a shared reference" error.  */
impl PartialEq for Box<dyn Trait> {
    fn eq(&self, _other: &Self) -> bool {
        todo!();
    }
}

/* Code fails to compile without this impl. */
impl PartialEq<&Self> for Box<dyn Trait> {
    fn eq(&self, _other: &&Self) -> bool {
        todo!();
    }
}

#[derive(PartialEq)]
struct Foo {
    x: Box<dyn Trait>
}

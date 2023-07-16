rust
mod A {
    pub trait Trait {}
}

mod B {
    struct A<H: A::Trait>(H);
}

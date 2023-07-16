rust
pub trait Trait1 {
    fn f();
}

pub trait Trait2 {
    type Type: Trait1;
}

pub trait Trait3 {
    type Type2: Trait2<Type = Self>;
    fn f2() {
        <Self as Trait1>::f();
    }
}

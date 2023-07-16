rust
pub trait Trait2 {
    type Type2;
}

pub trait Trait3 {
    type Type3;
}

pub struct Question {
    pub ins: <<usize as Trait3>::Type3 as Trait2>::Type2,
}

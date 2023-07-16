rust
pub trait SomeTrait {
    fn one(&self);
    fn two(&self);
}

pub struct SomeStruct;

impl SomeTrait for SomeStruct {
    /// Here are some impl-specific docs!
    fn one(&self) {}

    // (note that this function is not specially documented!)
    fn two(&self) {}
}

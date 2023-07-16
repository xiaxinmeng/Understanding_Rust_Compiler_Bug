rust
pub trait Trait1 {
    /// From Trait1
    fn trait_func1();
}

pub trait Trait2 {
    /// From Trait2
    fn trait_func2();
}

/// This is a MyStruct.
pub struct MyStruct;

impl MyStruct {
    /// do_stuff() with MyStruct
    pub fn do_stuff(&self) {}
}

impl Trait1 for MyStruct {
    fn trait_func1() {}
}

/// This is a MyAlias
pub type MyAlias = MyStruct;

impl MyAlias {
    /// do_more_stuff() with MyAlias
    pub fn do_more_stuff(&self) {}
}

impl Trait2 for MyAlias {
    fn trait_func2() {}
}

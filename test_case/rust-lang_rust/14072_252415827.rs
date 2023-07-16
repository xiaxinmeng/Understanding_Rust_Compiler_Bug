 rust
#![crate_type = "lib"]

pub trait Foo<'a> where Self: Sized {
    fn supply_ref(self) -> &'a u32;

    fn get_ref(self) -> &'a u32 {
        self.supply_ref()
    }
}

pub struct Wrapper<'a> {
    val: &'a u32
}

impl <'a, 'b: 'a> Foo<'b> for &'a Wrapper<'b> {
    fn supply_ref(self) -> &'b u32 {
        &self.val
    }
}

rust
// compile-flags: -Zsymbol-mangling-version=v0 -Cno-prepopulate-passes 

pub fn test<T>() {}

fn main() {
    // CHECK: ; call a::test::<&dyn for<'a> core::ops::function::FnMut<(&'a u8,), Output = ()>>
    test::<&dyn FnMut(&u8)>();
    // CHECK: ; call a::test::<for<'a> fn(&'a dyn for<'b> core::ops::function::FnOnce<(&'b u8,), Output = &'b u8> + 'a) -> &'a u8>
    test::<for<'a> fn(&'a dyn for<'b> FnOnce(&'b u8) -> &'b u8) -> &'a u8>();
}

rust
pub fn foo<'a, T: 'a>() {}

pub fn bar<'a, 'b>() {
    foo::<'a, fn(&'a u32)>(); // Passes
}

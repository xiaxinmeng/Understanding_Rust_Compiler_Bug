rust
const fn foo<'a>(x: &'a mut i32) -> &'a mut i32 {
     let mut y = 42;
     let z = &mut y;
     *z = 99;
     x
}

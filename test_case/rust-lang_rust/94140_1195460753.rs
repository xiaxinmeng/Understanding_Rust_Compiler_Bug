rust
pub type RefFn<'a> = &'a dyn for<'b> Fn(&'a i32) -> i32;

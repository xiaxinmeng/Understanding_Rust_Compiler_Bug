rust
fn or<'a>(first: &'static (dyn Fn(&'a i32))) -> dyn Fn(&'a i32) {
   return Box::new(panic!());
}

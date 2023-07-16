rust
// borrow checker properly handles this too.
// does not compile
// fn my<T: 'static>() {
//    let foo1: &dyn for<'a, 'b> FnOnce(&'a (), &'b T) -> (&'a &'b (), &'a T) = todo!();
//    let foo2: &dyn for<'b> FnOnce(&'static (), &'b T) -> (&'static &'b (), &'static T) = foo1;
//    let foo3: &dyn for<'b> FnOnce(&'static (), &'b T) -> (&'b &'b (), &'static T) = foo2;
//}

rust
a(x: A) {
   core::ptr::drop_in_place::<A>(&x);
}

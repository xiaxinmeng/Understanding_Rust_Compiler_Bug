 rust
fn foobar(x: &mut i32) {
  let zz : *mut _;
  { let y = x; // reborrow for the scope of y
    let z = y as *mut _;
    *z = 15;
    zz = *z;
   }
   // How is the compiler supposed to know that `x` can now alias? Or is thus UB?
   *x = 14;
   *zz = 13;
   *x
}

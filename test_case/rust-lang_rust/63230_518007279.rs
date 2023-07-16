rust
fn foo<T>(mk_T: impl FnOnce () -> T) -> (u32, T) {
  let x: (u32, T);
  x.0 = 42;
  x.1 = mk_T();
  x
}

foo::<!>(|| panic!())

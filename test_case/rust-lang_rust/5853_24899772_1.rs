 rust

extern "C++" {
  #[class = "S"]
  struct Simpl { x: int, }

  #[class = "S"]
  trait Stable {
    fn foo(*self),
  }
  impl Simpl { fn bar(*self); }
  impl Stable for Simpl { fn foo(*self); }

  #[class = "T"]
  struct Timpl { super_: Simpl, y: int, }
  impl Stable for Timpl { fn foo(*self); }
  impl Timpl { fn baz(*self); }

  fn process<S:Stable>(arg: *S);
  fn getS() -> *Simpl;
}

rust
  trait T { fn f(&self) -> bool; }
  struct A(u32);
  impl T for A { fn f(&self) -> bool { false } }
  struct B(A);
  impl T for B { fn f(&self) -> bool { true } }

  fn main() {
      let b = B(A(17));
      let p: &dyn T = &b.0;
      let q: &dyn T = &b;
      assert_eq!(p as *const dyn T as *const u8, q as *const dyn T as *const u8);
      assert_ne!(p.f(), q.f());
  }
  
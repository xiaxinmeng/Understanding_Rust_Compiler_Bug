 rust
impl Fn(isize) for A {
  extern "rust-call" fn call(&self, args: (isize,)) {
    Fn::<(isize,isize)>::call(self, (args.0,args.0))
  }
}

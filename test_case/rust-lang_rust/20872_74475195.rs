 rust
struct A;
impl Fn(isize,isize) for A {
  extern "rust-call" fn call(&self, args: (isize,isize)) {
    ....
  }
}
impl Fn(isize) for A {
  extern "rust-call" fn call(&self, args: (isize,)) {
    self(args.0, args.0) 
    // error: the type of this value must be known in this context
    // self(args.0 args.0)
    // error: cannot use call notation; the first type parameter for the function trait is neither a tuple nor unit [E0059]
  }
}

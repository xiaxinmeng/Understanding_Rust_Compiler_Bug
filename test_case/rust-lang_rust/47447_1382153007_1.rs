rust
impl A {
    fn foo() -> Self {
      /*Self::*/bar()
    }
    
    fn bar0() -> Self { A { } }
    fn bar1() -> u32 { 0 }  // mismatching-types
    fn bar2() -> Self { A { } }
}

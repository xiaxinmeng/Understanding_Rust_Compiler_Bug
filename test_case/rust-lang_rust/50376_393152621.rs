rust
macro_rules! m {
  ($x:ident) => {
    {
      use $x::bar;
      bar();
    }
  }
}

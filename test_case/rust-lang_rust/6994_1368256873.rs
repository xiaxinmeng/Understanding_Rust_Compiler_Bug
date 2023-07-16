rust
macro_rules! code_macro {
  ($gen:ident, $res:ident) => {
    macro_rules! $res {
      ($($token:tt)*) => {
        http_code!($gen $($token)*)
      };
    }
  };
  ($closure:tt, $res:ident) => {
    let f = $closure;
    code_macro!(f, $res)
  };
}

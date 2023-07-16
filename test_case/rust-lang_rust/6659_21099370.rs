
macro_rules! c_to_rust(
  ($v:ident c_int:ty) => ($v);
  ($v:ident $(*c_char):ty) => (str::raw::from_c_str($v));
)


macro_rules! c_to_rust(
  ($v:ident $t:ty) if $t match c_int => ($v);
  ($v:ident $t:ty) if $t match *c_char => (str::raw::from_c_str($v));
)

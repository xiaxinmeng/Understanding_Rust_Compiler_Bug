
warning: found non-foreign-function-safe member in struct marked #[repr(C)]: found struct without foreign-function-safe representation annotation in foreign module, consider adding a #[repr(C)] attribute to the type, #[warn(improper_ctypes)] on by default
 --> <anon>:9:19
9 |>     pub fn f(foo: *mut Foo);
  |> 

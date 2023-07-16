
warning: found non-foreign-function-safe member in struct marked #[repr(C)]: found struct without foreign-function-safe representation annotation in foreign module, consider adding a #[repr(C)] attribute to the type, #[warn(improper_ctypes)] on by default
  --> src/main.rs:19:20
   |
19 |     fn bogus(data: *const Data);
   |                    ^^^^^^^^^^^

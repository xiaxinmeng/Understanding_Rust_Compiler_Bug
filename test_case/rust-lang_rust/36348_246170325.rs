
$ cat test.rs
extern "stdcall" fn extern_stdcall_fn_with_return_value(_: Box<isize>) -> usize { 0 }
fn main() {}

$ rustc test.rs 
test.rs:1:1: 1:86 warning: function is never used: `extern_stdcall_fn_with_return_value`, #[warn(dead_code)] on by default
test.rs:1 extern "stdcall" fn extern_stdcall_fn_with_return_value(_: Box<isize>) -> usize { 0 }
          ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
Segmentation fault

rust
> use std::ffi::CString;
> use std::os::raw::c_char;
> 
> extern "C" {
>     fn my_printer(s: *const c_char);
> }
> 
> // We are certain that our string doesn't have 0 bytes in the middle,
> // so we can .expect()
> let c_to_print = CString::new("Hello, world!").expect("CString::new failed");
> unsafe {
>     my_printer(c_to_print.as_ptr());
> }
> 
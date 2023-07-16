 rust
#[no_mangle] // char *mylib_do_stuff(void);
extern fn mylib_do_stuff() -> *mut RustOwnedChar {...}
#[no_mangle] // void mylib_free(char *s);
extern fn mylib_free(s: *mut RustOwnedChar) {...}

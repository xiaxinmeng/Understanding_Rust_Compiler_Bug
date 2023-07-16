plain
    Checking addr2line v0.16.0
error[E0423]: expected function, found module `memchr`
    --> library/std/src/ffi/c_str.rs:1231:23
     |
1231 |         let nul_pos = memchr(b'\0', bytes);
     |
help: consider importing one of these items instead
     |
6    | use core::slice::memchr::memchr;

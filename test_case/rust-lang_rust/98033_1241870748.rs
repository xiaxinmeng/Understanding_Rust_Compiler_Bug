plain
    Checking hashbrown v0.12.3
    Checking miniz_oxide v0.4.0
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking addr2line v0.16.0
error[E0603]: module `fd` is private
    |
2   | use crate::os::fd::owned::AsFd;
    |                ^^ private module
    |
    |
note: the module `fd` is defined here
   --> library/std/src/os/mod.rs:150:1
    |
150 | mod fd;
    | ^^^^^^^

error[E0603]: module `fd` is private
    |
3   | use crate::os::fd::raw::AsRawFd;
    |                ^^ private module
    |

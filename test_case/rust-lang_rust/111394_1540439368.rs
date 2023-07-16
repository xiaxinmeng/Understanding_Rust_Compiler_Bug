plain
    Checking hashbrown v0.13.1
    Checking object v0.30.1
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking addr2line v0.19.0
error[E0425]: cannot find function, tuple struct or tuple variant `WSAStartup` in module `c`
   |
   |
39 |         let ret = c::WSAStartup(
   |                      ^^^^^^^^^^ not found in `c`
For more information about this error, try `rustc --explain E0425`.
error: could not compile `std` (lib) due to previous error
Build completed unsuccessfully in 0:00:15

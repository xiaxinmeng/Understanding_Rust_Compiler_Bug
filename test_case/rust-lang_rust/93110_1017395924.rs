plain
[RUSTC-TIMING] addr2line test:false 0.403
[RUSTC-TIMING] core test:false 20.832
[RUSTC-TIMING] gimli test:false 4.110
[RUSTC-TIMING] object test:false 4.503
error[E0609]: no field `d_type` on type `dirent`
     |
1651 |                 match child.entry.d_type {
     |                                   ^^^^^^ unknown field
     |
     |
     = note: available fields are: `d_ino`, `d_off`, `d_reclen`, `d_name`
For more information about this error, try `rustc --explain E0609`.
[RUSTC-TIMING] std test:false 2.188
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:05:39

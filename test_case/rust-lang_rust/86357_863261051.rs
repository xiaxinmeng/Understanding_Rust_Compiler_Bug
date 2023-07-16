plain
   Compiling addr2line v0.14.0
error[E0282]: type annotations needed
   --> library/std/src/os/./unix/net/ancillary.rs:309:33
    |
309 |             let cmsg_len_zero = libc::CMSG_LEN(0) as _;
    |                 -------------   ^^^^^^^^^^^^^^^^^^^^^^ cannot infer type
    |                 |
    |                 consider giving `cmsg_len_zero` a type
    = note: type must be known at this point

error: aborting due to previous error


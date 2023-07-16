plain
   Compiling addr2line v0.16.0
error: unexpected `cfg` condition name
   --> library/std/src/os/unix/net/datagram.rs:870:13
    |
870 |             targeet_os = "netbsd",
    |             |
    |             help: did you mean: `target_os`
    |
    |
    = note: `-D unexpected-cfgs` implied by `-D warnings`
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:03:58

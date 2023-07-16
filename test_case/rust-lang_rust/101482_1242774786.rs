plain
[RUSTC-TIMING] object test:false 5.581
error[E0432]: unresolved import `super::time`
  --> library/std/src/sys/unix/thread_parker/pthread.rs:78:20
   |
78 |         use super::time::Timespec;
   |                    ^^^^ could not find `time` in `super`
For more information about this error, try `rustc --explain E0432`.
[RUSTC-TIMING] std test:false 2.509
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:06:01

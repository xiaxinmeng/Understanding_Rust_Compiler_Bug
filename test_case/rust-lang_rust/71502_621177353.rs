
[root@host]# rustc -o /proc/version non-ice-error-on-worker-io-fail.rs 
warning: unused import: `crate::task::Waker`
  --> non-ice-error-on-worker-io-fail.rs:34:13
   |
34 |         use crate::task::Waker;
   |             ^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

error: failed to write bytecode to /proc/version.non_ice_error_on_worker_io_fail.3a1fbbbh-cgu.0.rcgu.bc.z: No such file or directory (os error 2)

error: aborting due to previous error

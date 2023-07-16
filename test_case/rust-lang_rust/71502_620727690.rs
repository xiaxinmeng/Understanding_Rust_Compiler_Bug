
# rustc -o /dev/null src/test/ui/non-ice-error-on-worker-io-fail.rs
warning: unused import: `crate::task::Waker`
  --> src/test/ui/non-ice-error-on-worker-io-fail.rs:34:13
   |
34 |         use crate::task::Waker;
   |             ^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

# ls -l /dev/null
-rw-r--r--. 1 root root 3562 Apr 28 09:48 /dev/null

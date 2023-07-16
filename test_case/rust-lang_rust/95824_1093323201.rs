plain
   Compiling addr2line v0.16.0
error[E0433]: failed to resolve: use of undeclared type `Ordering`
  --> library/std/src/sys/unix/rand.rs:45:44
   |
45 |         if !GRND_INSECURE_UNAVAILABLE.load(Ordering::Relaxed) {
   |                                            ^^^^^^^^ use of undeclared type `Ordering`
error[E0433]: failed to resolve: use of undeclared type `Ordering`
  --> library/std/src/sys/unix/rand.rs:49:55
   |
   |
49 |                 GRND_INSECURE_UNAVAILABLE.store(true, Ordering::Relaxed);
   |                                                       ^^^^^^^^ use of undeclared type `Ordering`
error[E0412]: cannot find type `AtomicBool` in this scope
  --> library/std/src/sys/unix/rand.rs:44:43
   |
   |
44 |         static GRND_INSECURE_UNAVAILABLE: AtomicBool = AtomicBool::new(false);
   |
help: consider importing one of these items
   |
25 |     use core::sync::atomic::AtomicBool;
25 |     use core::sync::atomic::AtomicBool;
   |
25 |     use crate::sync::atomic::AtomicBool;
   |

error[E0433]: failed to resolve: use of undeclared type `AtomicBool`
  --> library/std/src/sys/unix/rand.rs:44:56
   |
44 |         static GRND_INSECURE_UNAVAILABLE: AtomicBool = AtomicBool::new(false);
   |
help: consider importing one of these items
   |
25 |     use core::sync::atomic::AtomicBool;
25 |     use core::sync::atomic::AtomicBool;
   |
25 |     use crate::sync::atomic::AtomicBool;
   |

error[E0425]: cannot find value `GRND_INSECURE` in crate `libc`
   |
   |
47 |             unsafe { ret = getrandom(buf.as_mut_ptr().cast(), buf.len(), libc::GRND_INSECURE) }

error[E0425]: cannot find function `errno` in this scope
  --> library/std/src/sys/unix/rand.rs:48:29
   |
   |
48 |             if ret == -1 && errno() as libc::c_int == libc::EINVAL {
   |
help: consider importing this function
   |
25 |     use crate::sys::os::errno;

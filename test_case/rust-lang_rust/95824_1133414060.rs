
error[E0425]: cannot find value `GRND_INSECURE` in crate `libc`
  --> library/std/src/sys/unix/rand.rs:51:84
   |
51 |             let ret = unsafe { getrandom(buf.as_mut_ptr().cast(), buf.len(), libc::GRND_INSECURE) };
   |                                                                                    ^^^^^^^^^^^^^ not found in `libc`

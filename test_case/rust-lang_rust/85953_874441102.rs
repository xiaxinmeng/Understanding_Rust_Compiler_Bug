plain
    Checking object v0.22.0
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking miniz_oxide v0.4.0
    Checking addr2line v0.14.0
error: cannot find macro `syscall` in this scope
   --> library/std/src/sys/unix/kernel_copy.rs:519:5
519 |     syscall! {
    |     ^^^^^^^
    |
    = note: consider importing this macro:
    = note: consider importing this macro:
            crate::sys::weak::syscall

error: cannot find macro `syscall` in this scope
   --> library/std/src/sys/unix/kernel_copy.rs:614:5
614 |     syscall! {
    |     ^^^^^^^
    |
    = note: consider importing this macro:
    = note: consider importing this macro:
            crate::sys::weak::syscall

error: cannot find macro `weak` in this scope
   --> library/std/src/sys/unix/os.rs:652:5
652 |     weak! {
    |     ^^^^
    |
    = note: consider importing this macro:
    = note: consider importing this macro:
            crate::sys::weak::weak

error: cannot find macro `syscall` in this scope
   |
33 |         syscall! {
   |         ^^^^^^^
   |
   |
   = note: consider importing this macro:
           crate::sys::weak::syscall

error: cannot find macro `weak` in this scope
    |
    |
547 |     weak!(fn __pthread_get_minstack(*const libc::pthread_attr_t) -> libc::size_t);
    |
    = note: consider importing this macro:
            crate::sys::weak::weak


error[E0425]: cannot find function `copy_file_range` in this scope
   --> library/std/src/sys/unix/kernel_copy.rs:536:21
    |
536 |                 cvt(copy_file_range(INVALID_FD, ptr::null_mut(), INVALID_FD, ptr::null_mut(), 1, 0))
    |
help: consider importing this function
    |
47  | use libc::copy_file_range;
47  | use libc::copy_file_range;
    |

error[E0425]: cannot find function `copy_file_range` in this scope
   --> library/std/src/sys/unix/kernel_copy.rs:560:17
    |
560 |             cvt(copy_file_range(reader, ptr::null_mut(), writer, ptr::null_mut(), bytes_to_copy, 0))
    |
help: consider importing this function
    |
47  | use libc::copy_file_range;
---

error[E0425]: cannot find value `gnu_get_libc_version` in this scope
   --> library/std/src/sys/unix/os.rs:655:22
    |
655 |     if let Some(f) = gnu_get_libc_version.get() {


error[E0425]: cannot find value `__pthread_get_minstack` in this scope
    |
    |
549 |     match __pthread_get_minstack.get() {


error[E0277]: the trait bound `u64: IsMinusOne` is not satisfied
   --> library/std/src/sys/unix/kernel_copy.rs:560:13
    |
560 |             cvt(copy_file_range(reader, ptr::null_mut(), writer, ptr::null_mut(), bytes_to_copy, 0))
    |             ^^^ the trait `IsMinusOne` is not implemented for `u64`
   ::: library/std/src/sys/unix/mod.rs:199:15
    |
    |
199 | pub fn cvt<T: IsMinusOne>(t: T) -> crate::io::Result<T> {
    |               ---------- required by this bound in `sys::unix::cvt`
error[E0061]: this function takes 1 argument but 3 arguments were supplied
  --> library/std/src/sys/unix/rand.rs:41:18
   |
   |
41 |         unsafe { getrandom(buf.as_mut_ptr().cast(), buf.len(), libc::GRND_NONBLOCK) }
   |                  |
   |                  expected 1 argument
   |
note: function defined here
note: function defined here
  --> library/std/src/sys/unix/rand.rs:29:8
   |
29 |     fn getrandom(buf: &mut [u8]) -> libc::ssize_t {

error: aborting due to 12 previous errors

Some errors have detailed explanations: E0061, E0277, E0425.

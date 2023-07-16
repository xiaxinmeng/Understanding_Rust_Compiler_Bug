plain
   Compiling object v0.26.2
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling hashbrown v0.12.0
   Compiling addr2line v0.16.0
error[E0658]: the `linkage` attribute is experimental and not portable across platforms
    |
32  | / pub(crate) macro weak {
33  | |     (fn $name:ident($($t:ty),*) -> $ret:ty) => (
34  | |         let ref $name: ExternWeak<unsafe extern "C" fn($($t),*) -> $ret> = {
---
    | |             ----------------------------------- in this macro invocation (#2)
...   |
190 | |     )
191 | | }
    | |_- in this expansion of `syscall!` (#1)
   ::: library/std/src/sys/unix/fs.rs:134:9
    |
134 | /         syscall! {
135 | |             fn statx(
---
142 | |         }
    | |_________- in this macro invocation (#1)
    |
    = note: see issue #29603 <https://github.com/rust-lang/rust/issues/29603> for more information
    = help: add `#![feature(linkage)]` to the crate attributes to enable

error[E0658]: the `linkage` attribute is experimental and not portable across platforms
    |
32  | / pub(crate) macro weak {
33  | |     (fn $name:ident($($t:ty),*) -> $ret:ty) => (
34  | |         let ref $name: ExternWeak<unsafe extern "C" fn($($t),*) -> $ret> = {
---
    | |             ----------------------------------- in this macro invocation (#2)
...   |
190 | |     )
191 | | }
    | |_- in this expansion of `syscall!` (#1)
   ::: library/std/src/sys/unix/kernel_copy.rs:520:5
    |
520 | /     syscall! {
521 | |         fn copy_file_range(
521 | |         fn copy_file_range(
522 | |             fd_in: libc::c_int,
523 | |             off_in: *mut libc::loff_t,
528 | |         ) -> libc::ssize_t
529 | |     }
    | |_____- in this macro invocation (#1)
    |
    |
    = note: see issue #29603 <https://github.com/rust-lang/rust/issues/29603> for more information
    = help: add `#![feature(linkage)]` to the crate attributes to enable

error[E0658]: the `linkage` attribute is experimental and not portable across platforms
    |
32  | / pub(crate) macro weak {
33  | |     (fn $name:ident($($t:ty),*) -> $ret:ty) => (
34  | |         let ref $name: ExternWeak<unsafe extern "C" fn($($t),*) -> $ret> = {
---
    |
   ::: library/std/src/sys/unix/process/process_unix.rs:435:9
    |
435 | /         weak! {
436 | |             fn posix_spawn_file_actions_addchdir_np(
437 | |                 *mut libc::posix_spawn_file_actions_t,
439 | |             ) -> libc::c_int
440 | |         }
    | |_________- in this macro invocation
    |
    |
    = note: see issue #29603 <https://github.com/rust-lang/rust/issues/29603> for more information
    = help: add `#![feature(linkage)]` to the crate attributes to enable

error[E0658]: the `linkage` attribute is experimental and not portable across platforms
    |
32  |  / pub(crate) macro weak {
33  |  |     (fn $name:ident($($t:ty),*) -> $ret:ty) => (
34  |  |         let ref $name: ExternWeak<unsafe extern "C" fn($($t),*) -> $ret> = {
---
173 | |              weak! { fn $name($($t),*) -> $ret }
    | |              ----------------------------------- in this macro invocation (#2)
...   |
190 | |      )
191 | |  }
    | |__- in this expansion of `syscall!` (#1)
   ::: library/std/src/sys/unix/rand.rs:36:9
    |
36  | /          syscall! {
37  | |              fn getrandom(
---
42  | |          }
    | |__________- in this macro invocation (#1)
    |
    = note: see issue #29603 <https://github.com/rust-lang/rust/issues/29603> for more information
    = help: add `#![feature(linkage)]` to the crate attributes to enable

error[E0658]: the `linkage` attribute is experimental and not portable across platforms
   |
25 |         #[linkage = "extern_weak"]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: see issue #29603 <https://github.com/rust-lang/rust/issues/29603> for more information
   = help: add `#![feature(linkage)]` to the crate attributes to enable

error[E0658]: the `linkage` attribute is experimental and not portable across platforms
   |
27 |         #[linkage = "extern_weak"]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: see issue #29603 <https://github.com/rust-lang/rust/issues/29603> for more information
   = help: add `#![feature(linkage)]` to the crate attributes to enable
For more information about this error, try `rustc --explain E0658`.
error: could not compile `std` due to 6 previous errors
Build completed unsuccessfully in 0:00:17

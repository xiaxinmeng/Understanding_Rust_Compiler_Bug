plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between b15ca6635f752fefebfd101aa944c6167128183c and dbc5bc70cda5e86692608ac3cc01ee2183603908
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
    |
280 | pub struct DirEntry {
    | ------------------- previous definition of the type `DirEntry` here
...
315 | pub struct DirEntry {
    | ^^^^^^^^^^^^^^^^^^^ `DirEntry` redefined here
    |
    = note: `DirEntry` must be defined only once in the type namespace of this module
error[E0432]: unresolved import `libc::fstatat64`
  --> /checkout/library/std/src/sys/unix/fs.rs:55:5
   |
55 | use libc::fstatat64;
55 | use libc::fstatat64;
   |     ^^^^^^---------
   |     |     |
   |     |     help: a similar name exists in the module: `fstatat`
   |     no `fstatat64` in the root

error[E0432]: unresolved imports `libc::dirent64`, `libc::fstat64`, `libc::ftruncate64`, `libc::lseek64`, `libc::lstat64`, `libc::off64_t`, `libc::open64`, `libc::stat64`
    |
    |
102 | use libc::{dirent64, fstat64, ftruncate64, lseek64, lstat64, off64_t, open64, stat64};
    |            |         |        |            |        |        |        |
    |            |         |        |            |        |        |        no `open64` in the root
    |            |         |        |            |        |        no `off64_t` in the root
    |            |         |        |            |        no `lstat64` in the root
    |            |         |        |            |        no `lstat64` in the root
    |            |         |        |            no `lseek64` in the root
    |            |         |        no `ftruncate64` in the root
    |            |         no `fstat64` in the root
    |            no `dirent64` in the root
help: a similar name exists in the module
    |
    |
102 | use libc::{dirent, fstat64, ftruncate64, lseek64, lstat64, off64_t, open64, stat64};
help: a similar name exists in the module
    |
    |
102 | use libc::{dirent64, fstat, ftruncate64, lseek64, lstat64, off64_t, open64, stat64};
help: a similar name exists in the module
    |
    |
102 | use libc::{dirent64, fstat64, ftruncate, lseek64, lstat64, off64_t, open64, stat64};
help: a similar name exists in the module
    |
    |
102 | use libc::{dirent64, fstat64, ftruncate64, lseek, lstat64, off64_t, open64, stat64};
help: a similar name exists in the module
    |
    |
102 | use libc::{dirent64, fstat64, ftruncate64, lseek64, lstat, off64_t, open64, stat64};
help: a similar name exists in the module
    |
    |
102 | use libc::{dirent64, fstat64, ftruncate64, lseek64, lstat64, off_t, open64, stat64};
help: a similar name exists in the module
    |
    |
102 | use libc::{dirent64, fstat64, ftruncate64, lseek64, lstat64, off64_t, open, stat64};
help: a similar name exists in the module
    |
    |
102 | use libc::{dirent64, fstat64, ftruncate64, lseek64, lstat64, off64_t, open64, stat};

error[E0412]: cannot find type `dirent64_min` in this scope
    --> /checkout/library/std/src/sys/unix/fs.rs:282:12
     |
     |
282  |       entry: dirent64_min,
     |              ^^^^^^^^^^^^ help: a struct with a similar name exists: `dirent64`
     |
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.138/src/unix/bsd/apple/mod.rs:1031:1
     |
1031 | / s_no_extra_traits! {
1032 | |     #[cfg_attr(libc_packedN, repr(packed(4)))]
1033 | |     pub struct kevent {
1034 | |         pub ident: ::uintptr_t,
1309 | |     }
1310 | | }
     | |_- similarly named struct `dirent64` defined here


error[E0063]: missing field `name` in initializer of `sys::unix::fs::DirEntry`
   --> /checkout/library/std/src/sys/unix/fs.rs:715:27
    |
715 |             let mut ret = DirEntry { entry: mem::zeroed(), dir: Arc::clone(&self.inner) };
    |                           ^^^^^^^^ missing `name`
Some errors have detailed explanations: E0063, E0412, E0428, E0432.
For more information about an error, try `rustc --explain E0063`.
error: could not compile `std` due to 5 previous errors
fatal error: failed to build sysroot, see error details above

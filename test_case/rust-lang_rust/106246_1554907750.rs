plain
    |
299 | pub struct DirEntry {
    | ------------------- previous definition of the type `DirEntry` here
...
336 | pub struct DirEntry {
    | ^^^^^^^^^^^^^^^^^^^ `DirEntry` redefined here
    |
    = note: `DirEntry` must be defined only once in the type namespace of this module
error[E0432]: unresolved import `libc::fstatat64`
  --> /checkout/library/std/src/sys/unix/fs.rs:57:5
   |
57 | use libc::fstatat64;
57 | use libc::fstatat64;
   |     ^^^^^^---------
   |     |     |
   |     |     help: a similar name exists in the module: `fstatat`
   |     no `fstatat64` in the root

error[E0432]: unresolved imports `libc::dirent64`, `libc::fstat64`, `libc::ftruncate64`, `libc::lseek64`, `libc::lstat64`, `libc::off64_t`, `libc::open64`, `libc::stat64`
    |
    |
106 | use libc::{dirent64, fstat64, ftruncate64, lseek64, lstat64, off64_t, open64, stat64};
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
106 | use libc::{dirent, fstat64, ftruncate64, lseek64, lstat64, off64_t, open64, stat64};
help: a similar name exists in the module
    |
    |
106 | use libc::{dirent64, fstat, ftruncate64, lseek64, lstat64, off64_t, open64, stat64};
help: a similar name exists in the module
    |
    |
106 | use libc::{dirent64, fstat64, ftruncate, lseek64, lstat64, off64_t, open64, stat64};
help: a similar name exists in the module
    |
    |
106 | use libc::{dirent64, fstat64, ftruncate64, lseek, lstat64, off64_t, open64, stat64};
help: a similar name exists in the module
    |
    |
106 | use libc::{dirent64, fstat64, ftruncate64, lseek64, lstat, off64_t, open64, stat64};
help: a similar name exists in the module
    |
    |
106 | use libc::{dirent64, fstat64, ftruncate64, lseek64, lstat64, off_t, open64, stat64};
help: a similar name exists in the module
    |
    |
106 | use libc::{dirent64, fstat64, ftruncate64, lseek64, lstat64, off64_t, open, stat64};
help: a similar name exists in the module
    |
    |
106 | use libc::{dirent64, fstat64, ftruncate64, lseek64, lstat64, off64_t, open64, stat};

error[E0412]: cannot find type `dirent64_min` in this scope
    --> /checkout/library/std/src/sys/unix/fs.rs:301:12
     |
     |
301  |       entry: dirent64_min,
     |              ^^^^^^^^^^^^ help: a struct with a similar name exists: `dirent64`
     |
    ::: /cargo/registry/src/index.crates.io-6f17d22bba15001f/libc-0.2.143/src/unix/bsd/apple/mod.rs:1105:1
     |
1105 | / s_no_extra_traits! {
1106 | |     #[cfg_attr(libc_packedN, repr(packed(4)))]
1107 | |     pub struct kevent {
1108 | |         pub ident: ::uintptr_t,
1383 | |     }
1384 | | }
     | |_- similarly named struct `dirent64` defined here


error[E0063]: missing field `name` in initializer of `sys::unix::fs::DirEntry`
   --> /checkout/library/std/src/sys/unix/fs.rs:768:27
    |
768 |             let mut ret = DirEntry { entry: mem::zeroed(), dir: Arc::clone(&self.inner) };
    |                           ^^^^^^^^ missing `name`
Some errors have detailed explanations: E0063, E0412, E0428, E0432.
For more information about an error, try `rustc --explain E0063`.
error: could not compile `std` (lib) due to 5 previous errors
fatal error: failed to build sysroot: sysroot build failed

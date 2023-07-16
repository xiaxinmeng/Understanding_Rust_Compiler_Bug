plain
      Memory: 14 GB
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VM/nlACvQ1R9
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 3
hw.byteorder: 1234
---
   Compiling addr2line v0.16.0
[RUSTC-TIMING] addr2line test:false 1.015
[RUSTC-TIMING] gimli test:false 12.157
[RUSTC-TIMING] object test:false 10.355
error: expected one of `async`, `move`, `|`, or `||`, found `DLSYM`
     |
75   | / pub(crate) macro dlsym {
76   | |     (fn $name:ident($($t:ty),*) -> $ret:ty) => (
76   | |     (fn $name:ident($($t:ty),*) -> $ret:ty) => (
77   | |          dlsym!(fn $name($($t),*) -> $ret, stringify!($name));
78   | |     ),
78   | |     ),
79   | |     (fn $name:ident($($t:ty),*) -> $ret:ty, $sym:expr) => (
80   | |         static DLSYM: DlsymWeak<unsafe extern "C" fn($($t),*) -> $ret> =
     | |                ^^^^^ expected one of `async`, `move`, `|`, or `||`
83   | |     )
84   | | }
     | | -
     | | |
     | | |
     | |_in this expansion of `weak!` (#1)
     |   in this expansion of `dlsym!` (#2)
    ::: library/std/src/sys/unix/fs.rs:1073:36
     |
     |
1073 |                       let futimens = weak!(fn futimens(c_int, *const libc::timespec) -> c_int);
     |
     = note: the macro call doesn't expand to an expression, but it can expand to a statement
     = note: the macro call doesn't expand to an expression, but it can expand to a statement
help: add `;` to interpret the expansion as a statement
     |
77   |          dlsym!(fn $name($($t),*) -> $ret, stringify!($name));;

error: trailing semicolon in macro used in expression position
    --> library/std/src/sys/unix/weak.rs:77:62
     |
     |
75   | / pub(crate) macro dlsym {
76   | |     (fn $name:ident($($t:ty),*) -> $ret:ty) => (
77   | |          dlsym!(fn $name($($t),*) -> $ret, stringify!($name));
78   | |     ),
...    |
83   | |     )
84   | | }
84   | | }
     | |_- in this expansion of `weak!`
     |
    ::: library/std/src/sys/unix/fs.rs:1073:36
     |
1073 |                       let futimens = weak!(fn futimens(c_int, *const libc::timespec) -> c_int);
     |
     |
     = note: `-D semicolon-in-expressions-from-macros` implied by `-D warnings`
     = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>

error[E0308]: mismatched types
    --> library/std/src/sys/unix/fs.rs:1077:54
    --> library/std/src/sys/unix/fs.rs:1077:54
     |
1077 | ...                   let timevals = [ts_to_tv(times.0[0]), ts_to_tv(times.0[1])];
     |                                       |        |
     |                                       |        expected `&timespec`, found struct `timespec`
     |                                       |        help: consider borrowing here: `&times.0[0]`
     |                                       arguments to this function are incorrect
     |                                       arguments to this function are incorrect
     |
note: function defined here
    --> library/std/src/sys/unix/fs.rs:1069:20
     |
1069 |                 fn ts_to_tv(ts: &libc::timespec) -> libc::timeval {

error[E0308]: mismatched types
    --> library/std/src/sys/unix/fs.rs:1077:76
     |
     |
1077 | ...                   let timevals = [ts_to_tv(times.0[0]), ts_to_tv(times.0[1])];
     |                                                             |        |
     |                                                             |        expected `&timespec`, found struct `timespec`
     |                                                             |        help: consider borrowing here: `&times.0[1]`
     |                                                             arguments to this function are incorrect
     |                                                             arguments to this function are incorrect
     |
note: function defined here
    --> library/std/src/sys/unix/fs.rs:1069:20
     |
1069 |                 fn ts_to_tv(ts: &libc::timespec) -> libc::timeval {

error[E0308]: mismatched types
    --> library/std/src/sys/unix/fs.rs:1070:65
     |
     |
1070 |                     libc::timeval { tv_sec: ts.tv_sec, tv_usec: ts.tv_nsec / 1000 }
     |                                                                 ^^^^^^^^^^^^^^^^^ expected `i32`, found `i64`
For more information about this error, try `rustc --explain E0308`.
[RUSTC-TIMING] std test:false 3.894
error: could not compile `std` due to 5 previous errors
Build completed unsuccessfully in 0:01:05

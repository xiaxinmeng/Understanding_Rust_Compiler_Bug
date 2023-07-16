
$ cargo +nightly build
   Compiling libc v0.2.41 (file:///home/tobias/dev/libc)
error[E0463]: can't find crate for `dox`
   --> src/macros.rs:44:16
    |
44  |           impl ::dox::Copy for $i {}
    |                  ^^^ can't find crate
    | 
   ::: src/unix/mod.rs:19:1
    |
19  | / s! {
20  | |     pub struct group {
21  | |         pub gr_name: *mut crate::c_char,
22  | |         pub gr_passwd: *mut crate::c_char,
...   |
186 | |     }
187 | | }
    | |_- in this macro invocation

error: aborting due to previous error

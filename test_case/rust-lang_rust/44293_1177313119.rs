
...
Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
...
error[E0463]: can't find crate for `proc_macro`
   --> /root/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro-hack-0.5.19/src/lib.rs:150:1
    |
150 | extern crate proc_macro;
    | ^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate

error[E0463]: can't find crate for `proc_macro`
   --> /root/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.37/src/lib.rs:119:1
    |
119 | extern crate proc_macro;
    | ^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate

   Compiling unic-emoji-char v0.9.0
error[E0463]: can't find crate for `proc_macro`

error[E0635]: unknown feature `proc_macro_span`
  --> /root/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.37/src/lib.rs:90:59
   |
90 | #![cfg_attr(any(proc_macro_span, super_unstable), feature(proc_macro_span))]
   |                                                           ^^^^^^^^^^^^^^^


error[E0514]: found crate `cc` compiled by an incompatible version of rustc
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.4/build.rs:66:5
   |
66 |     extern crate cc;
   |     ^^^^^^^^^^^^^^^^
   |
   = help: please recompile that crate using this compiler (rustc 1.32.0-beta.11 (e64fee6a3 2019-01-04))
   = note: the following crate versions were found:
           crate `cc` compiled by rustc 1.32.0-beta.2 (a01e4761a 2018-12-08): /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps/libcc-34c709b76b7d1d6d.rlib                                                                               

error: aborting due to previous error

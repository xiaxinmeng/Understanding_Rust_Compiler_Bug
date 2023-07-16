
foo.rs:16:5: 16:23 error: default trait implementations are experimental and possibly buggy
foo.rs:16     impl Bar for .. {}
              ^~~~~~~~~~~~~~~~~~
foo.rs:16:23: 16:23 help: add #![feature(optin_builtin_traits)] to the crate attributes to enable
error: aborting due to previous error
make[1]: *** [all] Error 101


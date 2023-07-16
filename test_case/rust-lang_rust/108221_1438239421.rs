
$ echo '#![cfg_attr(unix, crate_name = "foo")] #![crate_type = "lib"]' | rustc +nightly - -Adeprecated_cfg_attr_crate_type_name
$ ls
librust_out.rlib
$ rustc +nightly -Zls librust_out.rlib
Crate info:
name rust_out
[...]

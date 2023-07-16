output
$ cat test.rs
extern crate proc_macro;
$ rustc test.rs --crate-type proc-macro
warning: dropping unsupported crate type `proc-macro` for target `x86_64-unknown-linux-musl`
$ rustc test.rs --crate-type proc-macro -C target-feature=-crt-static
$ rustc test.rs --crate-type proc-macro -C target-feature=+crt-static
warning: dropping unsupported crate type `proc-macro` for target `x86_64-unknown-linux-musl`
$ rustc +stage1 test.rs --crate-type proc-macro
$ rustc +stage1 test.rs --crate-type proc-macro -C target-feature=-crt-static
$ rustc +stage1 test.rs --crate-type proc-macro -C target-feature=+crt-static
warning: dropping unsupported crate type `proc-macro` for target `x86_64-unknown-linux-musl`

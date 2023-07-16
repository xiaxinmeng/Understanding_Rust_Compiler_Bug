
8537.rs:1:13: 1:14 error: illegal ABI: expected one of [cdecl, stdcall, fastcall, aapcs, Rust, C, rust-intrinsic], found `foo`
8537.rs:1 extern "foo" {}
                       ^
8537.rs:2:24: 2:26 error: illegal ABI: expected one of [cdecl, stdcall, fastcall, aapcs, Rust, C, rust-intrinsic], found `foo`
8537.rs:2 type Foo = extern "foo" fn();
                                  ^~
8537.rs:3:13: 3:15 error: illegal ABI: expected one of [cdecl, stdcall, fastcall, aapcs, Rust, C, rust-intrinsic], found `foo`
8537.rs:3 extern "foo" fn foo() {}
                       ^~
error: aborting due to 3 previous errors
task '<unnamed>' failed at 'explicit failure', /home/huon/rust/src/libsyntax/diagnostic.rs:101
task '<unnamed>' failed at 'explicit failure', /home/huon/rust/src/librustc/rustc.rs:396


[00:07:42]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:42] error[E0523]: found two different crates with name `getopts` that are not distinguished by differing `-C metadata`. This will result in symbol conflicts between the two.
[00:07:42]     |
[00:07:42] 119 | extern crate test;
[00:07:42]     | ^^^^^^^^^^^^^^^^^^
[00:07:42] 
...
[00:07:58] error: build failed
[00:07:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:07:58] expected success, got: exit code: 101
[00:07:58] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:07:58] travis_fold:end:stage0-rustc

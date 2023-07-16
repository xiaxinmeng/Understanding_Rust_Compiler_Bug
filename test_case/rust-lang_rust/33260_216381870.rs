

failures:

---- Rust_Compiler_Error_Index_13 stdout ----
    <anon>:13:14: 13:15 error: cannot bind by-move into a pattern guard [E0008]
<anon>:13         Some(y) if y.clone().consume() > 0 => {}
                       ^
error: aborting due to previous error
thread 'Rust_Compiler_Error_Index_13' panicked at 'Box<Any>', /buildslave/rust-buildbot/slave/auto-linux-64-opt-rustbuild/build/src/libsyntax/errors/mod.rs:614
thread 'Rust_Compiler_Error_Index_13' panicked at 'couldn't compile the test', /buildslave/rust-buildbot/slave/auto-linux-64-opt-rustbuild/build/src/librustdoc/test.rs:276


failures:
    Rust_Compiler_Error_Index_13

test result: FAILED. 427 passed; 1 failed; 45 ignored; 0 measured

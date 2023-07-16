plain
ruby 2.4.1p111 (2017-03-22 revision 58053) [x86_64-linux]
travis_fold:end:system_info

Network availability confirmed.
Running apt-get update by default has been disabled.
You can opt into running apt-get update by setting this in your .travis.yml file:
  apt:
travis_fold:start:git.checkout
travis_time:start:1e53ea2d
$ git clone --depth=2 https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:05:47]    Compiling backtrace v0.3.6
[00:05:56]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:07:31]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:07:48]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:07:49] error: unused variable: `attrs`
[00:07:49]    --> libsyntax_ext/deriving/encodable.rs:193:57
[00:07:49]     |
[00:07:49] 193 |             for (i, &FieldInfo { name, ref self_, span, attrs, .. }) in fields.iter().enumerate() {
[00:07:49]     |                                                         ^^^^^ help: consider using `_attrs` instead
[00:07:49]     = note: `-D unused-variables` implied by `-D warnings`
[00:07:49] 
[00:07:49] error: aborting due to previous error
[00:07:49] 
[00:07:49] 
[00:07:49] error: Could not compile `syntax_ext`.
[00:07:49] 
[00:07:49] Caused by:
[00:07:49]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name syntax_ext libsyntax_ext/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C metadata=4a747fc4f7c945c8 -C extra-filename=-4a747fc4f7c945c8 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-33787dcdac3a4dd2.so --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-8d928be2ff984c7f.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-566a8d95e6a18781.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-f456f53371aa074c.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-b9901acb1e9e6766.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-b789a86e1ab64d11.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-aed9d8ab86b35123.so` (exit code: 101)
[00:13:48] error: build failed
[00:13:48] error: build failed
[00:13:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:13:48] expected success, got: exit code: 101
[00:13:48] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:13:48] travis_fold:end:stage0-rustc

[00:13:48] travis_time:end:stage0-rustc:start=1525883686170843065,finish=1525884209892973948,duration=523722130883


[00:13:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:13:48] Build completed unsuccessfully in 0:08:58
[00:13:48] make: *** [all] Error 1
[00:13:48] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:27f0d6f4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)

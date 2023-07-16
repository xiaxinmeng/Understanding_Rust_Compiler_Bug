plain
travis_fold:end:system_info

Network availability confirmed.
127.0.0.1 localhost nettuno travis vagrant
127.0.1.1 travis-job-22fb73ff-8bf8-4a2c-8114-882905219099 travis-job-22fb73ff-8bf8-4a2c-8114-882905219099 ip4-loopback trusty64
travis_time:start:1bc1b5d7
$ git clone --depth=2 https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:06:49]    Compiling rustc_const_math v0.0.0 (file:///checkout/src/librustc_const_math)
[00:07:12]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:13:34]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:13:34]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:13:43] error[E0616]: field `features` of struct `rustc::session::Session` is private
[00:13:43]    --> librustc_mir/hair/pattern/check_match.rs:370:32
[00:13:43]     |
[00:13:43] 370 |                             if cx.tcx.sess.features.borrow().irrefutable_let_pattern {
[00:13:43] 
[00:13:43] 
[00:13:43] error[E0616]: field `features` of struct `rustc::session::Session` is private
[00:13:43]    --> librustc_mir/hair/pattern/check_match.rs:410:40
[00:13:43]     |
[00:13:43] 410 |                                     if cx.tcx.sess.features.borrow().irrefutable_let_pattern {
[00:13:43] 
[00:13:48] error: aborting due to 2 previous errors
[00:13:48] 
[00:13:48] For more information about this error, try `rustc --explain E0616`.
[00:13:48] For more information about this error, try `rustc --explain E0616`.
[00:13:48] error: Could not compile `rustc_mir`.
[00:13:48] 
[00:13:48] Caused by:
[00:13:48]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=809b6518a044b653 -C extra-filename=-809b6518a044b653 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-d3b6fcf798f7d22a.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c04ded78717d5d67.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c04ded78717d5d67.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-609e2421d03f9c9a.so --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-ca61dfec7c40c4d4.rlib --extern rustc_back=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_back-861dba9fe03aa669.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-93cb1ddd29ab61a4.so --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-0bde40de32995f14.so --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-dd80e21b25e89693.so --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-70f7ed359bd54256.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-3e35efb9e5bc7a9a.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-3e9ac89279d9d9fd.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-1299638d641ea770.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-9866e194db82a141.rlib --extern rustc_const_math=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_math-895f7ddc4467bb8d.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-9f3518d56a01456f.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-22398a187b4139a2/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-4a7fffed6b170d5b/out` (exit code: 101)
[00:15:29] error: build failed
[00:15:29] error: build failed
[00:15:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:15:29] expected success, got: exit code: 101
[00:15:29] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:15:29] travis_fold:end:stage0-rustc

[00:15:29] travis_time:end:stage0-rustc:start=1524328030770360204,finish=1524328675207797268,duration=644437437064


[00:15:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:15:29] Build completed unsuccessfully in 0:10:57
[00:15:29] Makefile:28: recipe for target 'all' failed
[00:15:29] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06b20ae8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)

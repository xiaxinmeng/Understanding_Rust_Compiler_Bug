plain
[00:13:42]    Compiling rustc_metadata_utils v0.0.0 (file:///checkout/src/librustc_metadata_utils)
[00:13:42]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:13:42]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:13:43]    Compiling rustc_allocator v0.0.0 (file:///checkout/src/librustc_allocator)
[00:13:44] error[E0531]: cannot find tuple struct/variant `Anon` in module `ty`
[00:13:44]    --> librustc_mir/transform/qualify_min_const_fn.rs:103:17
[00:13:44]     |
[00:13:44] 103 |             ty::Anon(..) => return Err((span, "`impl Trait` in const fn is unstable".into())),
[00:13:44] help: possible candidate is found in another module, you can import it into scope
[00:13:44]     |
[00:13:44]     |
[00:13:44] 1   | use rustc::dep_graph::OpenTask::Anon;
[00:13:44] 
[00:13:55] error: aborting due to previous error
[00:13:55] 
[00:13:55] For more information about this error, try `rustc --explain E0531`.
[00:13:55] For more information about this error, try `rustc --explain E0531`.
[00:13:55] error: Could not compile `rustc_mir`.
[00:13:55] 
[00:13:55] Caused by:
[00:13:55]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=9389ef21a7981207 -C extra-filename=-9389ef21a7981207 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-93ff3b002b18b7ed.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-5a39798fe03e47f4.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-8246be02936c9b1b.rlib --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-0a515e87c8afea9e.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-45ae4394366d07fd.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-c55d6c95192e4906.rlib --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-87ec950697a15ed0.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-218f3033f29f5493.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-09c0f3a89ad0d6b5.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-96aac12abd62414d.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-05301c67193a930e.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-f4ac364f854372fe.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-4a08b81d2b6640c1.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8d84add221c0f710.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8d84add221c0f710.rlib --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-cb741677cd0e0351.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-0651ffc5a9129db1.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-47b99ffec2efbd05.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-ee16f6821aef40e9/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-52504d5ed57fefc2/out` (exit code: 1)
[00:15:05] error: build failed
[00:15:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:15:05] expected success, got: exit code: 101
[00:15:05] expected success, got: exit code: 101
[00:15:05] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1155:9
[00:15:05] travis_fold:end:stage0-rustc

[00:15:05] travis_time:end:stage0-rustc:start=1536174534184896379,finish=1536175100528248583,duration=566343352204


[00:15:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:15:05] Build completed unsuccessfully in 0:10:25
[00:15:05] Makefile:28: recipe for target 'all' failed
[00:15:05] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04adc0a7
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
20384 ./.git/modules/src/liblibc/objects
20376 ./.git/modules/src/liblibc/objects/pack
20032 ./src/tools/lldb/source
travis_time:end:0b19f61c:start=1536175100968716617,finish=1536175101365119134,duration=396402517
travis_fold:end:after_faile/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:039b4a68
$ dmesg | grep -i kill

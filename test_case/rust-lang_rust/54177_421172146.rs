plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/e3/12/92bd3655f436aa009688e7ccb8b7581554fb64278d111f5845e79da8e618/awscli-1.16.14-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 11.8MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▊                               | 30kB 2.2MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[00:13:17]    Compiling rustc_traits v0.0.0 (file:///checkout/src/librustc_traits)
[00:13:18] error[E0432]: unresolved import `rustc_data_structures::indexed_set::Iter`
[00:13:18]   --> librustc_mir/borrow_check/flows.rs:18:5
[00:13:18]    |
[00:13:18] 18 | use rustc_data_structures::indexed_set::Iter;
[00:13:18]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `Iter` in `indexed_set`
[00:13:18] error[E0432]: unresolved import `rustc_data_structures::indexed_set::Iter`
[00:13:18] error[E0432]: unresolved import `rustc_data_structures::indexed_set::Iter`
[00:13:18]   --> librustc_mir/dataflow/at_location.rs:15:64
[00:13:18]    |
[00:13:18] 15 | use rustc_data_structures::indexed_set::{HybridIdxSet, IdxSet, Iter};
[00:13:18]    |                                                                ^^^^ no `Iter` in `indexed_set`
[00:13:28] error: aborting due to 2 previous errors
[00:13:28] 
[00:13:28] For more information about this error, try `rustc --explain E0432`.
[00:13:28] error: Could not compile `rustc_mir`.
[00:13:28] error: Could not compile `rustc_mir`.
[00:13:28] 
[00:13:28] Caused by:
[00:13:28]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=9389ef21a7981207 -C extra-filename=-9389ef21a7981207 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-93ff3b002b18b7ed.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-5a39798fe03e47f4.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-8246be02936c9b1b.rlib --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-0a515e87c8afea9e.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-45ae4394366d07fd.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-c55d6c95192e4906.rlib --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-87ec950697a15ed0.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-218f3033f29f5493.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-09c0f3a89ad0d6b5.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-96aac12abd62414d.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-05301c67193a930e.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-f4ac364f854372fe.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-4a08b81d2b6640c1.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8d84add221c0f710.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8d84add221c0f710.rlib --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-cb741677cd0e0351.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-0651ffc5a9129db1.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-47b99ffec2efbd05.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-ee16f6821aef40e9/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-52504d5ed57fefc2/out` (exit code: 1)
[00:14:37] error: build failed
[00:14:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:14:37] expected success, got: exit code: 101
[00:14:37] expected success, got: exit code: 101
[00:14:37] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1155:9
[00:14:37] travis_fold:end:stage0-rustc

[00:14:37] travis_time:end:stage0-rustc:start=1536876960878904047,finish=1536877496822500782,duration=535943596735


[00:14:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:14:37] Build completed unsuccessfully in 0:09:49
[00:14:37] make: *** [all] Error 1
[00:14:37] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2177207e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)

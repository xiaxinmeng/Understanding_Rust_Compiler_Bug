plain
    100% |████████████████████████████████| 61kB 5.5MB/s 
Collecting botocore==1.10.19 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/34/05/1ffe77f2b8fb03a9223f3d0743cd38cded9491b074c65f3bb664ed4b7ac6/botocore-1.10.19-py2.py3-none-any.whl (4.2MB)
    0% |                                | 10kB 27.2MB/s eta 0:00:01
    0% |▏                               | 20kB 8.7MB/s eta 0:00:01
    0% |▎                               | 30kB 12.1MB/s eta 0:00:01
    0% |▎                               | 40kB 10.2MB/s eta 0:00:01
---
[00:06:27]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:06:27]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:06:29]    Compiling rustc_allocator v0.0.0 (file:///checkout/src/librustc_allocator)
[00:06:30]    Compiling rustc_resolve v0.0.0 (file:///checkout/src/librustc_resolve)
[00:06:30] error[E0425]: cannot find value `path_segments` in this scope
[00:06:30]    --> librustc_resolve/macros.rs:469:24
[00:06:30]     |
[00:06:30] 469 |                 .push((path_segments.into_boxed_slice(), span));
[00:06:30] 
[00:06:33] error: aborting due to previous error
[00:06:33] 
[00:06:33] For more information about this error, try `rustc --explain E0425`.
[00:06:33] For more information about this error, try `rustc --explain E0425`.
[00:06:33] error: Could not compile `rustc_resolve`.
[00:06:33] 
[00:06:33] Caused by:
[00:06:33]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_resolve librustc_resolve/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,metadata -C prefer-dynamic -C debug-assertions=off -C overflow-checks=on -C metadata=a0f48aa432a99101 -C extra-filename=-a0f48aa432a99101 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-859e7ef7a6cf25eb.rmeta --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-c47a7799a87f795e.rmeta --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-2560933221fde227.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-b70225e395c9707a.rmeta --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-f3f5c7806b6ab2a3.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-f94e6b9bfd8025fa.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-9fd067218c2c9169.rmeta -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-ffd9dcc5ce13143f/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-41d448831c9d08f1/out` (exit code: 101)
[00:07:01] error: build failed
[00:07:01] error: build failed
[00:07:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:07:01] travis_fold:end:stage0-rustc


[00:07:01] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:07:01] travis_time:end:stage0-rustc:start=1526004596719599539,finish=1526004762565331279,duration=165845731740

[00:07:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
[00:07:01] Build completed unsuccessfully in 0:06:13

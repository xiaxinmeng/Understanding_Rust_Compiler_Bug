plain
  Downloading https://files.pythonhosted.org/packages/b7/31/05c8d001f7f87f0f07289a5fc0fc3832e9a57f2dbd4d3b0fee70e0d51365/jmespath-0.9.3-py2.py3-none-any.whl
Collecting python-dateutil<3.0.0,>=2.1; python_version >= "2.7" (from botocore==1.10.17->awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/cf/f5/af2b09c957ace60dcfac112b669c45c8c97e32f94aa8b56da4c6d1682825/python_dateutil-2.7.3-py2.py3-none-any.whl (211kB)
    4% |█▌                              | 10kB 45.3MB/s eta 0:00:01
    9% |███                             | 20kB 40.3MB/s eta 0:00:01
    14% |████▋                           | 30kB 45.8MB/s eta 0:00:01
    19% |██████▏                         | 40kB 48.4MB/s eta 0:00:01
---
[00:17:55]    Compiling rustc_traits v0.0.0 (file:///checkout/src/librustc_traits)
[00:17:56] error[E0282]: type annotations needed
[00:17:56]    --> librustc_traits/lowering.rs:258:9
[00:17:56]     |
[00:17:56] 258 |     let wellformed_clauses = where_clauses[1..]
[00:17:56]     |         |
[00:17:56]     |         cannot infer type for `_`
[00:17:56]     |         cannot infer type for `_`
[00:17:56]     |         consider giving `wellformed_clauses` a type
[00:17:56] 
[00:17:56] error[E0599]: no method named `into_wellformed_goal` found for type `rustc::traits::DomainGoal<'_>` in the current scope
[00:17:56]    --> librustc_traits/lowering.rs:301:64
[00:17:56]     |
[00:17:56] 301 |     let wellformed_wc = where_clause.lower().map_bound(|wc| wc.into_wellformed_goal());
[00:17:56] 
[00:17:56] error: aborting due to 2 previous errors
[00:17:56] 
[00:17:56] Some errors occurred: E0282, E0599.
[00:17:56] Some errors occurred: E0282, E0599.
[00:17:56] For more information about an error, try `rustc --explain E0282`.
[00:17:56] error: Could not compile `rustc_traits`.
[00:17:56] 
[00:17:56] Caused by:
[00:17:56]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_traits librustc_traits/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C metadata=21a43c459e5c4f83 -C extra-filename=-21a43c459e5c4f83 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-21ce4bd19908f0cc.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-f456f53371aa074c.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-b9901acb1e9e6766.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-dec7324c263ceb0f.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-401bddd0d1809e53.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-4f0866e958f59455.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-566a8d95e6a18781.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-ffd9dcc5ce13143f/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-41d448831c9d08f1/out` (exit code: 101)
[00:19:21] error: build failed
[00:19:21] error: build failed
[00:19:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:19:21] expected success, got: exit code: 101
[00:19:21] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:19:21] travis_fold:end:stage0-rustc

[00:19:21] travis_time:end:stage0-rustc:start=1525961030619140234,finish=1525961896673188150,duration=866054047916


[00:19:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:19:21] Build completed unsuccessfully in 0:14:41
[00:19:21] make: *** [all] Error 1
[00:19:21] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:21309462
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:2332a1a2
$ echo "#### Build failed; Disk usagry/Logs/DiagnosticReports/: No such file or directory
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:17f81f6a
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true

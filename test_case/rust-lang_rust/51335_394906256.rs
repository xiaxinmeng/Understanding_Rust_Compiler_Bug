plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/5f/86/363f1249d0b62d7cce3cb8973fb6715b57ca75f8425d6c45fe5e129531a9/awscli-1.15.33-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 3.7MB/s eta 0:00:01
    1% |▌                               | 20kB 1.6MB/s eta 0:00:01
    2% |▉                               | 30kB 1.8MB/s eta 0:00:01
    3% |█                               | 40kB 1.8MB/s eta 0:00:01
---
[00:13:17]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:13:17]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:16:10]    Compiling rustc_resolve v0.0.0 (file:///checkout/src/librustc_resolve)
[00:16:11]    Compiling rustc_allocator v0.0.0 (file:///checkout/src/librustc_allocator)
[00:16:12] error[E0412]: cannot find type `Mac` in this scope
[00:16:12]    --> librustc_allocator/expand.rs:171:33
[00:16:12]     |
[00:16:12] 171 |     fn fold_mac(&mut self, mac: Mac) -> Mac {
[00:16:12]     |                                 ^^^ not found in this scope
[00:16:12] help: possible candidate is found in another module, you can import it into scope
[00:16:12] 11  | use syntax::ast::Mac;
[00:16:12]     |
[00:16:12] 
[00:16:12] 
[00:16:12] error[E0412]: cannot find type `Mac` in this scope
[00:16:12]    --> librustc_allocator/expand.rs:171:41
[00:16:12]     |
[00:16:12] 171 |     fn fold_mac(&mut self, mac: Mac) -> Mac {
[00:16:12]     |                                         ^^^ not found in this scope
[00:16:12] help: possible candidate is found in another module, you can import it into scope
[00:16:12] 11  | use syntax::ast::Mac;
[00:16:12]     |
[00:16:12] 
[00:16:12] error: aborting due to 2 previous errors
[00:16:12] error: aborting due to 2 previous errors
[00:16:12] 
[00:16:12] For more information about this error, try `rustc --explain E0412`.
[00:16:12] error: Could not compile `rustc_allocator`.
[00:16:12] 
[00:16:12] Caused by:
[00:16:12]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_allocator librustc_allocator/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=74cdf675d98e099e -C extra-filename=-74cdf675d98e099e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-4c5434c80172b18c.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-5073f1296cd24b67.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-0404335fb4ae3dc1.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-58741ed9de9aae4f.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-41b116eaee1e5535.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-a30c37d59a6fc275.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-88673787176f9d86/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-62d80197b9ec531e/out` (exit code: 101)
[00:17:34] error: build failed
[00:17:34] error: build failed
[00:17:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:17:34] expected success, got: exit code: 101
[00:17:34] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:17:34] travis_fold:end:stage0-rustc

[00:17:34] travis_time:end:stage0-rustc:start=1528245101186962730,finish=1528245857939140127,duration=756752177397


[00:17:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:17:34] Build completed unsuccessfully in 0:12:48
[00:17:34] make: *** [all] Error 1
[00:17:34] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:3585ef20
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)

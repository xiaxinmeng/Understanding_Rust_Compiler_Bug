plain
2019-08-25T02:26:35.2623226Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-25T02:26:35.2813241Z ##[command]git config gc.auto 0
2019-08-25T02:26:35.2896658Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-25T02:26:35.2987114Z ##[command]git config --get-all http.proxy
2019-08-25T02:26:35.3128999Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63861/merge:refs/remotes/pull/63861/merge
---
2019-08-25T02:27:09.3896877Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-25T02:27:09.3897283Z 
2019-08-25T02:27:09.3897703Z   git checkout -b <new-branch-name>
2019-08-25T02:27:09.3897951Z 
2019-08-25T02:27:09.3898174Z HEAD is now at cbf19cdeb Merge caf3d66966a9546e0703324d6fa2e552f3616e17 into eeba189cfb2cfc5c5898513352d4ca8f1df06e05
2019-08-25T02:27:09.4062079Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-25T02:27:09.4065227Z ==============================================================================
2019-08-25T02:27:09.4065306Z Task         : Bash
2019-08-25T02:27:09.4065351Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-25T03:05:07.3748359Z Successfully built f2e9d6d50b36
2019-08-25T03:05:07.4589443Z Successfully tagged rust-ci:latest
2019-08-25T03:05:07.5740913Z Built container sha256:f2e9d6d50b36935a1d027eaad3abe361e493aac1dc2abf0ef855909329ebe88c
2019-08-25T03:05:07.5766010Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/90b84d21282dbf471b95123e7af029c6260ac40a7e213c81b2df828a8e000d9b9fe76d8ac576f383d9b2886e81067c7468fed36b8ac5a1f9916a6f1f0f466b31
2019-08-25T03:06:12.1690848Z upload failed: - to s3://rust-lang-ci-sccache2/docker/90b84d21282dbf471b95123e7af029c6260ac40a7e213c81b2df828a8e000d9b9fe76d8ac576f383d9b2886e81067c7468fed36b8ac5a1f9916a6f1f0f466b31 Unable to locate credentials
2019-08-25T03:06:13.2166915Z [CI_JOB_NAME=dist-aarch64-linux]
2019-08-25T03:06:13.2208668Z == clock drift check ==
2019-08-25T03:06:13.2218900Z   local time: Sun Aug 25 03:06:13 UTC 2019
2019-08-25T03:06:13.3053249Z   network time: Sun, 25 Aug 2019 03:06:13 GMT
---
2019-08-25T04:53:00.8535996Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-08-25T04:53:02.0264251Z    Compiling backtrace-sys v0.1.30
2019-08-25T04:53:03.1384020Z    Compiling profiler_builtins v0.0.0 (/checkout/src/libprofiler_builtins)
2019-08-25T04:53:04.2394265Z    Compiling std v0.0.0 (/checkout/src/libstd)
2019-08-25T04:53:08.9142819Z error: internal compiler error: src/librustc/ty/context.rs:214: node type <T>::IntoIter (hir_id=HirId { owner: DefIndex(4040), local_id: 61 }) with HirId::owner DefId(0:4040 ~ core[db27]::iter[0]::adapters[0]::flatten[0]::{{impl}}[13]::try_fold[0]::flatten[0]) cannot be placed in TypeckTables with local_id_root DefId(0:4036 ~ core[db27]::iter[0]::adapters[0]::flatten[0]::{{impl}}[13]::try_fold[0])
2019-08-25T04:53:08.9153434Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
2019-08-25T04:53:08.9158569Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-25T04:53:09.0547093Z error: aborting due to previous error
2019-08-25T04:53:09.0550668Z 
2019-08-25T04:53:09.0550668Z 
2019-08-25T04:53:09.1425175Z 
2019-08-25T04:53:09.1425833Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-25T04:53:09.1426039Z 
2019-08-25T04:53:09.1427032Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-25T04:53:09.1427756Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-25T04:53:09.1427992Z 
2019-08-25T04:53:09.1427992Z 
2019-08-25T04:53:09.1428948Z note: compiler flags: -Z binary-dep-depinfo -Z external-macro-backtrace -Z save-analysis -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=0 -C prefer-dynamic -C linker=aarch64-unknown-linux-gnueabi-gcc -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
2019-08-25T04:53:09.1429235Z 
2019-08-25T04:53:09.1429467Z note: some of the compiler flags provided by cargo are hidden
2019-08-25T04:53:09.1502694Z error: Could not compile `core`.
2019-08-25T04:53:09.1502770Z 
2019-08-25T04:53:09.1503060Z To learn more, run the command again with --verbose.
2019-08-25T04:53:09.1529801Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "aarch64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-25T04:53:09.1529801Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "aarch64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-25T04:53:09.1529930Z expected success, got: exit code: 101
2019-08-25T04:53:09.1530230Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host aarch64-unknown-linux-gnu --target aarch64-unknown-linux-gnu
2019-08-25T04:53:09.1530306Z Build completed unsuccessfully in 1:44:23
2019-08-25T04:53:09.1581992Z == clock drift check ==
2019-08-25T04:53:09.1620781Z   local time: Sun Aug 25 04:53:09 UTC 2019
2019-08-25T04:53:09.4345531Z   network time: Sun, 25 Aug 2019 04:53:09 GMT
2019-08-25T04:53:09.4349762Z == end clock drift check ==
2019-08-25T04:53:15.1503271Z ##[error]Bash exited with code '1'.
2019-08-25T04:53:15.1569088Z ##[section]Starting: Checkout
2019-08-25T04:53:15.1570722Z ==============================================================================
2019-08-25T04:53:15.1570786Z Task         : Get sources
2019-08-25T04:53:15.1570824Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

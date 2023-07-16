plain
2019-08-24T18:36:48.2940404Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-24T18:36:48.3118551Z ##[command]git config gc.auto 0
2019-08-24T18:36:48.3190120Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-24T18:36:48.3239570Z ##[command]git config --get-all http.proxy
2019-08-24T18:36:48.3363002Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63861/merge:refs/remotes/pull/63861/merge
---
2019-08-24T18:37:22.3802568Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-24T18:37:22.3802594Z 
2019-08-24T18:37:22.3802766Z   git checkout -b <new-branch-name>
2019-08-24T18:37:22.3802789Z 
2019-08-24T18:37:22.3802828Z HEAD is now at 35ed47d49 Merge 4fed573e1ce18bff8cf80addcf68950930c110d9 into 5ade61a4f1515d4a18f38dacdbdb592bfd384a84
2019-08-24T18:37:22.3958307Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-24T18:37:22.3960601Z ==============================================================================
2019-08-24T18:37:22.3960645Z Task         : Bash
2019-08-24T18:37:22.3960701Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-24T19:14:09.6830459Z Successfully built 2cfb942592ed
2019-08-24T19:14:09.7655891Z Successfully tagged rust-ci:latest
2019-08-24T19:14:09.9000625Z Built container sha256:2cfb942592eda77d7dfa8d4bd44e62bf432f88d13673eaf7d8011b3ea851035e
2019-08-24T19:14:09.9028855Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/90b84d21282dbf471b95123e7af029c6260ac40a7e213c81b2df828a8e000d9b9fe76d8ac576f383d9b2886e81067c7468fed36b8ac5a1f9916a6f1f0f466b31
2019-08-24T19:15:10.1913464Z upload failed: - to s3://rust-lang-ci-sccache2/docker/90b84d21282dbf471b95123e7af029c6260ac40a7e213c81b2df828a8e000d9b9fe76d8ac576f383d9b2886e81067c7468fed36b8ac5a1f9916a6f1f0f466b31 Unable to locate credentials
2019-08-24T19:15:11.0906383Z [CI_JOB_NAME=dist-aarch64-linux]
2019-08-24T19:15:11.0947739Z == clock drift check ==
2019-08-24T19:15:11.0956344Z   local time: Sat Aug 24 19:15:11 UTC 2019
2019-08-24T19:15:11.2844414Z   network time: Sat, 24 Aug 2019 19:15:11 GMT
---
2019-08-24T20:57:29.4034685Z    Compiling backtrace-sys v0.1.30
2019-08-24T20:57:30.5399095Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-08-24T20:57:31.7911621Z    Compiling profiler_builtins v0.0.0 (/checkout/src/libprofiler_builtins)
2019-08-24T20:57:32.9650820Z    Compiling std v0.0.0 (/checkout/src/libstd)
2019-08-24T20:57:37.0975811Z error: internal compiler error: src/librustc/ty/context.rs:214: node type <T>::IntoIter (hir_id=HirId { owner: DefIndex(4040), local_id: 61 }) with HirId::owner DefId(0:4040 ~ core[db27]::iter[0]::adapters[0]::flatten[0]::{{impl}}[13]::try_fold[0]::flatten[0]) cannot be placed in TypeckTables with local_id_root DefId(0:4036 ~ core[db27]::iter[0]::adapters[0]::flatten[0]::{{impl}}[13]::try_fold[0])
2019-08-24T20:57:37.0977449Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
2019-08-24T20:57:37.0977975Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-24T20:57:37.2390058Z error: aborting due to previous error
2019-08-24T20:57:37.2390668Z 
2019-08-24T20:57:37.2390668Z 
2019-08-24T20:57:37.3244844Z 
2019-08-24T20:57:37.3247988Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-24T20:57:37.3261845Z 
2019-08-24T20:57:37.3263227Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-24T20:57:37.3263660Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-24T20:57:37.3263697Z 
2019-08-24T20:57:37.3263697Z 
2019-08-24T20:57:37.3264224Z note: compiler flags: -Z binary-dep-depinfo -Z external-macro-backtrace -Z save-analysis -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=0 -C prefer-dynamic -C linker=aarch64-unknown-linux-gnueabi-gcc -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
2019-08-24T20:57:37.3264283Z 
2019-08-24T20:57:37.3264338Z note: some of the compiler flags provided by cargo are hidden
2019-08-24T20:57:37.3399022Z error: Could not compile `core`.
2019-08-24T20:57:37.3399321Z warning: build failed, waiting for other jobs to finish...
2019-08-24T20:57:38.7927133Z error: build failed
2019-08-24T20:57:38.7959741Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "aarch64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-24T20:57:38.7959741Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "aarch64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-24T20:57:38.7960103Z expected success, got: exit code: 101
2019-08-24T20:57:38.7971922Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host aarch64-unknown-linux-gnu --target aarch64-unknown-linux-gnu
2019-08-24T20:57:38.7972035Z Build completed unsuccessfully in 1:40:02
2019-08-24T20:57:38.8023072Z == clock drift check ==
2019-08-24T20:57:38.8032819Z   local time: Sat Aug 24 20:57:38 UTC 2019
2019-08-24T20:57:38.9514005Z   network time: Sat, 24 Aug 2019 20:57:38 GMT
2019-08-24T20:57:38.9517821Z == end clock drift check ==
2019-08-24T20:57:41.5703361Z ##[error]Bash exited with code '1'.
2019-08-24T20:57:41.5749517Z ##[section]Starting: Checkout
2019-08-24T20:57:41.5751294Z ==============================================================================
2019-08-24T20:57:41.5751515Z Task         : Get sources
2019-08-24T20:57:41.5751555Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

plain
2019-08-22T20:41:34.6893619Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-22T20:41:34.7068930Z ##[command]git config gc.auto 0
2019-08-22T20:41:34.7141275Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-22T20:41:34.7183409Z ##[command]git config --get-all http.proxy
2019-08-22T20:41:34.7320118Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63814/merge:refs/remotes/pull/63814/merge
---
2019-08-22T20:42:10.2484615Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-22T20:42:10.2484645Z 
2019-08-22T20:42:10.2484855Z   git checkout -b <new-branch-name>
2019-08-22T20:42:10.2484884Z 
2019-08-22T20:42:10.2484947Z HEAD is now at c69c2031c Merge 654cf6383d6e9c7f4f2d4bd42b8abc25d4dcf9fe into 201e52e5fe73ccf3dd22946b1216ad8d64f8c2ba
2019-08-22T20:42:10.2657529Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-22T20:42:10.2660230Z ==============================================================================
2019-08-22T20:42:10.2660302Z Task         : Bash
2019-08-22T20:42:10.2660348Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-22T21:25:45.3017315Z    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
2019-08-22T21:25:45.8610649Z    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
2019-08-22T21:25:46.4393822Z    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
2019-08-22T21:25:47.0054537Z    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
2019-08-22T21:25:53.7400176Z error: internal compiler error: src/librustc/ty/context.rs:214: node type <T>::IntoIter (hir_id=HirId { owner: DefIndex(4040), local_id: 61 }) with HirId::owner DefId(0:4040 ~ core[db27]::iter[0]::adapters[0]::flatten[0]::{{impl}}[13]::try_fold[0]::flatten[0]) cannot be placed in TypeckTables with local_id_root DefId(0:4036 ~ core[db27]::iter[0]::adapters[0]::flatten[0]::{{impl}}[13]::try_fold[0])
2019-08-22T21:25:53.7401797Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
2019-08-22T21:25:53.7401907Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-22T21:25:53.9233884Z error: aborting due to previous error
2019-08-22T21:25:53.9234035Z 
2019-08-22T21:25:53.9234035Z 
2019-08-22T21:25:54.0250169Z 
2019-08-22T21:25:54.0257625Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-22T21:25:54.0257686Z 
2019-08-22T21:25:54.0259812Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-22T21:25:54.0260162Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-22T21:25:54.0260215Z 
2019-08-22T21:25:54.0260215Z 
2019-08-22T21:25:54.0260647Z note: compiler flags: -Z binary-dep-depinfo -Z external-macro-backtrace -Z save-analysis -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=0 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
2019-08-22T21:25:54.0260699Z 
2019-08-22T21:25:54.0260769Z note: some of the compiler flags provided by cargo are hidden
2019-08-22T21:25:54.0432953Z error: Could not compile `core`.
2019-08-22T21:25:54.0433080Z 
2019-08-22T21:25:54.0433364Z To learn more, run the command again with --verbose.
2019-08-22T21:25:54.0476310Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-22T21:25:54.0476310Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-22T21:25:54.0476460Z expected success, got: exit code: 101
2019-08-22T21:25:54.0484244Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --target x86_64-fuchsia,aarch64-fuchsia,wasm32-unknown-unknown,wasm32-wasi,sparcv9-sun-solaris,x86_64-sun-solaris,x86_64-unknown-linux-gnux32,x86_64-unknown-cloudabi,x86_64-fortanix-unknown-sgx,nvptx64-nvidia-cuda,armv7-unknown-linux-gnueabi,armv7-unknown-linux-musleabi
2019-08-22T21:25:54.0484419Z Build completed unsuccessfully in 0:36:46
2019-08-22T21:25:54.0541524Z == clock drift check ==
2019-08-22T21:25:54.0566829Z   local time: Thu Aug 22 21:25:54 UTC 2019
2019-08-22T21:25:54.2063847Z   network time: Thu, 22 Aug 2019 21:25:54 GMT
2019-08-22T21:25:54.2063960Z == end clock drift check ==
2019-08-22T21:25:58.5054243Z ##[error]Bash exited with code '1'.
2019-08-22T21:25:58.5092799Z ##[section]Starting: Checkout
2019-08-22T21:25:58.5094434Z ==============================================================================
2019-08-22T21:25:58.5094489Z Task         : Get sources
2019-08-22T21:25:58.5094549Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

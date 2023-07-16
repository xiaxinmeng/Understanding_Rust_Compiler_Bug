plain
2019-08-22T18:01:43.8693512Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-22T18:01:43.8902922Z ##[command]git config gc.auto 0
2019-08-22T18:01:43.8974363Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-22T18:01:43.9019335Z ##[command]git config --get-all http.proxy
2019-08-22T18:01:43.9161045Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63814/merge:refs/remotes/pull/63814/merge
---
2019-08-22T18:02:19.6752137Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-22T18:02:19.6752473Z 
2019-08-22T18:02:19.6755469Z   git checkout -b <new-branch-name>
2019-08-22T18:02:19.6756814Z 
2019-08-22T18:02:19.6758367Z HEAD is now at 44972fb92 Merge c238e458464d369a52f2d32e5398e257c804b4fc into 201e52e5fe73ccf3dd22946b1216ad8d64f8c2ba
2019-08-22T18:02:19.6928377Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-22T18:02:19.6930799Z ==============================================================================
2019-08-22T18:02:19.6930848Z Task         : Bash
2019-08-22T18:02:19.6930889Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-22T18:46:50.7472692Z    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
2019-08-22T18:46:51.3252626Z    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
2019-08-22T18:46:51.9539364Z    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
2019-08-22T18:46:52.5358396Z    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
2019-08-22T18:46:58.6442922Z error: internal compiler error: src/librustc/ty/context.rs:214: node type <T>::IntoIter (hir_id=HirId { owner: DefIndex(4040), local_id: 61 }) with HirId::owner DefId(0:4040 ~ core[db27]::iter[0]::adapters[0]::flatten[0]::{{impl}}[13]::try_fold[0]::flatten[0]) cannot be placed in TypeckTables with local_id_root DefId(0:4036 ~ core[db27]::iter[0]::adapters[0]::flatten[0]::{{impl}}[13]::try_fold[0])
2019-08-22T18:46:58.6451542Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
2019-08-22T18:46:58.6456711Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-22T18:46:58.8385634Z error: aborting due to previous error
2019-08-22T18:46:58.8390188Z 
2019-08-22T18:46:58.8390188Z 
2019-08-22T18:46:58.9468751Z 
2019-08-22T18:46:58.9475151Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-22T18:46:58.9475254Z 
2019-08-22T18:46:58.9476840Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-22T18:46:58.9482239Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-22T18:46:58.9483680Z 
2019-08-22T18:46:58.9483680Z 
2019-08-22T18:46:58.9526970Z note: compiler flags: -Z binary-dep-depinfo -Z external-macro-backtrace -Z save-analysis -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=0 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
2019-08-22T18:46:58.9527071Z 
2019-08-22T18:46:58.9527148Z note: some of the compiler flags provided by cargo are hidden
2019-08-22T18:46:58.9619109Z error: Could not compile `core`.
2019-08-22T18:46:58.9623600Z warning: build failed, waiting for other jobs to finish...
2019-08-22T18:46:59.7478978Z error: build failed
2019-08-22T18:46:59.7496671Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-22T18:46:59.7496671Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-22T18:46:59.7496826Z expected success, got: exit code: 101
2019-08-22T18:46:59.7509001Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --target x86_64-fuchsia,aarch64-fuchsia,wasm32-unknown-unknown,wasm32-wasi,sparcv9-sun-solaris,x86_64-sun-solaris,x86_64-unknown-linux-gnux32,x86_64-unknown-cloudabi,x86_64-fortanix-unknown-sgx,nvptx64-nvidia-cuda,armv7-unknown-linux-gnueabi,armv7-unknown-linux-musleabi
2019-08-22T18:46:59.7509098Z Build completed unsuccessfully in 0:37:29
2019-08-22T18:46:59.7568506Z == clock drift check ==
2019-08-22T18:46:59.7578642Z   local time: Thu Aug 22 18:46:59 UTC 2019
2019-08-22T18:46:59.9082190Z   network time: Thu, 22 Aug 2019 18:46:59 GMT
2019-08-22T18:46:59.9085764Z == end clock drift check ==
2019-08-22T18:47:04.3119548Z ##[error]Bash exited with code '1'.
2019-08-22T18:47:04.3168430Z ##[section]Starting: Checkout
2019-08-22T18:47:04.3170676Z ==============================================================================
2019-08-22T18:47:04.3170764Z Task         : Get sources
2019-08-22T18:47:04.3170814Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

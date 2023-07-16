plain
2019-07-30T20:37:54.9719935Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-30T20:37:54.9892734Z ##[command]git config gc.auto 0
2019-07-30T20:37:54.9957535Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-30T20:37:55.0007374Z ##[command]git config --get-all http.proxy
2019-07-30T20:37:55.0144145Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63146/merge:refs/remotes/pull/63146/merge
---
2019-07-30T20:38:29.1497317Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-30T20:38:29.1498798Z 
2019-07-30T20:38:29.1501126Z   git checkout -b <new-branch-name>
2019-07-30T20:38:29.1502582Z 
2019-07-30T20:38:29.1504000Z HEAD is now at bede0c8fc Merge b8ad424cfe62aecb0477e6a1ea5ca1a3d5fb03d8 into f690098e6d65ad7b33dc7fdefccc387806782027
2019-07-30T20:38:29.1648050Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-30T20:38:29.1651191Z ==============================================================================
2019-07-30T20:38:29.1651249Z Task         : Bash
2019-07-30T20:38:29.1651309Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-30T21:07:15.5670265Z    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
2019-07-30T21:07:16.1852654Z    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
2019-07-30T21:07:16.8104767Z    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
2019-07-30T21:07:17.5346897Z    Compiling rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
2019-07-30T21:07:17.6732188Z error[E0658]: internal implementation detail
2019-07-30T21:07:17.6738065Z   --> /cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.18/src/float/cmp.rs:7:1
2019-07-30T21:07:17.6742917Z    |
2019-07-30T21:07:17.6748894Z 7  | / enum Result {
2019-07-30T21:07:17.6754373Z 8  | |     Less,
2019-07-30T21:07:17.6755596Z 9  | |     Equal,
2019-07-30T21:07:17.6755911Z 10 | |     Greater,
2019-07-30T21:07:17.6756234Z 11 | |     Unordered,
2019-07-30T21:07:17.6761413Z    | |_^
2019-07-30T21:07:17.6761642Z    |
2019-07-30T21:07:17.6761642Z    |
2019-07-30T21:07:17.6762099Z    = note: for more information, see ***/issues/29642
2019-07-30T21:07:17.6765868Z    = help: add `#![feature(rustc_attrs)]` to the crate attributes to enable
2019-07-30T21:07:18.4554002Z error: aborting due to previous error
2019-07-30T21:07:18.4554962Z 
2019-07-30T21:07:18.4555673Z For more information about this error, try `rustc --explain E0658`.
2019-07-30T21:07:18.4642523Z error: Could not compile `compiler_builtins`.
2019-07-30T21:07:18.4642523Z error: Could not compile `compiler_builtins`.
2019-07-30T21:07:18.4652717Z warning: build failed, waiting for other jobs to finish...
2019-07-30T21:07:19.2343488Z error: build failed
2019-07-30T21:07:19.2357206Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-07-30T21:07:19.2357411Z expected success, got: exit code: 101
2019-07-30T21:07:19.2368919Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-30T21:07:19.2369944Z Build completed unsuccessfully in 0:22:40
2019-07-30T21:07:21.9427236Z ##[error]Bash exited with code '1'.
2019-07-30T21:07:21.9465372Z ##[section]Starting: Checkout
2019-07-30T21:07:21.9467465Z ==============================================================================
2019-07-30T21:07:21.9467523Z Task         : Get sources
2019-07-30T21:07:21.9467605Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

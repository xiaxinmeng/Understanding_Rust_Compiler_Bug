plain
2019-07-30T19:14:22.1101588Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-30T19:14:22.1285474Z ##[command]git config gc.auto 0
2019-07-30T19:14:22.1356949Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-30T19:14:22.1419780Z ##[command]git config --get-all http.proxy
2019-07-30T19:14:22.1565803Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63146/merge:refs/remotes/pull/63146/merge
---
2019-07-30T19:14:57.9740526Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-30T19:14:57.9740570Z 
2019-07-30T19:14:57.9740775Z   git checkout -b <new-branch-name>
2019-07-30T19:14:57.9740801Z 
2019-07-30T19:14:57.9740841Z HEAD is now at 4e5b9fdf3 Merge 12766c9392dd00a663d8607060d13a8d42b907f8 into f690098e6d65ad7b33dc7fdefccc387806782027
2019-07-30T19:14:57.9929242Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-30T19:14:57.9932193Z ==============================================================================
2019-07-30T19:14:57.9932246Z Task         : Bash
2019-07-30T19:14:57.9932287Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-30T19:46:09.8463138Z    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
2019-07-30T19:46:10.4916559Z    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
2019-07-30T19:46:10.5631163Z    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
2019-07-30T19:46:11.3108028Z    Compiling rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
2019-07-30T19:46:11.4466630Z error[E0658]: internal implementation detail
2019-07-30T19:46:11.4467188Z   --> /cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.18/src/float/cmp.rs:7:1
2019-07-30T19:46:11.4467441Z    |
2019-07-30T19:46:11.4467731Z 7  | / enum Result {
2019-07-30T19:46:11.4468044Z 8  | |     Less,
2019-07-30T19:46:11.4468348Z 9  | |     Equal,
2019-07-30T19:46:11.4468636Z 10 | |     Greater,
2019-07-30T19:46:11.4468925Z 11 | |     Unordered,
2019-07-30T19:46:11.4469485Z    | |_^
2019-07-30T19:46:11.4469692Z    |
2019-07-30T19:46:11.4469692Z    |
2019-07-30T19:46:11.4470385Z    = note: for more information, see ***/issues/29642
2019-07-30T19:46:11.4470776Z    = help: add `#![feature(rustc_attrs)]` to the crate attributes to enable
2019-07-30T19:46:12.2629600Z error: aborting due to previous error
2019-07-30T19:46:12.2629767Z 
2019-07-30T19:46:12.2630050Z For more information about this error, try `rustc --explain E0658`.
2019-07-30T19:46:12.2715141Z error: Could not compile `compiler_builtins`.
2019-07-30T19:46:12.2715141Z error: Could not compile `compiler_builtins`.
2019-07-30T19:46:12.2715782Z warning: build failed, waiting for other jobs to finish...
2019-07-30T19:46:13.2851704Z error: build failed
2019-07-30T19:46:13.2873397Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-07-30T19:46:13.2873568Z expected success, got: exit code: 101
2019-07-30T19:46:13.2885513Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-30T19:46:13.2885815Z Build completed unsuccessfully in 0:24:20
2019-07-30T19:46:15.9256415Z ##[error]Bash exited with code '1'.
2019-07-30T19:46:15.9293104Z ##[section]Starting: Checkout
2019-07-30T19:46:15.9294971Z ==============================================================================
2019-07-30T19:46:15.9295163Z Task         : Get sources
2019-07-30T19:46:15.9295205Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

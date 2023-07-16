plain
2019-12-01T22:02:32.3698109Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-01T22:02:32.3713102Z ##[command]git config gc.auto 0
2019-12-01T22:02:32.3716705Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-01T22:02:32.3720980Z ##[command]git config --get-all http.proxy
2019-12-01T22:02:32.3724820Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66942/merge:refs/remotes/pull/66942/merge
---
2019-12-01T22:31:26.1694852Z fatal runtime error: stack overflow
2019-12-01T22:31:26.4482103Z error: could not compile `core`.
2019-12-01T22:31:26.4488313Z 
2019-12-01T22:31:26.4488461Z Caused by:
2019-12-01T22:31:26.4497869Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --edition=2018 --crate-name core src/libcore/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=2 -C codegen-units=1 -C debuginfo=0 -C metadata=8dab76f6898d904f -C extra-filename=-8dab76f6898d904f --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -Zexternal-macro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings -Cprefer-dynamic -Zbinary-dep-depinfo` (signal: 6, SIGABRT: process abort signal)
2019-12-01T22:31:28.2430796Z error: build failed
2019-12-01T22:31:28.2458289Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-01T22:31:28.2458491Z expected success, got: exit code: 101
2019-12-01T22:31:28.2473116Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-01T22:31:28.2473116Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-01T22:31:28.2473434Z Build completed unsuccessfully in 0:22:54
2019-12-01T22:31:28.2541034Z == clock drift check ==
2019-12-01T22:31:28.2558256Z   local time: Sun Dec  1 22:31:28 UTC 2019
2019-12-01T22:31:28.5220017Z   network time: Sun, 01 Dec 2019 22:31:28 GMT
2019-12-01T22:31:28.5224052Z == end clock drift check ==
2019-12-01T22:31:29.2487314Z 
2019-12-01T22:31:29.2599790Z ##[error]Bash exited with code '1'.
2019-12-01T22:31:29.2633297Z ##[section]Starting: Checkout
2019-12-01T22:31:29.2634905Z ==============================================================================
2019-12-01T22:31:29.2634953Z Task         : Get sources
2019-12-01T22:31:29.2634994Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

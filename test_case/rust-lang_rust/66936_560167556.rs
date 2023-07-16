plain
2019-12-01T21:59:42.4907890Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-01T21:59:42.5095167Z ##[command]git config gc.auto 0
2019-12-01T21:59:42.5164552Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-01T21:59:42.5218571Z ##[command]git config --get-all http.proxy
2019-12-01T21:59:42.5357626Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66936/merge:refs/remotes/pull/66936/merge
---
2019-12-01T22:26:45.8779019Z fatal runtime error: stack overflow
2019-12-01T22:26:46.1910746Z error: could not compile `core`.
2019-12-01T22:26:46.1914508Z 
2019-12-01T22:26:46.1914641Z Caused by:
2019-12-01T22:26:46.1922034Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --edition=2018 --crate-name core src/libcore/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=2 -C codegen-units=1 -C debuginfo=0 -C metadata=8dab76f6898d904f -C extra-filename=-8dab76f6898d904f --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -Zexternal-macro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings -Cprefer-dynamic -Zbinary-dep-depinfo` (signal: 6, SIGABRT: process abort signal)
2019-12-01T22:26:47.5996194Z error: build failed
2019-12-01T22:26:47.6020038Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-01T22:26:47.6020382Z expected success, got: exit code: 101
2019-12-01T22:26:47.6028860Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-01T22:26:47.6028860Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-01T22:26:47.6029249Z Build completed unsuccessfully in 0:21:09
2019-12-01T22:26:47.6082768Z == clock drift check ==
2019-12-01T22:26:47.6101030Z   local time: Sun Dec  1 22:26:47 UTC 2019
2019-12-01T22:26:47.8913491Z   network time: Sun, 01 Dec 2019 22:26:47 GMT
2019-12-01T22:26:47.8916626Z == end clock drift check ==
2019-12-01T22:26:48.6695824Z 
2019-12-01T22:26:48.6783844Z ##[error]Bash exited with code '1'.
2019-12-01T22:26:48.6838521Z ##[section]Starting: Checkout
2019-12-01T22:26:48.6840667Z ==============================================================================
2019-12-01T22:26:48.6840731Z Task         : Get sources
2019-12-01T22:26:48.6840780Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

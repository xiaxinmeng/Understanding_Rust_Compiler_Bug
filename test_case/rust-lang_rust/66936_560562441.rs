plain
2019-12-02T19:49:52.0883694Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-02T19:49:52.1073113Z ##[command]git config gc.auto 0
2019-12-02T19:49:52.1134435Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-02T19:49:52.1198558Z ##[command]git config --get-all http.proxy
2019-12-02T19:49:52.1323834Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66936/merge:refs/remotes/pull/66936/merge
---
2019-12-02T20:15:55.0260053Z fatal runtime error: stack overflow
2019-12-02T20:15:55.3125594Z error: could not compile `core`.
2019-12-02T20:15:55.3125719Z 
2019-12-02T20:15:55.3125762Z Caused by:
2019-12-02T20:15:55.3136067Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --edition=2018 --crate-name core src/libcore/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=2 -C codegen-units=1 -C debuginfo=0 -C metadata=8dab76f6898d904f -C extra-filename=-8dab76f6898d904f --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -Zexternal-macro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings -Cprefer-dynamic -Zbinary-dep-depinfo` (signal: 6, SIGABRT: process abort signal)
2019-12-02T20:15:56.9123561Z error: build failed
2019-12-02T20:15:56.9140676Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-02T20:15:56.9140809Z expected success, got: exit code: 101
2019-12-02T20:15:56.9153845Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-02T20:15:56.9153845Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-02T20:15:56.9153943Z Build completed unsuccessfully in 0:20:35
2019-12-02T20:15:56.9202769Z == clock drift check ==
2019-12-02T20:15:56.9218274Z   local time: Mon Dec  2 20:15:56 UTC 2019
2019-12-02T20:15:57.1877585Z   network time: Mon, 02 Dec 2019 20:15:57 GMT
2019-12-02T20:15:57.1881821Z == end clock drift check ==
2019-12-02T20:15:57.8993319Z 
2019-12-02T20:15:57.9091492Z ##[error]Bash exited with code '1'.
2019-12-02T20:15:57.9126620Z ##[section]Starting: Checkout
2019-12-02T20:15:57.9128281Z ==============================================================================
2019-12-02T20:15:57.9128330Z Task         : Get sources
2019-12-02T20:15:57.9128372Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

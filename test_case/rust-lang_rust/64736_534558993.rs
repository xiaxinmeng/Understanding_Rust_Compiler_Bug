plain
2019-09-24T13:10:24.1463816Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-24T13:10:24.1657059Z ##[command]git config gc.auto 0
2019-09-24T13:10:24.1748206Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-24T13:10:24.6919427Z ##[command]git config --get-all http.proxy
2019-09-24T13:10:24.6924411Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64736/merge:refs/remotes/pull/64736/merge
---
2019-09-24T13:17:33.4353433Z    Compiling serde_json v1.0.40
2019-09-24T13:17:35.2442011Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-24T13:17:46.0510383Z     Finished release [optimized] target(s) in 1m 29s
2019-09-24T13:17:46.0605631Z tidy check
2019-09-24T13:17:46.3993955Z tidy error: /checkout/src/librustc/mir/mod.rs:233: TODO is deprecated; use FIXME
2019-09-24T13:17:48.1038100Z some tidy checks failed
2019-09-24T13:17:48.1042233Z 
2019-09-24T13:17:48.1042233Z 
2019-09-24T13:17:48.1043351Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-24T13:17:48.1047127Z 
2019-09-24T13:17:48.1053982Z 
2019-09-24T13:17:48.1054108Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-24T13:17:48.1055024Z Build completed unsuccessfully in 0:01:32
2019-09-24T13:17:48.1055024Z Build completed unsuccessfully in 0:01:32
2019-09-24T13:17:48.1107557Z == clock drift check ==
2019-09-24T13:17:48.1124722Z   local time: Tue Sep 24 13:17:48 UTC 2019
2019-09-24T13:17:48.2620173Z   network time: Tue, 24 Sep 2019 13:17:48 GMT
2019-09-24T13:17:48.2622742Z == end clock drift check ==
2019-09-24T13:17:49.6692301Z ##[error]Bash exited with code '1'.
2019-09-24T13:17:49.6728211Z ##[section]Starting: Checkout
2019-09-24T13:17:49.6730073Z ==============================================================================
2019-09-24T13:17:49.6730120Z Task         : Get sources
2019-09-24T13:17:49.6730180Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

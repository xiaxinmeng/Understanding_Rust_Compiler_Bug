plain
2019-11-02T10:36:23.2144980Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-02T10:36:23.2344152Z ##[command]git config gc.auto 0
2019-11-02T10:36:23.2417877Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-02T10:36:23.2473935Z ##[command]git config --get-all http.proxy
2019-11-02T10:36:23.2622235Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65385/merge:refs/remotes/pull/65385/merge
---
2019-11-02T10:42:03.1811906Z    Compiling serde_json v1.0.40
2019-11-02T10:42:04.6769286Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-02T10:42:14.4485428Z     Finished release [optimized] target(s) in 1m 15s
2019-11-02T10:42:14.4558727Z tidy check
2019-11-02T10:42:14.8431677Z tidy error: /checkout/src/librustc_mir/dataflow/move_paths/builder.rs:359: TODO is deprecated; use FIXME
2019-11-02T10:42:16.4383434Z some tidy checks failed
2019-11-02T10:42:16.4387518Z Found 485 error codes
2019-11-02T10:42:16.4387619Z Found 0 error codes with no tests
2019-11-02T10:42:16.4388749Z Done!
2019-11-02T10:42:16.4388749Z Done!
2019-11-02T10:42:16.4388807Z 
2019-11-02T10:42:16.4388834Z 
2019-11-02T10:42:16.4389736Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-02T10:42:16.4389866Z 
2019-11-02T10:42:16.4389911Z 
2019-11-02T10:42:16.4399907Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-02T10:42:16.4400001Z Build completed unsuccessfully in 0:01:18
2019-11-02T10:42:16.4400001Z Build completed unsuccessfully in 0:01:18
2019-11-02T10:42:16.4450592Z == clock drift check ==
2019-11-02T10:42:16.4457950Z   local time: Sat Nov  2 10:42:16 UTC 2019
2019-11-02T10:42:16.5813137Z   network time: Sat, 02 Nov 2019 10:42:16 GMT
2019-11-02T10:42:16.5818945Z == end clock drift check ==
2019-11-02T10:42:17.9651889Z 
2019-11-02T10:42:17.9770797Z ##[error]Bash exited with code '1'.
2019-11-02T10:42:17.9795221Z ##[section]Starting: Checkout
2019-11-02T10:42:17.9796977Z ==============================================================================
2019-11-02T10:42:17.9797056Z Task         : Get sources
2019-11-02T10:42:17.9797099Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

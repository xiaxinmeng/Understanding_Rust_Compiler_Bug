plain
2019-12-14T23:24:36.2780507Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-14T23:24:36.8098211Z ##[command]git config gc.auto 0
2019-12-14T23:24:36.8102277Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-14T23:24:36.8107220Z ##[command]git config --get-all http.proxy
2019-12-14T23:24:36.8111011Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67304/merge:refs/remotes/pull/67304/merge
---
2019-12-14T23:31:16.3459502Z     Checking rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2019-12-14T23:31:17.9130756Z error[E0624]: method `current_num_threads` is private
2019-12-14T23:31:17.9131383Z    --> src/librustc_data_structures/sync.rs:440:39
2019-12-14T23:31:17.9132007Z     |
2019-12-14T23:31:17.9132887Z 440 |                 SharedWorkerLocal((0..Registry::current_num_threads()).map(|i| f(i)).collect())
2019-12-14T23:31:17.9134461Z 
2019-12-14T23:31:17.9207962Z error: aborting due to previous error
2019-12-14T23:31:17.9244710Z 
2019-12-14T23:31:17.9245709Z For more information about this error, try `rustc --explain E0624`.
2019-12-14T23:31:17.9245709Z For more information about this error, try `rustc --explain E0624`.
2019-12-14T23:31:17.9285132Z error: could not compile `rustc_data_structures`.
2019-12-14T23:31:17.9285459Z warning: build failed, waiting for other jobs to finish...
2019-12-14T23:31:28.1744213Z error: build failed
2019-12-14T23:31:28.1774189Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-14T23:31:28.1785767Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-12-14T23:31:28.1786009Z Build completed unsuccessfully in 0:03:42
2019-12-14T23:31:28.1839843Z == clock drift check ==
2019-12-14T23:31:28.1853320Z   local time: Sat Dec 14 23:31:28 UTC 2019
2019-12-14T23:31:28.1853320Z   local time: Sat Dec 14 23:31:28 UTC 2019
2019-12-14T23:31:28.4619487Z   network time: Sat, 14 Dec 2019 23:31:28 GMT
2019-12-14T23:31:28.4628870Z == end clock drift check ==
2019-12-14T23:31:29.6912837Z 
2019-12-14T23:31:29.7020620Z ##[error]Bash exited with code '1'.
2019-12-14T23:31:29.7053223Z ##[section]Starting: Checkout
2019-12-14T23:31:29.7055044Z ==============================================================================
2019-12-14T23:31:29.7055101Z Task         : Get sources
2019-12-14T23:31:29.7055168Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

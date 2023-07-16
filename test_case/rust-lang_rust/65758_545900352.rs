plain
2019-10-24T10:05:12.4284293Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-24T10:05:12.4486407Z ##[command]git config gc.auto 0
2019-10-24T10:05:12.4565972Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-24T10:05:12.4630492Z ##[command]git config --get-all http.proxy
2019-10-24T10:05:12.4781046Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65758/merge:refs/remotes/pull/65758/merge
---
2019-10-24T11:55:11.8680806Z     Finished release [optimized] target(s) in 8m 54s
2019-10-24T11:57:10.2354193Z Error: there are broken links
2019-10-24T11:57:10.2355943Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html" returned 404 Not Found
2019-10-24T11:57:10.2358030Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html#method.buffer_lint" returned 404 Not Found
2019-10-24T11:57:10.2358990Z  Caused By: "***/tree/master/src/libsyntax/ext/tt" returned 404 Not Found
2019-10-24T11:57:10.2360065Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/fn.parse.html" returned 404 Not Found
2019-10-24T11:57:10.2360538Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_rules/" returned 404 Not Found
2019-10-24T11:57:10.2361034Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/" returned 404 Not Found
2019-10-24T11:57:10.2361491Z  Caused By: "https://github.com/rust-lang/chalk/blob/master/src/test/wf.rs" returned 404 Not Found
---
2019-10-24T12:42:52.4321093Z    Compiling rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
2019-10-24T12:44:44.6332188Z     Finished release [optimized] target(s) in 3m 25s
2019-10-24T12:44:44.6634596Z     Finished release [optimized] target(s) in 0.22s
2019-10-24T12:44:44.6795472Z      Running `build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo-miri miri setup`
2019-10-24T12:44:44.6831431Z fatal error: Your xargo is too old; please upgrade to the latest version
2019-10-24T12:44:44.6834823Z 
2019-10-24T12:44:44.6837932Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "run" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/miri/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--bin" "cargo-miri" "--" "miri" "setup"
2019-10-24T12:44:44.6839119Z expected success, got: exit code: 1
2019-10-24T12:44:44.6839296Z 
---
2019-10-24T12:44:44.7038630Z Verifying status of clippy-driver...
2019-10-24T12:44:44.7053024Z Verifying status of miri...
2019-10-24T12:44:44.7066937Z This PR updated 'src/tools/miri', verifying if status is 'test-pass'...
2019-10-24T12:44:44.7083444Z 
2019-10-24T12:44:44.7084321Z ⚠️ We detected that this PR updated 'miri', but its tests failed.
2019-10-24T12:44:44.7084533Z 
2019-10-24T12:44:44.7085019Z If you do intend to update 'miri', please check the error messages above and
2019-10-24T12:44:44.7085076Z commit another update.
2019-10-24T12:44:44.7085124Z 
2019-10-24T12:44:44.7085381Z If you do NOT intend to update 'miri', please ensure you did not accidentally
2019-10-24T12:44:44.7085635Z change the submodule at 'src/tools/miri'. You may ask your reviewer for the
2019-10-24T12:44:44.7085704Z proper steps.
2019-10-24T12:44:44.7168490Z   local time: Thu Oct 24 12:44:44 UTC 2019
2019-10-24T12:44:45.0044853Z   network time: Thu, 24 Oct 2019 12:44:44 GMT
2019-10-24T12:44:45.0044975Z == end clock drift check ==
2019-10-24T12:44:46.0729732Z 
2019-10-24T12:44:46.0729732Z 
2019-10-24T12:44:46.0840806Z ##[error]Bash exited with code '3'.
2019-10-24T12:44:46.0879088Z ##[section]Starting: Checkout
2019-10-24T12:44:46.0881097Z ==============================================================================
2019-10-24T12:44:46.0881171Z Task         : Get sources
2019-10-24T12:44:46.0881222Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

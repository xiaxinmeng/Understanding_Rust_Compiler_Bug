plain
2019-11-09T18:13:38.8141241Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-09T18:13:38.8365445Z ##[command]git config gc.auto 0
2019-11-09T18:13:38.8441052Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-09T18:13:39.7835522Z ##[command]git config --get-all http.proxy
2019-11-09T18:13:39.7846700Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66207/merge:refs/remotes/pull/66207/merge
---
2019-11-09T19:51:40.0001956Z     Finished release [optimized] target(s) in 10m 36s
2019-11-09T19:53:22.3735368Z Error: there are broken links
2019-11-09T19:53:22.3737323Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html" returned 404 Not Found
2019-11-09T19:53:22.3739207Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html#method.buffer_lint" returned 404 Not Found
2019-11-09T19:53:22.3740111Z  Caused By: "***/tree/master/src/libsyntax/ext/tt" returned 404 Not Found
2019-11-09T19:53:22.3741503Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/fn.parse.html" returned 404 Not Found
2019-11-09T19:53:22.3742148Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_rules/" returned 404 Not Found
2019-11-09T19:53:22.3742890Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/" returned 404 Not Found
2019-11-09T19:53:22.3743476Z  Caused By: "https://github.com/rust-lang/chalk/blob/master/src/test/wf.rs" returned 404 Not Found
---
2019-11-09T20:34:48.7449173Z Verifying status of rustfmt...
2019-11-09T20:34:48.7460528Z Verifying status of clippy-driver...
2019-11-09T20:34:48.7473474Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-11-09T20:34:48.7483084Z 
2019-11-09T20:34:48.7484261Z ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
2019-11-09T20:34:48.7484967Z 
2019-11-09T20:34:48.7485272Z If you do intend to update 'clippy-driver', please check the error messages above and
2019-11-09T20:34:48.7485322Z commit another update.
2019-11-09T20:34:48.7485349Z 
2019-11-09T20:34:48.7485800Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2019-11-09T20:34:48.7486083Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2019-11-09T20:34:48.7486131Z proper steps.
2019-11-09T20:34:48.7507458Z   local time: Sat Nov  9 20:34:48 UTC 2019
2019-11-09T20:34:48.9102696Z   network time: Sat, 09 Nov 2019 20:34:48 GMT
2019-11-09T20:34:48.9106505Z == end clock drift check ==
2019-11-09T20:34:49.4390108Z 
2019-11-09T20:34:49.4390108Z 
2019-11-09T20:34:49.4472644Z ##[error]Bash exited with code '3'.
2019-11-09T20:34:49.4514677Z ##[section]Starting: Checkout
2019-11-09T20:34:49.4516342Z ==============================================================================
2019-11-09T20:34:49.4516390Z Task         : Get sources
2019-11-09T20:34:49.4516448Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

plain
2019-10-19T20:45:17.6780037Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-19T20:45:17.7009589Z ##[command]git config gc.auto 0
2019-10-19T20:45:17.7103362Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-19T20:45:17.7168705Z ##[command]git config --get-all http.proxy
2019-10-19T20:45:17.7318931Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65609/merge:refs/remotes/pull/65609/merge
---
2019-10-19T22:40:20.1979358Z     Finished release [optimized] target(s) in 9m 14s
2019-10-19T22:42:00.7805321Z Error: there are broken links
2019-10-19T22:42:00.7806647Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html" returned 404 Not Found
2019-10-19T22:42:00.7808230Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html#method.buffer_lint" returned 404 Not Found
2019-10-19T22:42:00.7809918Z  Caused By: "***/tree/master/src/libsyntax/ext/tt" returned 404 Not Found
2019-10-19T22:42:00.7811151Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/fn.parse.html" returned 404 Not Found
2019-10-19T22:42:00.7811646Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_rules/" returned 404 Not Found
2019-10-19T22:42:00.7811932Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/" returned 404 Not Found
2019-10-19T22:42:00.7812613Z  Caused By: "https://github.com/rust-lang/chalk/blob/master/src/test/wf.rs" returned 404 Not Found
---
2019-10-19T23:30:06.6896128Z Verifying status of clippy-driver...
2019-10-19T23:30:06.6911708Z Verifying status of miri...
2019-10-19T23:30:06.6926536Z This PR updated 'src/tools/miri', verifying if status is 'test-pass'...
2019-10-19T23:30:06.6941306Z 
2019-10-19T23:30:06.6942323Z ⚠️ We detected that this PR updated 'miri', but its tests failed.
2019-10-19T23:30:06.6942603Z 
2019-10-19T23:30:06.6942930Z If you do intend to update 'miri', please check the error messages above and
2019-10-19T23:30:06.6943061Z commit another update.
2019-10-19T23:30:06.6943088Z 
2019-10-19T23:30:06.6943344Z If you do NOT intend to update 'miri', please ensure you did not accidentally
2019-10-19T23:30:06.6943601Z change the submodule at 'src/tools/miri'. You may ask your reviewer for the
2019-10-19T23:30:06.6943667Z proper steps.
2019-10-19T23:30:06.6967145Z   local time: Sat Oct 19 23:30:06 UTC 2019
2019-10-19T23:30:06.8541843Z   network time: Sat, 19 Oct 2019 23:30:06 GMT
2019-10-19T23:30:06.8545442Z == end clock drift check ==
2019-10-19T23:30:08.3311894Z 
2019-10-19T23:30:08.3311894Z 
2019-10-19T23:30:08.3440452Z ##[error]Bash exited with code '3'.
2019-10-19T23:30:08.3491300Z ##[section]Starting: Checkout
2019-10-19T23:30:08.3497194Z ==============================================================================
2019-10-19T23:30:08.3497306Z Task         : Get sources
2019-10-19T23:30:08.3497353Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

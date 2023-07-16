plain
2019-10-25T19:26:19.4782447Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-25T19:26:19.4964278Z ##[command]git config gc.auto 0
2019-10-25T19:26:19.5040280Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-25T19:26:19.5086202Z ##[command]git config --get-all http.proxy
2019-10-25T19:26:19.5224197Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65829/merge:refs/remotes/pull/65829/merge
---
2019-10-25T21:10:07.2039364Z     Finished release [optimized] target(s) in 8m 00s
2019-10-25T21:11:47.0730490Z Error: there are broken links
2019-10-25T21:11:47.0731817Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html" returned 404 Not Found
2019-10-25T21:11:47.0732716Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html#method.buffer_lint" returned 404 Not Found
2019-10-25T21:11:47.0733595Z  Caused By: "***/tree/master/src/libsyntax/ext/tt" returned 404 Not Found
2019-10-25T21:11:47.0734690Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/fn.parse.html" returned 404 Not Found
2019-10-25T21:11:47.0735167Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_rules/" returned 404 Not Found
2019-10-25T21:11:47.0735661Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/" returned 404 Not Found
2019-10-25T21:11:47.0736116Z  Caused By: "https://github.com/rust-lang/chalk/blob/master/src/test/wf.rs" returned 404 Not Found
---
2019-10-25T21:53:37.9678125Z This PR updated 'src/tools/rustfmt', verifying if status is 'test-pass'...
2019-10-25T21:53:37.9689892Z Verifying status of clippy-driver...
2019-10-25T21:53:37.9716047Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-10-25T21:53:37.9726604Z 
2019-10-25T21:53:37.9727430Z ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
2019-10-25T21:53:37.9728124Z 
2019-10-25T21:53:37.9729700Z If you do intend to update 'clippy-driver', please check the error messages above and
2019-10-25T21:53:37.9729819Z commit another update.
2019-10-25T21:53:37.9729852Z 
2019-10-25T21:53:37.9730206Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2019-10-25T21:53:37.9730476Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2019-10-25T21:53:37.9730528Z proper steps.
2019-10-25T21:53:37.9750456Z   local time: Fri Oct 25 21:53:37 UTC 2019
2019-10-25T21:53:38.0148313Z   network time: Fri, 25 Oct 2019 21:53:38 GMT
2019-10-25T21:53:38.0149550Z == end clock drift check ==
2019-10-25T21:53:40.5869916Z 
2019-10-25T21:53:40.5869916Z 
2019-10-25T21:53:40.5992841Z ##[error]Bash exited with code '3'.
2019-10-25T21:53:40.6050353Z ##[section]Starting: Checkout
2019-10-25T21:53:40.6052459Z ==============================================================================
2019-10-25T21:53:40.6052521Z Task         : Get sources
2019-10-25T21:53:40.6052581Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

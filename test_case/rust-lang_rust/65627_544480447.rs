plain
2019-10-21T09:18:18.1692169Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-21T09:18:18.1881683Z ##[command]git config gc.auto 0
2019-10-21T09:18:18.1985105Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-21T09:18:18.2077760Z ##[command]git config --get-all http.proxy
2019-10-21T09:18:18.2226925Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65627/merge:refs/remotes/pull/65627/merge
---
2019-10-21T11:07:35.6699493Z     Finished release [optimized] target(s) in 8m 50s
2019-10-21T11:09:27.9870038Z Error: there are broken links
2019-10-21T11:09:27.9871920Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html" returned 404 Not Found
2019-10-21T11:09:27.9872915Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html#method.buffer_lint" returned 404 Not Found
2019-10-21T11:09:27.9873612Z  Caused By: "***/tree/master/src/libsyntax/ext/tt" returned 404 Not Found
2019-10-21T11:09:27.9874644Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/fn.parse.html" returned 404 Not Found
2019-10-21T11:09:27.9875105Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_rules/" returned 404 Not Found
2019-10-21T11:09:27.9875558Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/" returned 404 Not Found
2019-10-21T11:09:27.9876019Z  Caused By: "https://github.com/rust-lang/chalk/blob/master/src/test/wf.rs" returned 404 Not Found
---
2019-10-21T11:55:04.4510781Z Verifying status of rustfmt...
2019-10-21T11:55:04.4523661Z Verifying status of clippy-driver...
2019-10-21T11:55:04.4537305Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-10-21T11:55:04.4549937Z 
2019-10-21T11:55:04.4550513Z ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
2019-10-21T11:55:04.4550557Z 
2019-10-21T11:55:04.4550872Z If you do intend to update 'clippy-driver', please check the error messages above and
2019-10-21T11:55:04.4550929Z commit another update.
2019-10-21T11:55:04.4550958Z 
2019-10-21T11:55:04.4551237Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2019-10-21T11:55:04.4552409Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2019-10-21T11:55:04.4552469Z proper steps.
2019-10-21T11:55:04.4571168Z   local time: Mon Oct 21 11:55:04 UTC 2019
2019-10-21T11:55:04.7546302Z   network time: Mon, 21 Oct 2019 11:55:04 GMT
2019-10-21T11:55:04.7550153Z == end clock drift check ==
2019-10-21T11:55:06.1847331Z 
2019-10-21T11:55:06.1847331Z 
2019-10-21T11:55:06.1958304Z ##[error]Bash exited with code '3'.
2019-10-21T11:55:06.1997589Z ##[section]Starting: Checkout
2019-10-21T11:55:06.1999773Z ==============================================================================
2019-10-21T11:55:06.1999878Z Task         : Get sources
2019-10-21T11:55:06.1999931Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

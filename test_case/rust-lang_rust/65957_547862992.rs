plain
2019-10-30T08:52:25.9461818Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-30T08:52:25.9667718Z ##[command]git config gc.auto 0
2019-10-30T08:52:25.9761588Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-30T08:52:25.9813491Z ##[command]git config --get-all http.proxy
2019-10-30T08:52:25.9965693Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65957/merge:refs/remotes/pull/65957/merge
---
2019-10-30T10:42:14.8515920Z     Finished release [optimized] target(s) in 11m 17s
2019-10-30T10:44:31.0389658Z Error: there are broken links
2019-10-30T10:44:31.0390675Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html" returned 404 Not Found
2019-10-30T10:44:31.0391087Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html#method.buffer_lint" returned 404 Not Found
2019-10-30T10:44:31.0391558Z  Caused By: "***/tree/master/src/libsyntax/ext/tt" returned 404 Not Found
2019-10-30T10:44:31.0401065Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/fn.parse.html" returned 404 Not Found
2019-10-30T10:44:31.0401869Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_rules/" returned 404 Not Found
2019-10-30T10:44:31.0402601Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/" returned 404 Not Found
2019-10-30T10:44:31.0405770Z  Caused By: "https://github.com/rust-lang/chalk/blob/master/src/test/wf.rs" returned 404 Not Found
---
2019-10-30T10:44:31.0426168Z  Caused By: "https://rust-lang.github.io/chalk/doc/chalk_parse/index.html" returned 404 Not Found
2019-10-30T10:44:31.0426459Z  Caused By: "https://rust-lang.github.io/chalk/doc/chalk/index.html" returned 404 Not Found
2019-10-30T10:44:31.0427085Z  Caused By: "https://rust-lang.github.io/chalk/doc/chalki/index.html" returned 404 Not Found
2019-10-30T10:44:31.0427408Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html" returned 404 Not Found
2019-10-30T10:44:31.0427791Z  Caused By: There was an error while fetching "http://www.cs.bgu.ac.il/%7Ehendlerd/papers/p280-hendler.pdf", http://www.cs.bgu.ac.il/%7Ehendlerd/papers/p280-hendler.pdf: timed out
2019-10-30T10:44:31.0438275Z 
2019-10-30T10:44:31.0439218Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustbook" "linkcheck" "/checkout/src/doc/rustc-guide"
2019-10-30T10:44:31.0439726Z expected success, got: exit code: 101
2019-10-30T10:44:31.0439873Z 
---
2019-10-30T10:49:36.9366944Z    Compiling toml v0.5.3
2019-10-30T10:49:48.4752341Z    Compiling url v2.1.0
2019-10-30T10:50:20.8382580Z    Compiling cargo_metadata v0.9.0
2019-10-30T10:50:46.0220764Z    Compiling clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
2019-10-30T10:57:11.3468482Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-10-30T10:57:11.3469590Z    |
2019-10-30T10:57:11.3469871Z 12 | #[plugin_registrar]
2019-10-30T10:57:11.3470259Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-10-30T10:57:11.3470506Z    |
---
2019-10-30T11:46:06.0403904Z Verifying status of edition-guide...
2019-10-30T11:46:06.0416813Z Verifying status of rls...
2019-10-30T11:46:06.0431695Z This PR updated 'src/tools/rls', verifying if status is 'test-pass'...
2019-10-30T11:46:06.0443597Z 
2019-10-30T11:46:06.0444432Z ⚠️ We detected that this PR updated 'rls', but its tests failed.
2019-10-30T11:46:06.0444694Z 
2019-10-30T11:46:06.0446186Z If you do intend to update 'rls', please check the error messages above and
2019-10-30T11:46:06.0446534Z commit another update.
2019-10-30T11:46:06.0446620Z 
2019-10-30T11:46:06.0446962Z If you do NOT intend to update 'rls', please ensure you did not accidentally
2019-10-30T11:46:06.0447231Z change the submodule at 'src/tools/rls'. You may ask your reviewer for the
2019-10-30T11:46:06.0447579Z proper steps.
2019-10-30T11:46:06.0468957Z   local time: Wed Oct 30 11:46:06 UTC 2019
2019-10-30T11:46:06.3398489Z   network time: Wed, 30 Oct 2019 11:46:06 GMT
2019-10-30T11:46:06.3398923Z == end clock drift check ==
2019-10-30T11:46:07.3464180Z 
2019-10-30T11:46:07.3464180Z 
2019-10-30T11:46:07.3582407Z ##[error]Bash exited with code '3'.
2019-10-30T11:46:07.3629803Z ##[section]Starting: Checkout
2019-10-30T11:46:07.3632125Z ==============================================================================
2019-10-30T11:46:07.3632184Z Task         : Get sources
2019-10-30T11:46:07.3632252Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

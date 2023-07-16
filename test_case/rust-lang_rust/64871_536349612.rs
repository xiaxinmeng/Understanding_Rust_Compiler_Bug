plain
2019-09-29T20:16:22.2962973Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-29T20:16:22.3142882Z ##[command]git config gc.auto 0
2019-09-29T20:16:22.3224894Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-29T20:16:22.3284506Z ##[command]git config --get-all http.proxy
2019-09-29T20:16:22.3423943Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64871/merge:refs/remotes/pull/64871/merge
---
2019-09-29T22:06:31.6499142Z    Compiling mdbook-linkcheck v0.3.0
2019-09-29T22:06:46.3960515Z    Compiling rustbook v0.1.0 (/checkout/src/tools/rustbook)
2019-09-29T22:06:50.9221732Z     Finished release [optimized] target(s) in 8m 41s
2019-09-29T22:08:30.8909946Z Error: there are broken links
2019-09-29T22:08:30.8912774Z  Caused By: "***/tree/master/src/libsyntax/ext/tt" returned 404 Not Found
2019-09-29T22:08:30.8921970Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/fn.parse.html" returned 404 Not Found
2019-09-29T22:08:30.8922658Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_rules/" returned 404 Not Found
2019-09-29T22:08:30.8922988Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/" returned 404 Not Found
2019-09-29T22:08:30.8923261Z  Caused By: "https://rust-lang.github.io/chalk/doc/chalk_engine/forest/struct.Forest.html" returned 404 Not Found
---
2019-09-29T22:52:53.7007064Z Verifying status of rustfmt...
2019-09-29T22:52:53.7022886Z Verifying status of clippy-driver...
2019-09-29T22:52:53.7039379Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-09-29T22:52:53.7051594Z 
2019-09-29T22:52:53.7052495Z ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
2019-09-29T22:52:53.7052573Z 
2019-09-29T22:52:53.7052915Z If you do intend to update 'clippy-driver', please check the error messages above and
2019-09-29T22:52:53.7052978Z commit another update.
2019-09-29T22:52:53.7053011Z 
2019-09-29T22:52:53.7053341Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2019-09-29T22:52:53.7053652Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2019-09-29T22:52:53.7053709Z proper steps.
2019-09-29T22:52:53.7100155Z   local time: Sun Sep 29 22:52:53 UTC 2019
2019-09-29T22:52:53.8863891Z   network time: Sun, 29 Sep 2019 22:52:53 GMT
2019-09-29T22:52:53.8870125Z == end clock drift check ==
2019-09-29T22:52:53.8870125Z == end clock drift check ==
2019-09-29T22:52:55.4233596Z ##[error]Bash exited with code '3'.
2019-09-29T22:52:55.4291883Z ##[section]Starting: Checkout
2019-09-29T22:52:55.4294304Z ==============================================================================
2019-09-29T22:52:55.4294370Z Task         : Get sources
2019-09-29T22:52:55.4294422Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

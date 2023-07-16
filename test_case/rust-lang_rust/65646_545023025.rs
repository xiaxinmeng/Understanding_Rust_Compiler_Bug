plain
2019-10-22T13:16:12.1904242Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-22T13:16:12.2083163Z ##[command]git config gc.auto 0
2019-10-22T13:16:12.2148733Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-22T13:16:12.2199013Z ##[command]git config --get-all http.proxy
2019-10-22T13:16:12.2330084Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65646/merge:refs/remotes/pull/65646/merge
---
2019-10-22T14:54:34.4707950Z     Finished release [optimized] target(s) in 7m 59s
2019-10-22T14:56:17.1469915Z Error: there are broken links
2019-10-22T14:56:17.1470839Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html" returned 404 Not Found
2019-10-22T14:56:17.1471273Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html#method.buffer_lint" returned 404 Not Found
2019-10-22T14:56:17.1471754Z  Caused By: "***/tree/master/src/libsyntax/ext/tt" returned 404 Not Found
2019-10-22T14:56:17.1472433Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/fn.parse.html" returned 404 Not Found
2019-10-22T14:56:17.1473013Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_rules/" returned 404 Not Found
2019-10-22T14:56:17.1473356Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/" returned 404 Not Found
2019-10-22T14:56:17.1473647Z  Caused By: "https://github.com/rust-lang/chalk/blob/master/src/test/wf.rs" returned 404 Not Found
---
2019-10-22T15:37:23.9381441Z Verifying status of rustfmt...
2019-10-22T15:37:23.9393053Z Verifying status of clippy-driver...
2019-10-22T15:37:23.9405550Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-10-22T15:37:23.9416551Z 
2019-10-22T15:37:23.9417456Z ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
2019-10-22T15:37:23.9417813Z 
2019-10-22T15:37:23.9418380Z If you do intend to update 'clippy-driver', please check the error messages above and
2019-10-22T15:37:23.9418637Z commit another update.
2019-10-22T15:37:23.9419544Z 
2019-10-22T15:37:23.9420048Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2019-10-22T15:37:23.9420684Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2019-10-22T15:37:23.9420750Z proper steps.
2019-10-22T15:37:23.9441350Z   local time: Tue Oct 22 15:37:23 UTC 2019
2019-10-22T15:37:24.0332879Z   network time: Tue, 22 Oct 2019 15:37:24 GMT
2019-10-22T15:37:24.0334974Z == end clock drift check ==
2019-10-22T15:37:26.6841675Z 
2019-10-22T15:37:26.6841675Z 
2019-10-22T15:37:26.6940268Z ##[error]Bash exited with code '3'.
2019-10-22T15:37:26.6994177Z ##[section]Starting: Checkout
2019-10-22T15:37:26.6995902Z ==============================================================================
2019-10-22T15:37:26.6996096Z Task         : Get sources
2019-10-22T15:37:26.6996144Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

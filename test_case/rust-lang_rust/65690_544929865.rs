plain
2019-10-22T09:31:29.3298403Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-22T09:31:29.3486002Z ##[command]git config gc.auto 0
2019-10-22T09:31:29.3575120Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-22T09:31:29.3617153Z ##[command]git config --get-all http.proxy
2019-10-22T09:31:29.3758680Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65690/merge:refs/remotes/pull/65690/merge
---
2019-10-22T11:18:41.2552372Z     Finished release [optimized] target(s) in 8m 43s
2019-10-22T11:20:59.3839306Z Error: there are broken links
2019-10-22T11:20:59.3841355Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html" returned 404 Not Found
2019-10-22T11:20:59.3842160Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html#method.buffer_lint" returned 404 Not Found
2019-10-22T11:20:59.3842954Z  Caused By: "***/tree/master/src/libsyntax/ext/tt" returned 404 Not Found
2019-10-22T11:20:59.3848368Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/fn.parse.html" returned 404 Not Found
2019-10-22T11:20:59.3853481Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_rules/" returned 404 Not Found
2019-10-22T11:20:59.3855196Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/" returned 404 Not Found
2019-10-22T11:20:59.3855903Z  Caused By: "https://github.com/rust-lang/chalk/blob/master/src/test/wf.rs" returned 404 Not Found
---
2019-10-22T11:20:59.3891182Z  Caused By: "https://rust-lang.github.io/chalk/doc/chalk_parse/index.html" returned 404 Not Found
2019-10-22T11:20:59.3891810Z  Caused By: "https://rust-lang.github.io/chalk/doc/chalk/index.html" returned 404 Not Found
2019-10-22T11:20:59.3892360Z  Caused By: "https://rust-lang.github.io/chalk/doc/chalki/index.html" returned 404 Not Found
2019-10-22T11:20:59.3892960Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html" returned 404 Not Found
2019-10-22T11:20:59.3893231Z  Caused By: There was an error while fetching "http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.170.1097&rep=rep1&type=pdf", http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.170.1097&rep=rep1&type=pdf: timed out
2019-10-22T11:20:59.3893585Z  Caused By: "http://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.704.1768" returned 500 Internal Server Error
2019-10-22T11:20:59.3900328Z 
2019-10-22T11:20:59.3901219Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustbook" "linkcheck" "/checkout/src/doc/rustc-guide"
2019-10-22T11:20:59.3901511Z expected success, got: exit code: 101
2019-10-22T11:20:59.3901664Z 
---
2019-10-22T11:27:45.9147040Z    Compiling clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
2019-10-22T11:27:55.7767048Z error[E0507]: cannot move out of `self.s` which is behind a shared reference
2019-10-22T11:27:55.7767507Z    --> src/tools/clippy/clippy_lints/src/utils/hir_utils.rs:365:9
2019-10-22T11:27:55.7767834Z     |
2019-10-22T11:27:55.7768147Z 365 |         self.s.finish()
2019-10-22T11:27:55.7768984Z     |         ^^^^^^ move occurs because `self.s` has type `rustc_data_structures::stable_hasher::StableHasher`, which does not implement the `Copy` trait
2019-10-22T11:27:59.0366205Z error: aborting due to previous error
2019-10-22T11:27:59.0366316Z 
2019-10-22T11:27:59.0378204Z For more information about this error, try `rustc --explain E0507`.
2019-10-22T11:27:59.1641025Z error: could not compile `clippy_lints`.
2019-10-22T11:27:59.1641025Z error: could not compile `clippy_lints`.
2019-10-22T11:27:59.1656625Z warning: build failed, waiting for other jobs to finish...
2019-10-22T11:28:02.4851045Z error[E0507]: cannot move out of `self.s` which is behind a shared reference
2019-10-22T11:28:02.4852953Z    --> src/tools/clippy/clippy_lints/src/utils/hir_utils.rs:365:9
2019-10-22T11:28:02.4853642Z     |
2019-10-22T11:28:02.4854342Z 365 |         self.s.finish()
2019-10-22T11:28:02.4855208Z     |         ^^^^^^ move occurs because `self.s` has type `rustc_data_structures::stable_hasher::StableHasher`, which does not implement the `Copy` trait
2019-10-22T11:28:05.6683938Z error: aborting due to previous error
2019-10-22T11:28:05.6684967Z 
2019-10-22T11:28:05.6685729Z For more information about this error, try `rustc --explain E0507`.
2019-10-22T11:28:05.7975871Z error: could not compile `clippy_lints`.
---
2019-10-22T12:05:35.2648608Z Verifying status of rustfmt...
2019-10-22T12:05:35.2662969Z Verifying status of clippy-driver...
2019-10-22T12:05:35.2676979Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-10-22T12:05:35.2700756Z 
2019-10-22T12:05:35.2702206Z ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
2019-10-22T12:05:35.2702655Z 
2019-10-22T12:05:35.2704297Z If you do intend to update 'clippy-driver', please check the error messages above and
2019-10-22T12:05:35.2704383Z commit another update.
2019-10-22T12:05:35.2704416Z 
2019-10-22T12:05:35.2704922Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2019-10-22T12:05:35.2705165Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2019-10-22T12:05:35.2705222Z proper steps.
2019-10-22T12:05:35.2730097Z   local time: Tue Oct 22 12:05:35 UTC 2019
2019-10-22T12:05:35.4343633Z   network time: Tue, 22 Oct 2019 12:05:35 GMT
2019-10-22T12:05:35.4343822Z == end clock drift check ==
2019-10-22T12:05:38.2894264Z 
2019-10-22T12:05:38.2894264Z 
2019-10-22T12:05:38.3003362Z ##[error]Bash exited with code '3'.
2019-10-22T12:05:38.3051874Z ##[section]Starting: Checkout
2019-10-22T12:05:38.3053864Z ==============================================================================
2019-10-22T12:05:38.3053917Z Task         : Get sources
2019-10-22T12:05:38.3053960Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

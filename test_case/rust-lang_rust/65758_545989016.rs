plain
2019-10-24T13:40:03.7711027Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-24T13:40:03.7900451Z ##[command]git config gc.auto 0
2019-10-24T13:40:03.7977212Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-24T13:40:03.8033899Z ##[command]git config --get-all http.proxy
2019-10-24T13:40:03.8172239Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65758/merge:refs/remotes/pull/65758/merge
---
2019-10-24T15:20:40.0113038Z     Finished release [optimized] target(s) in 8m 09s
2019-10-24T15:22:41.2730983Z Error: there are broken links
2019-10-24T15:22:41.2732926Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html" returned 404 Not Found
2019-10-24T15:22:41.2733589Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html#method.buffer_lint" returned 404 Not Found
2019-10-24T15:22:41.2734441Z  Caused By: "***/tree/master/src/libsyntax/ext/tt" returned 404 Not Found
2019-10-24T15:22:41.2736033Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/fn.parse.html" returned 404 Not Found
2019-10-24T15:22:41.2736342Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_rules/" returned 404 Not Found
2019-10-24T15:22:41.2736662Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/" returned 404 Not Found
2019-10-24T15:22:41.2737348Z  Caused By: "https://github.com/rust-lang/chalk/blob/master/src/test/wf.rs" returned 404 Not Found
---
2019-10-24T16:04:00.0540368Z    Compiling openssl v0.10.16
2019-10-24T16:04:00.0578483Z    Compiling curl-sys v0.4.22
2019-10-24T16:04:08.6708922Z    Compiling rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
2019-10-24T16:05:49.9517141Z     Finished release [optimized] target(s) in 3m 05s
2019-10-24T16:05:49.9785583Z error: Found argument '--manifest-path' which wasn't expected, or isn't valid in this context
2019-10-24T16:05:49.9786673Z USAGE:
2019-10-24T16:05:49.9786673Z USAGE:
2019-10-24T16:05:49.9787298Z     cargo install --color <WHEN> --jobs <N> --locked -Z <FLAG>...
2019-10-24T16:05:49.9788556Z For more information try --help
2019-10-24T16:05:49.9792041Z 
2019-10-24T16:05:49.9792368Z 
2019-10-24T16:05:49.9793232Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "install" "-Zconfig-profile" "-Zbinary-dep-depinfo" "-j" "2" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/miri/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "xargo"
---
2019-10-24T16:05:49.9961291Z Verifying status of clippy-driver...
2019-10-24T16:05:49.9971999Z Verifying status of miri...
2019-10-24T16:05:49.9984222Z This PR updated 'src/tools/miri', verifying if status is 'test-pass'...
2019-10-24T16:05:49.9996141Z 
2019-10-24T16:05:49.9996713Z ⚠️ We detected that this PR updated 'miri', but its tests failed.
2019-10-24T16:05:49.9996755Z 
2019-10-24T16:05:49.9997036Z If you do intend to update 'miri', please check the error messages above and
2019-10-24T16:05:49.9997116Z commit another update.
2019-10-24T16:05:49.9997145Z 
2019-10-24T16:05:49.9997405Z If you do NOT intend to update 'miri', please ensure you did not accidentally
2019-10-24T16:05:49.9998107Z change the submodule at 'src/tools/miri'. You may ask your reviewer for the
2019-10-24T16:05:49.9998169Z proper steps.
2019-10-24T16:05:50.0022373Z   local time: Thu Oct 24 16:05:50 UTC 2019
2019-10-24T16:05:50.2913533Z   network time: Thu, 24 Oct 2019 16:05:50 GMT
2019-10-24T16:05:50.2914514Z == end clock drift check ==
2019-10-24T16:05:51.2550580Z 
2019-10-24T16:05:51.2550580Z 
2019-10-24T16:05:51.2660096Z ##[error]Bash exited with code '3'.
2019-10-24T16:05:51.2698712Z ##[section]Starting: Checkout
2019-10-24T16:05:51.2700853Z ==============================================================================
2019-10-24T16:05:51.2700911Z Task         : Get sources
2019-10-24T16:05:51.2700973Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

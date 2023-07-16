plain
2019-11-06T05:19:30.8357446Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-06T05:19:30.8592715Z ##[command]git config gc.auto 0
2019-11-06T05:19:30.8657742Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-06T05:19:30.8704212Z ##[command]git config --get-all http.proxy
2019-11-06T05:19:30.8890042Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66142/merge:refs/remotes/pull/66142/merge
---
2019-11-06T06:57:09.8790151Z     Finished release [optimized] target(s) in 10m 33s
2019-11-06T06:59:02.8670425Z Error: there are broken links
2019-11-06T06:59:02.8671332Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html" returned 404 Not Found
2019-11-06T06:59:02.8672064Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html#method.buffer_lint" returned 404 Not Found
2019-11-06T06:59:02.8672681Z  Caused By: "***/tree/master/src/libsyntax/ext/tt" returned 404 Not Found
2019-11-06T06:59:02.8673395Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/fn.parse.html" returned 404 Not Found
2019-11-06T06:59:02.8673720Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_rules/" returned 404 Not Found
2019-11-06T06:59:02.8674058Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/" returned 404 Not Found
2019-11-06T06:59:02.8674372Z  Caused By: "https://github.com/rust-lang/chalk/blob/master/src/test/wf.rs" returned 404 Not Found
---
2019-11-06T07:13:13.5237603Z 
2019-11-06T07:13:13.5237843Z ------------------------------------------
2019-11-06T07:13:13.5237890Z stderr:
2019-11-06T07:13:13.5238104Z ------------------------------------------
2019-11-06T07:13:13.5239402Z {"message":"can't find crate for `clippy_lints`","code":{"code":"E0463","explanation":"\nA plugin/crate was declared but cannot be found. Erroneous code example:\n\n
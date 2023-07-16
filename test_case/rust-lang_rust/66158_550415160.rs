plain
2019-11-06T14:49:18.8645508Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-06T14:49:18.8841203Z ##[command]git config gc.auto 0
2019-11-06T14:49:18.8899850Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-06T14:49:18.8961463Z ##[command]git config --get-all http.proxy
2019-11-06T14:49:19.4240687Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66158/merge:refs/remotes/pull/66158/merge
---
2019-11-06T16:31:30.5688990Z     Finished release [optimized] target(s) in 11m 12s
2019-11-06T16:33:28.3052992Z Error: there are broken links
2019-11-06T16:33:28.3056713Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html" returned 404 Not Found
2019-11-06T16:33:28.3057448Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html#method.buffer_lint" returned 404 Not Found
2019-11-06T16:33:28.3058002Z  Caused By: "***/tree/master/src/libsyntax/ext/tt" returned 404 Not Found
2019-11-06T16:33:28.3058673Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/fn.parse.html" returned 404 Not Found
2019-11-06T16:33:28.3058986Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_rules/" returned 404 Not Found
2019-11-06T16:33:28.3059292Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/" returned 404 Not Found
2019-11-06T16:33:28.3059776Z  Caused By: "https://github.com/rust-lang/chalk/blob/master/src/test/wf.rs" returned 404 Not Found
---
2019-11-06T16:48:44.8281918Z 
2019-11-06T16:48:44.8282173Z ------------------------------------------
2019-11-06T16:48:44.8282223Z stderr:
2019-11-06T16:48:44.8282470Z ------------------------------------------
2019-11-06T16:48:44.8283819Z {"message":"can't find crate for `clippy_lints`","code":{"code":"E0463","explanation":"\nA plugin/crate was declared but cannot be found. Erroneous code example:\n\n
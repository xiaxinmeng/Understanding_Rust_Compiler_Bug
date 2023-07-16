plain
2019-11-06T15:29:42.2692965Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-06T15:29:42.2886251Z ##[command]git config gc.auto 0
2019-11-06T15:29:42.2962609Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-06T15:29:42.3039187Z ##[command]git config --get-all http.proxy
2019-11-06T15:29:42.3203454Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66160/merge:refs/remotes/pull/66160/merge
---
2019-11-06T17:08:52.2086374Z     Finished release [optimized] target(s) in 10m 40s
2019-11-06T17:10:43.7467263Z Error: there are broken links
2019-11-06T17:10:43.7469074Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html" returned 404 Not Found
2019-11-06T17:10:43.7469955Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html#method.buffer_lint" returned 404 Not Found
2019-11-06T17:10:43.7471075Z  Caused By: "***/tree/master/src/libsyntax/ext/tt" returned 404 Not Found
2019-11-06T17:10:43.7471888Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/fn.parse.html" returned 404 Not Found
2019-11-06T17:10:43.7472250Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_rules/" returned 404 Not Found
2019-11-06T17:10:43.7472609Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/" returned 404 Not Found
2019-11-06T17:10:43.7472926Z  Caused By: "https://github.com/rust-lang/chalk/blob/master/src/test/wf.rs" returned 404 Not Found
---
2019-11-06T17:25:15.9614167Z 
2019-11-06T17:25:15.9614406Z ------------------------------------------
2019-11-06T17:25:15.9614473Z stderr:
2019-11-06T17:25:15.9614707Z ------------------------------------------
2019-11-06T17:25:15.9616101Z {"message":"can't find crate for `clippy_lints`","code":{"code":"E0463","explanation":"\nA plugin/crate was declared but cannot be found. Erroneous code example:\n\n
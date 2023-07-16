plain
2019-11-06T19:43:59.2566326Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-06T19:43:59.2730096Z ##[command]git config gc.auto 0
2019-11-06T19:43:59.2796880Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-06T19:43:59.2860930Z ##[command]git config --get-all http.proxy
2019-11-06T19:43:59.3012213Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66158/merge:refs/remotes/pull/66158/merge
---
2019-11-06T21:19:43.8247102Z     Finished release [optimized] target(s) in 10m 26s
2019-11-06T21:21:50.5626576Z Error: there are broken links
2019-11-06T21:21:50.5629104Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html" returned 404 Not Found
2019-11-06T21:21:50.5629986Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html#method.buffer_lint" returned 404 Not Found
2019-11-06T21:21:50.5630554Z  Caused By: "***/tree/master/src/libsyntax/ext/tt" returned 404 Not Found
2019-11-06T21:21:50.5631254Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/fn.parse.html" returned 404 Not Found
2019-11-06T21:21:50.5631539Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_rules/" returned 404 Not Found
2019-11-06T21:21:50.5631797Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/" returned 404 Not Found
2019-11-06T21:21:50.5632056Z  Caused By: "https://github.com/rust-lang/chalk/blob/master/src/test/wf.rs" returned 404 Not Found
---
2019-11-06T21:36:06.7077047Z 
2019-11-06T21:36:06.7077283Z ------------------------------------------
2019-11-06T21:36:06.7077328Z stderr:
2019-11-06T21:36:06.7077546Z ------------------------------------------
2019-11-06T21:36:06.7079694Z {"message":"can't find crate for `clippy_lints`","code":{"code":"E0463","explanation":"\nA plugin/crate was declared but cannot be found. Erroneous code example:\n\n
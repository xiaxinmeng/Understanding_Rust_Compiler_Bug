plain
2019-11-03T07:10:04.2092474Z  Caused By: "https://rust-lang.github.io/chalk/doc/chalk_parse/index.html" returned 404 Not Found
2019-11-03T07:10:04.2092993Z  Caused By: "https://rust-lang.github.io/chalk/doc/chalk/index.html" returned 404 Not Found
2019-11-03T07:10:04.2093349Z  Caused By: "https://rust-lang.github.io/chalk/doc/chalki/index.html" returned 404 Not Found
2019-11-03T07:10:04.2093753Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html" returned 404 Not Found
2019-11-03T07:10:04.2093937Z  Caused By: There was an error while fetching "http://www.cs.rice.edu/%7Eyguo/pubs/PID824943.pdf", http://www.cs.rice.edu/%7Eyguo/pubs/PID824943.pdf: timed out
2019-11-03T07:10:04.2094114Z 
2019-11-03T07:10:04.2097005Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustbook" "linkcheck" "/checkout/src/doc/rustc-guide"
2019-11-03T07:10:04.2098340Z expected success, got: exit code: 101
2019-11-03T07:10:04.2098741Z 
---
2019-11-03T07:16:38.3107307Z 
2019-11-03T07:16:43.1579761Z error[E0277]: the trait bound `syntax_pos::symbol::SymbolStr: std::convert::AsRef<std::ffi::OsStr>` is not satisfied
2019-11-03T07:16:43.1581114Z     --> src/tools/clippy/clippy_lints/src/path_buf_push_overwrite.rs:53:44
2019-11-03T07:16:43.1581796Z      |
2019-11-03T07:16:43.1582827Z 53   |             if let pushed_path = Path::new(&path_lit.as_str());
2019-11-03T07:16:43.1583837Z      |                                            ^^^^^^^^^^^^^^^^^^ the trait `std::convert::AsRef<std::ffi::OsStr>` is not implemented for `syntax_pos::symbol::SymbolStr`
2019-11-03T07:16:43.1585151Z     ::: /checkout/src/libstd/path.rs:1814:19
2019-11-03T07:16:43.1585719Z      |
2019-11-03T07:16:43.1585719Z      |
2019-11-03T07:16:43.1586396Z 1814 |     pub fn new<S: AsRef<OsStr> + ?Sized>(s: &S) -> &Path {
2019-11-03T07:16:43.1587769Z 
2019-11-03T07:16:44.0585621Z error: aborting due to 5 previous errors
2019-11-03T07:16:44.0585803Z 
2019-11-03T07:16:44.0592176Z Some errors have detailed explanations: E0277, E0432.
---
2019-11-03T07:55:01.6353394Z   local time: Sun Nov  3 07:55:01 UTC 2019
2019-11-03T07:55:01.9051349Z   network time: Sun, 03 Nov 2019 07:55:01 GMT
2019-11-03T07:55:01.9055246Z == end clock drift check ==
2019-11-03T07:55:02.9035817Z 
2019-11-03T07:55:02.9162598Z ##[error]Bash exited with code '1'.
2019-11-03T07:55:02.9203954Z ##[section]Starting: Checkout
2019-11-03T07:55:02.9206104Z ==============================================================================
2019-11-03T07:55:02.9206207Z Task         : Get sources
2019-11-03T07:55:02.9206319Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

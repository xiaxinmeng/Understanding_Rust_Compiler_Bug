\n"},"level":"error","spans":[{"file_name":"tests/run-pass/panic/catch_panic.rs","byte_start":2103,"byte_end":2104,"line_start":70,"line_end":70,"column_start":41,"column_end":42,"is_primary":true,"text":[{"text":"fn test(do_panic: impl FnOnce(usize) -> !) {","highlight_start":41,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"for more information, see ***/issues/35121","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add `#![feature(never_type)]` to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: The `!` type is experimental\n  --> tests/run-pass/panic/catch_panic.rs:70:41\n   |\n70 | fn test(do_panic: impl FnOnce(usize) -> !) {\n   |                                         ^\n   |\n   = note: for more information, see ***/issues/35121\n   = help: add `#![feature(never_type)]` to the crate attributes to enable\n\n"}
2019-12-14T16:43:41.4545122Z {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
2019-12-14T16:43:41.4545324Z 
2019-12-14T16:43:41.4545681Z ------------------------------------------
2019-12-14T16:43:41.4545838Z 
---
2019-12-14T16:43:51.1769748Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-12-14T16:43:51.1769802Z Verifying status of miri...
2019-12-14T16:43:51.1770046Z Verifying status of embedded-book...
2019-12-14T16:43:51.1770405Z Verifying status of rustc-guide...
2019-12-14T16:43:51.1770658Z error: Tool `reference` should be test-pass but is test-fail during beta week.
2019-12-14T16:43:51.1778633Z Build completed unsuccessfully in 0:00:01
2019-12-14T16:43:51.1828527Z == clock drift check ==
2019-12-14T16:43:51.1844084Z   local time: Sat Dec 14 16:43:51 UTC 2019
2019-12-14T16:43:51.4610827Z   network time: Sat, 14 Dec 2019 16:43:51 GMT
2019-12-14T16:43:51.4610827Z   network time: Sat, 14 Dec 2019 16:43:51 GMT
2019-12-14T16:43:51.4615601Z == end clock drift check ==
2019-12-14T16:43:52.1659403Z 
2019-12-14T16:43:52.1760906Z ##[error]Bash exited with code '1'.
2019-12-14T16:43:52.1812106Z ##[section]Starting: Checkout
2019-12-14T16:43:52.1813731Z ==============================================================================
2019-12-14T16:43:52.1813796Z Task         : Get sources
2019-12-14T16:43:52.1813839Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

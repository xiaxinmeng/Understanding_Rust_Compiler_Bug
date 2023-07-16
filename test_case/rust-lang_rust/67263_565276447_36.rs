\n"},"level":"error","spans":[{"file_name":"tests/run-pass/panic/catch_panic.rs","byte_start":2103,"byte_end":2104,"line_start":70,"line_end":70,"column_start":41,"column_end":42,"is_primary":true,"text":[{"text":"fn test(do_panic: impl FnOnce(usize) -> !) {","highlight_start":41,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"for more information, see https://github.com/rust-lang/rust/issues/35121","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add `#![feature(never_type)]` to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: The `!` type is experimental\n  --> tests/run-pass/panic/catch_panic.rs:70:41\n   |\n70 | fn test(do_panic: impl FnOnce(usize) -> !) {\n   |                                         ^\n   |\n   = note: for more information, see https://github.com/rust-lang/rust/issues/35121\n   = help: add `#![feature(never_type)]` to the crate attributes to enable\n\n"}
2019-12-13T02:40:06.5612521Z {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
2019-12-13T02:40:06.5612674Z 
2019-12-13T02:40:06.5612952Z ------------------------------------------
2019-12-13T02:40:06.5613009Z 
---
2019-12-13T02:40:16.9906044Z Verifying status of clippy-driver...
2019-12-13T02:40:16.9906319Z Verifying status of miri...
2019-12-13T02:40:16.9910030Z Verifying status of embedded-book...
2019-12-13T02:40:16.9910786Z Verifying status of rustc-guide...
2019-12-13T02:40:16.9912185Z error: Tool `reference` should be test-pass but is test-fail during beta week.
2019-12-13T02:40:16.9912730Z error: Tool `clippy-driver` should be test-pass but is test-fail during beta week.
2019-12-13T02:40:16.9918802Z Build completed unsuccessfully in 0:00:01
2019-12-13T02:40:16.9971897Z == clock drift check ==
2019-12-13T02:40:16.9988217Z   local time: Fri Dec 13 02:40:16 UTC 2019
2019-12-13T02:40:17.5445126Z   network time: Fri, 13 Dec 2019 02:40:17 GMT
2019-12-13T02:40:17.5445126Z   network time: Fri, 13 Dec 2019 02:40:17 GMT
2019-12-13T02:40:17.5448477Z == end clock drift check ==
2019-12-13T02:40:18.2383075Z 
2019-12-13T02:40:18.2452031Z ##[error]Bash exited with code '1'.
2019-12-13T02:40:18.2505071Z ##[section]Starting: Checkout
2019-12-13T02:40:18.2507521Z ==============================================================================
2019-12-13T02:40:18.2507627Z Task         : Get sources
2019-12-13T02:40:18.2507709Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

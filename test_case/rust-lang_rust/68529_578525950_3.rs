\n"},"level":"error","spans":[{"file_name":"tests/run-pass/heap_allocator.rs","byte_start":76,"byte_end":81,"line_start":4,"line_end":4,"column_start":26,"column_end":31,"is_primary":true,"text":[{"text":"use std::alloc::{Global, Alloc, Layout, System};","highlight_start":26,"highlight_end":31}],"label":"no `Alloc` in `alloc`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"a similar name exists in the module","code":null,"level":"help","spans":[{"file_name":"tests/run-pass/heap_allocator.rs","byte_start":76,"byte_end":81,"line_start":4,"line_end":4,"column_start":26,"column_end":31,"is_primary":true,"text":[{"text":"use std::alloc::{Global, Alloc, Layout, System};","highlight_start":26,"highlight_end":31}],"label":null,"suggested_replacement":"alloc","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0432]: unresolved import `std::alloc::Alloc`\n --> tests/run-pass/heap_allocator.rs:4:26\n  |\n4 | use std::alloc::{Global, Alloc, Layout, System};\n  |                          ^^^^^\n  |                          |\n  |                          no `Alloc` in `alloc`\n  |                          help: a similar name exists in the module: `alloc`\n\n"}
2020-01-26T17:49:14.7587553Z {"message":"For more information about this error, try `rustc --explain E0432`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0432`.\n"}
2020-01-26T17:49:14.7587688Z 
2020-01-26T17:49:14.7587940Z ------------------------------------------
2020-01-26T17:49:14.7588008Z 
---
2020-01-26T17:49:31.0023578Z Verifying status of clippy-driver...
2020-01-26T17:49:31.0023680Z Verifying status of miri...
2020-01-26T17:49:31.0023950Z Verifying status of embedded-book...
2020-01-26T17:49:31.0024213Z Verifying status of rustc-guide...
2020-01-26T17:49:31.0024522Z error: Tool `nomicon` should be test-pass but is test-fail during beta week.
2020-01-26T17:49:31.0028937Z Build completed unsuccessfully in 0:00:01
2020-01-26T17:49:31.0088904Z == clock drift check ==
2020-01-26T17:49:31.0104437Z   local time: Sun Jan 26 17:49:31 UTC 2020
2020-01-26T17:49:31.2855462Z   network time: Sun, 26 Jan 2020 17:49:31 GMT
2020-01-26T17:49:31.2855462Z   network time: Sun, 26 Jan 2020 17:49:31 GMT
2020-01-26T17:49:31.2858594Z == end clock drift check ==
2020-01-26T17:49:31.7662108Z 
2020-01-26T17:49:31.7749333Z ##[error]Bash exited with code '1'.
2020-01-26T17:49:31.7788257Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-01-26T17:49:31.7790479Z ==============================================================================
2020-01-26T17:49:31.7790575Z Task         : Get sources
2020-01-26T17:49:31.7790686Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

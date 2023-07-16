plain
2019-10-30T10:24:32.9334647Z  Caused By: "https://rust-lang.github.io/chalk/doc/chalk_parse/index.html" returned 404 Not Found
2019-10-30T10:24:32.9335129Z  Caused By: "https://rust-lang.github.io/chalk/doc/chalk/index.html" returned 404 Not Found
2019-10-30T10:24:32.9335543Z  Caused By: "https://rust-lang.github.io/chalk/doc/chalki/index.html" returned 404 Not Found
2019-10-30T10:24:32.9336011Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/parse/struct.ParseSess.html" returned 404 Not Found
2019-10-30T10:24:32.9336577Z  Caused By: There was an error while fetching "http://www.cs.bgu.ac.il/%7Ehendlerd/papers/p280-hendler.pdf", http://www.cs.bgu.ac.il/%7Ehendlerd/papers/p280-hendler.pdf: timed out
2019-10-30T10:24:32.9395205Z 
2019-10-30T10:24:32.9396669Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustbook" "linkcheck" "/checkout/src/doc/rustc-guide"
2019-10-30T10:24:32.9397116Z expected success, got: exit code: 101
2019-10-30T10:24:32.9397299Z 
---
2019-10-30T10:30:02.0825484Z [RUSTC-TIMING] cargo_metadata test:false 22.465
2019-10-30T10:30:05.8500472Z error[E0609]: no field `path` on type `&&syntax::ast::Attribute`
2019-10-30T10:30:05.8501001Z   --> src/tools/clippy/clippy_lints/src/utils/attrs.rs:60:35
2019-10-30T10:30:05.8501321Z    |
2019-10-30T10:30:05.8501728Z 60 |         let attr_segments = &attr.path.segments;
2019-10-30T10:30:05.8502545Z    |
2019-10-30T10:30:05.8502946Z    = note: available fields are: `kind`, `id`, `style`, `span`
2019-10-30T10:30:05.8503885Z 
2019-10-30T10:30:06.1397651Z error[E0609]: no field `path` on type `&&syntax::ast::Attribute`
2019-10-30T10:30:06.1397651Z error[E0609]: no field `path` on type `&&syntax::ast::Attribute`
2019-10-30T10:30:06.1398311Z   --> src/tools/clippy/clippy_lints/src/utils/attrs.rs:60:35
2019-10-30T10:30:06.1398659Z    |
2019-10-30T10:30:06.1399067Z 60 |         let attr_segments = &attr.path.segments;
2019-10-30T10:30:06.1400195Z    |
2019-10-30T10:30:06.1400595Z    = note: available fields are: `kind`, `id`, `style`, `span`
2019-10-30T10:30:06.1401487Z 
2019-10-30T10:30:06.1401487Z 
2019-10-30T10:30:07.3756441Z error[E0609]: no field `is_sugared_doc` on type `&syntax::ast::Attribute`
2019-10-30T10:30:07.3757633Z     |
2019-10-30T10:30:07.3757633Z     |
2019-10-30T10:30:07.3757927Z 420 |         if attr.is_sugared_doc {
2019-10-30T10:30:07.3758572Z     |
2019-10-30T10:30:07.3758879Z     = note: available fields are: `kind`, `id`, `style`, `span`
2019-10-30T10:30:07.3759345Z 
2019-10-30T10:30:07.3781212Z error[E0609]: no field `tokens` on type `&syntax::ast::Attribute`
2019-10-30T10:30:07.3781212Z error[E0609]: no field `tokens` on type `&syntax::ast::Attribute`
2019-10-30T10:30:07.3782169Z    --> src/tools/clippy/clippy_lints/src/attrs.rs:424:21
2019-10-30T10:30:07.3782486Z     |
2019-10-30T10:30:07.3783173Z 424 |             if attr.tokens.is_empty() || !is_present_in_source(cx, attr.span) {
2019-10-30T10:30:07.3783813Z     |
2019-10-30T10:30:07.3784132Z     = note: available fields are: `kind`, `id`, `style`, `span`
2019-10-30T10:30:07.3784569Z 
2019-10-30T10:30:07.3784569Z 
2019-10-30T10:30:07.6519326Z error[E0609]: no field `is_sugared_doc` on type `&syntax::ast::Attribute`
2019-10-30T10:30:07.6520613Z     |
2019-10-30T10:30:07.6520613Z     |
2019-10-30T10:30:07.6521000Z 420 |         if attr.is_sugared_doc {
2019-10-30T10:30:07.6521819Z     |
2019-10-30T10:30:07.6522215Z     = note: available fields are: `kind`, `id`, `style`, `span`
2019-10-30T10:30:07.6523159Z 
2019-10-30T10:30:07.6523159Z 
2019-10-30T10:30:07.6533759Z error[E0609]: no field `is_sugared_doc` on type `&syntax::ast::Attribute`
2019-10-30T10:30:07.6534603Z     |
2019-10-30T10:30:07.6534603Z     |
2019-10-30T10:30:07.6535001Z 250 |         if attr.is_sugared_doc {
2019-10-30T10:30:07.6537297Z     |
2019-10-30T10:30:07.6537997Z     = note: available fields are: `kind`, `id`, `style`, `span`
2019-10-30T10:30:07.6538578Z 
2019-10-30T10:30:07.6581899Z error[E0609]: no field `tokens` on type `&syntax::ast::Attribute`
2019-10-30T10:30:07.6581899Z error[E0609]: no field `tokens` on type `&syntax::ast::Attribute`
2019-10-30T10:30:07.6582510Z    --> src/tools/clippy/clippy_lints/src/attrs.rs:424:21
2019-10-30T10:30:07.6582835Z     |
2019-10-30T10:30:07.6583286Z 424 |             if attr.tokens.is_empty() || !is_present_in_source(cx, attr.span) {
2019-10-30T10:30:07.6584074Z     |
2019-10-30T10:30:07.6584491Z     = note: available fields are: `kind`, `id`, `style`, `span`
2019-10-30T10:30:07.6585083Z 
2019-10-30T10:30:07.6585083Z 
2019-10-30T10:30:07.9623380Z error[E0609]: no field `is_sugared_doc` on type `&syntax::ast::Attribute`
2019-10-30T10:30:07.9624197Z     |
2019-10-30T10:30:07.9624197Z     |
2019-10-30T10:30:07.9624589Z 250 |         if attr.is_sugared_doc {
2019-10-30T10:30:07.9625363Z     |
2019-10-30T10:30:07.9625767Z     = note: available fields are: `kind`, `id`, `style`, `span`
2019-10-30T10:30:07.9625857Z 
2019-10-30T10:30:08.5877532Z error[E0609]: no field `path` on type `&syntax::ast::Attribute`
2019-10-30T10:30:08.5877532Z error[E0609]: no field `path` on type `&syntax::ast::Attribute`
2019-10-30T10:30:08.5877957Z   --> src/tools/clippy/clippy_lints/src/main_recursion.rs:37:67
2019-10-30T10:30:08.5878253Z    |
2019-10-30T10:30:08.5878618Z 37 |         self.has_no_std_attr = krate.attrs.iter().any(|attr| attr.path == sym::no_std);
2019-10-30T10:30:08.5879302Z    |
2019-10-30T10:30:08.5879621Z    = note: available fields are: `kind`, `id`, `style`, `span`
2019-10-30T10:30:08.5879687Z 
2019-10-30T10:30:08.8483437Z error[E0609]: no field `path` on type `&syntax::ast::Attribute`
2019-10-30T10:30:08.8483437Z error[E0609]: no field `path` on type `&syntax::ast::Attribute`
2019-10-30T10:30:08.8483974Z   --> src/tools/clippy/clippy_lints/src/main_recursion.rs:37:67
2019-10-30T10:30:08.8484332Z    |
2019-10-30T10:30:08.8484776Z 37 |         self.has_no_std_attr = krate.attrs.iter().any(|attr| attr.path == sym::no_std);
2019-10-30T10:30:08.8485644Z    |
2019-10-30T10:30:08.8486033Z    = note: available fields are: `kind`, `id`, `style`, `span`
2019-10-30T10:30:08.8486137Z 
2019-10-30T10:30:10.3342870Z error: aborting due to 5 previous errors
---
2019-10-30T11:02:28.8374815Z   local time: Wed Oct 30 11:02:28 UTC 2019
2019-10-30T11:02:29.3718783Z   network time: Wed, 30 Oct 2019 11:02:29 GMT
2019-10-30T11:02:29.3723064Z == end clock drift check ==
2019-10-30T11:02:32.2423744Z 
2019-10-30T11:02:32.2560218Z ##[error]Bash exited with code '1'.
2019-10-30T11:02:32.2603208Z ##[section]Starting: Upload CPU usage statistics
2019-10-30T11:02:32.2615693Z ==============================================================================
2019-10-30T11:02:32.2615814Z Task         : Bash
2019-10-30T11:02:32.2615919Z Description  : Run a Bash script on macOS, Linux, or Windows

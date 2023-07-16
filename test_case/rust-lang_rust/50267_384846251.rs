plain
[01:12:21] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:12:21]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[01:12:34] error[E0658]: use of unstable library feature 'inner_deref': newly added (see issue #50264)
[01:12:34]    --> libcore/../libcore/tests/option.rs:305:27
[01:12:34]     |
[01:12:34] 305 |     assert_eq!(ref_option.deref(), Some(&42));
[01:12:34]     |
[01:12:34]     |
[01:12:34]     = help: add #![feature(inner_deref)] to the crate attributes to enable
[01:12:34] 
[01:12:34] error[E0658]: use of unstable library feature 'inner_deref': newly added (see issue #50264)
[01:12:34]    --> libcore/../libcore/tests/option.rs:309:27
[01:12:34]     |
[01:12:34] 309 |     assert_eq!(ref_option.deref(), None);
[01:12:34]     |
[01:12:34]     |
[01:12:34]     = help: add #![feature(inner_deref)] to the crate attributes to enable
[01:12:34] 
[01:12:35] error[E0599]: no method named `deref_ok` found for type `&std::result::Result<&i32, &u8>` in the current scope
[01:12:35]    --> libcore/../libcore/tests/result.rs:239:23
[01:12:35]     |
[01:12:35] 239 |     assert_eq!(ref_ok.deref_ok(), Ok(&42));
[01:12:35]     |
[01:12:35]     = help: items from traits can only be used if the trait is in scope
[01:12:35]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:12:35]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:12:35]             candidate #1: `use std::result::ResultDeref;`
[01:12:35] 
[01:12:35] error[E0599]: no method named `deref_ok` found for type `&std::result::Result<&i32, &u8>` in the current scope
[01:12:35]    --> libcore/../libcore/tests/result.rs:240:23
[01:12:35]     |
[01:12:35] 240 |     assert_eq!(ref_ok.deref_ok(), Ok(&42));
[01:12:35]     |
[01:12:35]     = help: items from traits can only be used if the trait is in scope
[01:12:35]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:12:35]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:12:35]             candidate #1: `use std::result::ResultDeref;`
[01:12:35] 
[01:12:35] error[E0599]: no method named `deref` found for type `&std::result::Result<&i32, &u8>` in the current scope
[01:12:35]    --> libcore/../libcore/tests/result.rs:241:23
[01:12:35]     |
[01:12:35] 241 |     assert_eq!(ref_ok.deref(), Ok(&42));
[01:12:35]     |
[01:12:35]     = help: items from traits can only be used if the trait is in scope
[01:12:35]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:12:35]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:12:35]             candidate #1: `use std::result::ResultDeref;`
[01:12:35] 
[01:12:35] error[E0599]: no method named `deref_err` found for type `&std::result::Result<&i32, &u8>` in the current scope
[01:12:35]    --> libcore/../libcore/tests/result.rs:245:24
[01:12:35]     |
[01:12:35] 245 |     assert_eq!(ref_err.deref_err(), Err(&41));
[01:12:35]     |
[01:12:35]     = help: items from traits can only be used if the trait is in scope
[01:12:35]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:12:35]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:12:35]             candidate #1: `use std::result::ResultDeref;`
[01:12:35] 
[01:12:35] error[E0599]: no method named `deref_err` found for type `&std::result::Result<&i32, &u8>` in the current scope
[01:12:35]    --> libcore/../libcore/tests/result.rs:246:24
[01:12:35]     |
[01:12:35] 246 |     assert_eq!(ref_err.deref_err(), Err(&41));
[01:12:35]     |
[01:12:35]     = help: items from traits can only be used if the trait is in scope
[01:12:35]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:12:35]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:12:35]             candidate #1: `use std::result::ResultDeref;`
[01:12:35] 
[01:12:35] error[E0599]: no method named `deref` found for type `&std::result::Result<&i32, &u8>` in the current scope
[01:12:35]    --> libcore/../libcore/tests/result.rs:247:24
[01:12:35]     |
[01:12:35] 247 |     assert_eq!(ref_err.deref(), Err(&41));
[01:12:35]     |
[01:12:35]     = help: items from traits can only be used if the trait is in scope
[01:12:35]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:12:35]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:12:35]             candidate #1: `use std::result::ResultDeref;`
[01:12:35] 
[01:12:35] error[E0599]: no method named `deref_err` found for type `&std::result::Result<&i32, &u8>` in the current scope
[01:12:35]    --> libcore/../libcore/tests/result.rs:250:23
[01:12:35]     |
[01:12:35] 250 |     assert_eq!(ref_ok.deref_err(), Ok(&&42));
[01:12:35]     |
[01:12:35]     = help: items from traits can only be used if the trait is in scope
[01:12:35]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:12:35]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:12:35]             candidate #1: `use std::result::ResultDeref;`
[01:12:35] 
[01:12:35] error[E0599]: no method named `deref_ok` found for type `&std::result::Result<&i32, &u8>` in the current scope
[01:12:35]    --> libcore/../libcore/tests/result.rs:253:24
[01:12:35]     |
[01:12:35] 253 |     assert_eq!(ref_err.deref_ok(), Err(&&41));
[01:12:35]     |
[01:12:35]     = help: items from traits can only be used if the trait is in scope
[01:12:35]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:12:35]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:12:35]             candidate #1: `use std::result::ResultDeref;`
[01:12:36] error: aborting due to 10 previous errors
[01:12:36] 
[01:12:36] Some errors occurred: E0599, E0658.
[01:12:36] For more information about an error, try `rustc --explain E0599`.
---
149116 ./src/llvm-emscripten/test
149024 ./.git/modules
149020 ./.git/modules/src
122708 ./obj/build/bootstrap/debug/incremental/bootstrap-1sil8jgb030ka
122704 ./obj/build/bootstrap/debug/incremental/bootstrap-1sil8jgb030ka/s-f0hrxvhid0-vahhe6-2f3cmz1knheyt
112820 ./obj/build/x86_64-unknown-linux-gnu/test/mir-opt
111052 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
111048 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
107176 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps
107176 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps
102808 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
101924 ./obj/build/bootstrap/debug/incremental/bootstrap-2s8ik7x786gic
101920 ./obj/build/bootstrap/debug/incremental/bootstrap-2s8ik7x786gic/s-f0ht2ahrrb-upqz3e-8chh8umqc096
89904 ./obj/build/x86_64-unknown-linux-gnu/stage1
89880 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
89696 ./src/llvm/test/CodeGen
88292 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
88292 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
88288 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz/s-f0hszry5sj-1ynlg71-113338xyxqhne
85124 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
81160 ./obj/build/x86_64-unknown-linux-gnu/doc/std
78752 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot
78748 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib
---
60840 ./src/llvm-emscripten/lib
56092 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/bin
55348 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release
53568 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build
51784 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-3cudc1zz68gz8
51780 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-3cudc1zz68gz8/s-f0ht1jg0v2-1gae625-go2k6as6xr98
48584 ./obj/build/x86_64-unknown-linux-gnu/stage0/bin
47832 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
46780 ./src/test
46660 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release

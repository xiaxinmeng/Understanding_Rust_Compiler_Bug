
[00:38:49] failures:
[00:38:49] 
[00:38:49] ---- [ui] ui/fmt/send-sync.rs stdout ----
[00:38:49] 	
[00:38:49] error: /checkout/src/test/ui/fmt/send-sync.rs:18: unexpected error: '18:5: 18:9: the trait bound `*mut std::ops::Fn() + 'static: std::marker::Sync` is not satisfied in `[std::fmt::ArgumentV1<'_>]` [E0277]'
[00:38:49] 
[00:38:49] error: /checkout/src/test/ui/fmt/send-sync.rs:19: unexpected error: '19:5: 19:9: the trait bound `*mut std::ops::Fn() + 'static: std::marker::Sync` is not satisfied in `std::fmt::Arguments<'_>` [E0277]'
[00:38:49] 
[00:38:49] error: 2 unexpected errors found, 0 expected errors not found
[00:38:49] status: exit code: 101
[00:38:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fmt/send-sync.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/send-sync.stage2-x86_64-unknown-linux-musl" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-x86_64/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/send-sync.stage2-x86_64-unknown-linux-musl.aux" "-A" "unused"
[00:38:49] unexpected errors (from JSON output): [
[00:38:49]     Error {
[00:38:49]         line_num: 18,
[00:38:49]         kind: Some(
[00:38:49]             Error
[00:38:49]         ),
[00:38:49]         msg: "18:5: 18:9: the trait bound `*mut std::ops::Fn() + \'static: std::marker::Sync` is not satisfied in `[std::fmt::ArgumentV1<\'_>]` [E0277]"
[00:38:49]     },
[00:38:49]     Error {
[00:38:49]         line_num: 19,
[00:38:49]         kind: Some(
[00:38:49]             Error
[00:38:49]         ),
[00:38:49]         msg: "19:5: 19:9: the trait bound `*mut std::ops::Fn() + \'static: std::marker::Sync` is not satisfied in `std::fmt::Arguments<\'_>` [E0277]"
[00:38:49]     }
[00:38:49] ]
[00:38:49] 
[00:38:49] thread '[ui] ui/fmt/send-sync.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:1086:12
[00:38:49] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:38:49] 
[00:38:49] ---- [ui] ui/impl-trait/no-method-suggested-traits.rs stdout ----
[00:38:49] 	
[00:38:49] error: /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:14: unexpected help message: '14:1: 14:1: the following traits are implemented but not in scope, perhaps add a `use` for one of them:'
[00:38:49] 
[00:38:49] error: /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:14: unexpected help message: '14:1: 14:1: the following traits are implemented but not in scope, perhaps add a `use` for one of them:'
[00:38:49] 
[00:38:49] error: /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:14: unexpected help message: '14:1: 14:1: the following trait is implemented but not in scope, perhaps add a `use` for it:'
[00:38:49] 
[00:38:49] error: /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:14: unexpected help message: '14:1: 14:1: the following trait is implemented but not in scope, perhaps add a `use` for it:'
[00:38:49] 
[00:38:49] error: /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:14: unexpected help message: '14:1: 14:1: the following trait is implemented but not in scope, perhaps add a `use` for it:'
[00:38:49] 
[00:38:49] error: /checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs:14: unexpected help message: '14:1: 14:1: the following trait is implemented but not in scope, perhaps add a `use` for it:'
[00:38:49] 
[00:38:49] error: 6 unexpected errors found, 0 expected errors not found
[00:38:49] status: exit code: 101
[00:38:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/no-method-suggested-traits.stage2-x86_64-unknown-linux-musl" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-x86_64/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/no-method-suggested-traits.stage2-x86_64-unknown-linux-musl.aux" "-A" "unused"
[00:38:49] unexpected errors (from JSON output): [
[00:38:49]     Error {
[00:38:49]         line_num: 14,
[00:38:49]         kind: Some(
[00:38:49]             Help
[00:38:49]         ),
[00:38:49]         msg: "14:1: 14:1: the following traits are implemented but not in scope, perhaps add a `use` for one of them:"
[00:38:49]     },
[00:38:49]     Error {
[00:38:49]         line_num: 14,
[00:38:49]         kind: Some(
[00:38:49]             Help
[00:38:49]         ),
[00:38:49]         msg: "14:1: 14:1: the following traits are implemented but not in scope, perhaps add a `use` for one of them:"
[00:38:49]     },
[00:38:49]     Error {
[00:38:49]         line_num: 14,
[00:38:49]         kind: Some(
[00:38:49]             Help
[00:38:49]         ),
[00:38:49]         msg: "14:1: 14:1: the following trait is implemented but not in scope, perhaps add a `use` for it:"
[00:38:49]     },
[00:38:49]     Error {
[00:38:49]         line_num: 14,
[00:38:49]         kind: Some(
[00:38:49]             Help
[00:38:49]         ),
[00:38:49]         msg: "14:1: 14:1: the following trait is implemented but not in scope, perhaps add a `use` for it:"
[00:38:49]     },
[00:38:49]     Error {
[00:38:49]         line_num: 14,
[00:38:49]         kind: Some(
[00:38:49]             Help
[00:38:49]         ),
[00:38:49]         msg: "14:1: 14:1: the following trait is implemented but not in scope, perhaps add a `use` for it:"
[00:38:49]     },
[00:38:49]     Error {
[00:38:49]         line_num: 14,
[00:38:49]         kind: Some(
[00:38:49]             Help
[00:38:49]         ),
[00:38:49]         msg: "14:1: 14:1: the following trait is implemented but not in scope, perhaps add a `use` for it:"
[00:38:49]     }
[00:38:49] ]
[00:38:49] 
[00:38:49] thread '[ui] ui/impl-trait/no-method-suggested-traits.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:1086:12
[00:38:49] 
[00:38:49] ---- [ui] ui/issue-35976.rs stdout ----
[00:38:49] 	
[00:38:49] error: /checkout/src/test/ui/issue-35976.rs:24: expected message not found: another candidate was found in the following trait, perhaps add a `use` for it:
[00:38:49] 
[00:38:49] error: 0 unexpected errors found, 1 expected errors not found
[00:38:49] status: exit code: 101
[00:38:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-35976.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-35976.stage2-x86_64-unknown-linux-musl" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-x86_64/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-35976.stage2-x86_64-unknown-linux-musl.aux" "-A" "unused"
[00:38:49] not found errors (from test file): [
[00:38:49]     Error {
[00:38:49]         line_num: 24,
[00:38:49]         kind: None,
[00:38:49]         msg: "another candidate was found in the following trait, perhaps add a `use` for it:"
[00:38:49]     }
[00:38:49] ]
[00:38:49] 
[00:38:49] thread '[ui] ui/issue-35976.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:1086:12
[00:38:49] 
[00:38:49] 
[00:38:49] failures:
[00:38:49]     [ui] ui/fmt/send-sync.rs
[00:38:49]     [ui] ui/impl-trait/no-method-suggested-traits.rs
[00:38:49]     [ui] ui/issue-35976.rs
[00:38:49] 
[00:38:49] test result: FAILED. 462 passed; 3 failed; 1 ignored; 0 measured; 0 filtered out

plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:48:42] 
[00:48:42] running 4541 tests
[00:48:44] ......................F............................................................................. 100/4541
[00:48:50] .................................................................................................... 300/4541
[00:48:53] .................................................................................................... 400/4541
[00:48:56] .................................................................................................... 500/4541
[00:49:00] .......................i............................................................................ 600/4541
[00:49:00] .......................i............................................................................ 600/4541
[00:49:05] .................................................................................................... 700/4541
[00:49:10] ................................i...........i....................................................... 800/4541
[00:49:13] ...................................................iiiii............................................ 900/4541
[00:49:16] .................................................................................................... 1000/4541
[00:49:18] .................................................................................................... 1100/4541
[00:49:20] ..........................................................................................F......... 1200/4541
[00:49:25] .................................................................................................... 1400/4541
[00:49:28] .........................................i.......................................................... 1500/4541
[00:49:31] ........i........................................................................................... 1600/4541
[00:49:34] .................................................................................................... 1700/4541
[00:49:34] .................................................................................................... 1700/4541
[00:49:38] .................................................................................................... 1800/4541
[00:49:41] ..................................i.............................................F................... 1900/4541
[00:49:45] .....................................................F.............................................. 2000/4541
[00:49:53] .................................................................................................... 2200/4541
[00:49:57] .................................................................................................... 2300/4541
[00:50:00] .................................................................................................... 2400/4541
[00:50:04] .................................................................................................... 2500/4541
---
[00:50:55] .................................................................................................... 4000/4541
[00:50:58] .................................................................................................... 4100/4541
[00:51:02] ..........................i......................................................................... 4200/4541
[00:51:05] .................................................................................................... 4300/4541
[00:51:08] ........................................F........................................................... 4400/4541
mmand: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/E0705.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/E0705/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/E0705/auxiliary" "-A" "unused"
[00:51:12] ------------------------------------------
[00:51:12] 
[00:51:12] ------------------------------------------
[00:51:12] stderr:
[00:51:12] stderr:
[00:51:12] ------------------------------------------
[00:51:12] {"message":"the feature `impl_header_lifetime_elision` has been stable since 1.31.0 and no longer requires an attribute to enable","code":{"code":"stable_features","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/E0705.rs","byte_start":495,"byte_end":523,"line_start":13,"line_end":13,"column_start":12,"column_end":40,"is_primary":true,"text":[{"text":"#![feature(impl_header_lifetime_elision)]","highlight_start":12,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(stable_features)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: the feature `impl_header_lifetime_elision` has been stable since 1.31.0 and no longer requires an attribute to enable\n  --> /checkout/src/test/ui/E0705.rs:13:12\n   |\nLL | #![feature(impl_header_lifetime_elision)]\n   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(stable_features)] on by default\n\n"}
[00:51:12] ------------------------------------------
[00:51:12] 
[00:51:12] thread '[ui] ui/E0705.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:51:12] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:12] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:12] 
[00:51:12] ---- [ui] ui/error-codes/E0637.rs stdout ----
[00:51:12] diff of stderr:
[00:51:12] 
[00:51:12] 10 LL | fn foo<'a: '_>(_: &'a u8) {} //~ ERROR invalid lifetime bound name: `'_`
[00:51:12] 11    |            ^^ `'_` is a reserved lifetime name
[00:51:12] 12 
[00:51:12] - error[E0637]: invalid lifetime bound name: `'_`
[00:51:12] -   --> $DIR/E0637.rs:15:10
[00:51:12] -    |
[00:51:12] - LL | impl<'a: '_> Bar<'a> { //~ ERROR invalid lifetime bound name: `'_`
[00:51:12] -    |          ^^ `'_` is a reserved lifetime name
[00:51:12] - error: aborting due to 3 previous errors
[00:51:12] + error: aborting due to 2 previous errors
[00:51:12] 20 
[00:51:12] 21 For more information about this error, try `rustc --explain E0637`.
[00:51:12] 21 For more information about this error, try `rustc --explain E0637`.
[00:51:12] 22 
[00:51:12] 
[00:51:12] 
[00:51:12] The actual stderr differed from the expected stderr.
[00:51:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0637/E0637.stderr
[00:51:12] To update references, rerun the tests and pass the `--bless` flag
[00:51:12] To only update this specific test, also pass `--test-args error-codes/E0637.rs`
[00:51:12] error: 1 errors occurred comparing output.
[00:51:12] status: exit code: 1
[00:51:12] status: exit code: 1
[00:51:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0637.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0637/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0637/auxiliary" "-A" "unused"
[00:51:12] ------------------------------------------
[00:51:12] 
[00:51:12] ------------------------------------------
[00:51:12] stderr:
[00:51:12] stderr:
[00:51:12] ------------------------------------------
[00:51:12] {"message":"invalid lifetime bound name: `'_`","code":{"code":"E0637","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0637.rs","byte_start":482,"byte_end":484,"line_start":11,"line_end":11,"column_start":16,"column_end":18,"is_primary":true,"text":[{"text":"struct Foo<'a: '_>(&'a u8); //~ ERROR invalid lifetime bound name: `'_`","highlight_start":16,"highlight_end":18}],"label":"`'_` is a reserved lifetime name","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0637]: invalid lifetime bound name: `'_`\n  --> /checkout/src/test/ui/error-codes/E0637.rs:11:16\n   |\nLL | struct Foo<'a: '_>(&'a u8); //~ ERROR invalid lifetime bound name: `'_`\n   |                ^^ `'_` is a reserved lifetime name\n\n"}
[00:51:12] {"message":"invalid lifetime bound name: `'_`","code":{"code":"E0637","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0637.rs","byte_start":550,"byte_end":552,"line_start":12,"line_end":12,"column_start":12,"column_end":14,"is_primary":true,"text":[{"text":"fn foo<'a: '_>(_: &'a u8) {} //~ ERROR invalid lifetime bound name: `'_`","highlight_start":12,"highlight_end":14}],"label":"`'_` is a reserved lifetime name","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0637]: invalid lifetime bound name: `'_`\n  --> /checkout/src/test/ui/error-codes/E0637.rs:12:12\n   |\nLL | fn foo<'a: '_>(_: &'a u8) {} //~ ERROR invalid lifetime bound name: `'_`\n   |            ^^ `'_` is a reserved lifetime name\n\n"}
[00:51:12] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:51:12] {"message":"For more information about this error, try `rustc --explain E0637`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0637`.\n"}
[00:51:12] ------------------------------------------
[00:51:12] 
[00:51:12] thread '[ui] ui/error-codes/E0637.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:51:12] 
[00:51:12] 
[00:51:12] ---- [ui] ui/issues/issue-17905.rs stdout ----
[00:51:12] diff of stderr:
[00:51:12] 
[00:51:12] - error[E0106]: missing lifetime specifier
[00:51:12] + error[E0308]: mismatched method receiver
[00:51:12] +   --> $DIR/issue-17905.rs:18:18
[00:51:12] +    |
[00:51:12] + LL |     fn say(self: &Pa1:12] stderr:
[00:51:12] ------------------------------------------
[00:51:12] {"message":"mismatched method receiver","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n
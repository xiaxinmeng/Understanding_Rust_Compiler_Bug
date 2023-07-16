plain
[00:49:34] ................................................................................................i... 2200/4553
[00:49:38] .................................................................................................... 2300/4553
[00:49:42] .................................................................................................... 2400/4553
[00:49:45] .................................................................................................... 2500/4553
[00:49:49] ........iiiiiiiii................................................................................... 2600/4553
[00:49:55] .................................................................................................... 2800/4553
[00:49:59] .................................................................................................... 2900/4553
[00:50:01] ............................i....................................................................... 3000/4553
[00:50:04] ........................................................................................i.i..ii..... 3100/4553
---
[00:50:50] 
[00:50:50] running 2872 tests
[00:51:00] .................................................................................................... 100/2872
[00:51:10] .............................................................................i...................... 200/2872
[00:51:17] ...........................................................................F....F................... 300/2872
[00:51:35] .................................................................................................... 500/2872
[00:51:45] .................................................................................................... 600/2872
[00:51:58] .................................................................................................... 700/2872
[00:52:08] .................................................................................................... 800/2872
---
[00:55:19] .................................................................................................... 2500/2872
[00:55:43] .................................................................................................... 2600/2872
[00:55:51] .................................................................................................... 2700/2872
[00:55:59] .................................................................................................... 2800/2872
[00:56:08] To only update this specific test, also pass `--test-args binding/pat-tuple-7.rs`
[00:56:08] error: 1 errors occurred comparing output.
[00:56:08] status: exit code: 0
[00:56:08] status: exit code: 0
[00:56:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/binding/pat-tuple-7.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/binding/pat-tuple-7/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/binding/pat-tuple-7/auxiliary"
[00:56:08] ------------------------------------------
[00:56:08] 
[00:56:08] ------------------------------------------
[00:56:08] stderr:
[00:56:08] stderr:
[00:56:08] ------------------------------------------
[00:56:08] {"message":"unnecessary parentheses around pattern","code":{"code":"unused_parens","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/binding/pat-tuple-7.rs","byte_start":514,"byte_end":519,"line_start":15,"line_end":15,"column_start":9,"column_end":14,"is_primary":true,"text":[{"text":"        (pat) => assert_eq!(pat, 0)","highlight_start":9,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_parens)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"remove these parentheses","code":null,"level":"help/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/binding/range-inclusive-pattern-precedence/auxiliary"
[00:56:08] ------------------------------------------
[00:56:08] 
[00:56:08] ------------------------------------------
[00:56:08] stderr:
[00:56:08] stderr:
[00:56:08] ------------------------------------------
[00:56:08] {"message":"unnecessary parentheses around pattern","code":{"code":"unused_parens","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/binding/range-inclusive-pattern-precedence.rs","byte_start":573,"byte_end":582,"line_start":18,"line_end":18,"column_start":10,"column_end":19,"is_primary":true,"text":[{"text":"        &(18..=18) => {}","highlight_start":10,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_parens)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"remove these parentheses","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/binding/range-inclusive-pattern-precedence.rs","byte_start":573,"byte_end":582,"line_start":18,"line_end":18,"column_start":10,"column_end":19,"is_primary":true,"text":[{"text":"        &(18..=18) => {}","highlight_start":10,"highlight_end":19}],"label":null,"suggested_replacement":"18 ..=18","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unnecessary parentheses around pattern\n  --> /checkout/src/test/run-paull,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove these parentheses","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/binding/range-inclusive-pattern-precedence.rs","byte_start":843,"byte_end":858,"line_start":30,"line_end":30,"column_start":13,"column_end":28,"is_primary":true,"text":[{"text":"        box (VALUE..=VALUE) => {}","highlight_start":13,"highlight_end":28}],"label":null,"suggested_replacement":"VALUE ..=VALUE","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unnecessary parentheses around pattern\n  --> /checkout/src/test/run-pass/binding/range-inclusive-pattern-precedence.rs:30:13\n   |\nLL |         box (VALUE..=VALUE) => {}\n   |             ^^^^^^^^^^^^^^^ help: remove these parentheses\n\n"}
[00:56:08] ------------------------------------------
[00:56:08] 
[00:56:08] thread '[run-pass] run-pass/binding/range-inclusive-pattern-precedence.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:56:08] 
---
151464 ./obj/build/bootstrap/debug/incremental
151412 ./src/tools/clang
149116 ./src/llvm-emscripten/test
135996 ./obj/build/bootstrap/debug/incremental/bootstrap-3ivyub3ic2113
135992 ./obj/build/bootstrap/debug/incremental/bootstrap-3ivyub3ic2113/s-f5gfcgykwr-1xq5cpj-1akhp3

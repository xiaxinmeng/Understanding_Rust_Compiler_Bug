plain
travis_time:end:01466efd:start=1546473963103392512,finish=1546474018903361840,duration=55799969328
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:58:50] .................................................................................................... 2600/5223
[00:58:54] .................................................................................................... 2700/5223
[00:58:58] .................................................................................................... 2800/5223
[00:59:01] .................................................................................................... 2900/5223
[00:59:04] ......................................................................................FF............ 3000/5223
[00:59:11] ............i....................................................................................... 3200/5223
[00:59:14] ...........................................................................ii...i..ii............... 3300/5223
[00:59:18] .................................................................................................... 3400/5223
[00:59:21] .................................................................................................... 3500/5223
---
[01:00:19] diff of stderr:
[01:00:19] 
[01:00:19] 11    |            ^^^^^^^^^^
[01:00:19] 12 
[01:00:19] 13 error: denote infinite loops with `loop { ... }`
[01:00:19] +   --> $DIR/lint-impl-fn.rs:27:5
[01:00:19] 15    |
[01:00:19] 15    |
[01:00:19] - LL |         fn foo(&self) { while true {} } //~ ERROR: infinite loops
[01:00:19] -    |                         ^^^^^^^^^^ help: use `loop`
[01:00:19] + LL |     while true {} //~ ERROR: infinite loops
[01:00:19] +    |     ^^^^^^^^^^ help: use `loop`
[01:00:19] 19 note: lint level defined here
[01:00:19] -   --> $DIR/lint-impl-fn.rs:13:8
[01:00:19] +   --> $DIR/lint-impl-fn.rs:25:8
[01:00:19] 21    |
[01:00:19] 21    |
[01:00:19] 22 LL | #[deny(while_true)]
[01:00:19] 
[01:00:19] 24 
[01:00:19] 24 
[01:00:19] 25 error: denote infinite loops with `loop { ... }`
[01:00:19] +   --> $DIR/lint-impl-fn.rs:18:25
[01:00:19] 27    |
[01:00:19] 27    |
[01:00:19] - LL |     while true {} //~ ERROR: infinite loops
[01:00:19] -    |     ^^^^^^^^^^ help: use `loop`
[01:00:19] + LL |         fn foo(&self) { while true {} } //~ ERROR: infinite loops
[01:00:19] +    |                         ^^^^^^^^^^ help: use `loop`
[01:00:19] 31 note: lint level defined here
[01:00:19] -   --> $DIR/lint-impl-fn.rs:25:8
[01:00:19] +   --> $DIR/lint-impl-fn.rs:13:8
[01:00:19] 33    |
[01:00:19] 33    |
[01:00:19] 34 LL | #[deny(while_true)]
[01:00:19] 
[01:00:19] 
[01:00:19] The actual stderr differed from the expected stderr.
[01:00:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-impl-fn/lint-impl-fn.stderr
[01:00:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-impl-fn/lint-impl-fn.stderr
[01:00:19] To update references, rerun the tests and pass the `--bless` flag
[01:00:19] To only update this specific test, also pass `--test-args lint/lint-impl-fn.rs`
[01:00:19] error: 1 errors occurred comparing output.
[01:00:19] status: exit code: 1
[01:00:19] status: exit code: 1
[01:00:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-impl-fn.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-impl-fn/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-impl-fn/auxiliary" "-A" "unused"
[01:00:19] ------------------------------------------
[01:00:19] 
[01:00:19] ------------------------------------------
[01:00:19] stderr:
[01:00:19] stderr:
[01:00:19] ------------------------------------------
[01:00:19] {"message":"denote infinite loops with `loop { ... }`","code":{"code":"while_true","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-impl-fn.rs","byte_start":152,"byte_end":162,"line_start":10,"line_end":10,"column_start":21,"column_end":31,"is_primary":true,"text":[{"text":"    fn bar(&self) { while true {} } //~ ERROR: infinite loops","highlight_start":21,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-impl-fn.rs","byte_start":119,"byte_end":129,"line_start":9,"line_end":9,"column_start":12,"column_end":22,"is_primary":true,"text":[{"text":"    #[deny(while_true)]","highlight_start":12,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"use `loop`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-impl-fn.rs","byte_start":152,"byte_end":162,"line_start":10,"line_end":10,"column_start":21,"column_end":31,"is_primary":true,"text":[{"text":"    fn bar(&self) { while true {} } //~ ERROR: infinite loops","highlight_start":21,"highlight_end":31}],"label":null,"suggested_replacement":"loop","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: denote infinite loops with `loop { ... }`\n  --> /checkout/src/test/ui/lint/lint-impl-fn.rs:10:21\n   |\nLL |     fn bar(&self) { while true {} } //~ ERROR: infinite loops\n   |                     ^^^^^^^^^^ help: use `loop`\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/lint-impl-fn.rs:9:12\n   |\nLL |     #[deny(while_true)]\n   |            ^^^^^^^^^^\n\n"}
[01:00:19] {"message":"denote infinite loops with `loop { ... }`","code":{"code":"while_true","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-impl-fn.rs","byte_start":443,"byte_end":453,"line_start":27,"line_end":27,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    while true {} //~ ERROR: infinite loops","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-impl-fn.rs","byte_start":414,"byte_end":424,"line_start":25,"line_end":25,"column_start":8,"column_end":18,"is_primary":true,"text":[{"text":"#[deny(while_true)]","highlight_start":8,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"use `loop`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-impl-fn.rs","byte_start":443,"byte_end":453,"line_start":27,"line_end":27,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    while true {} //~ ERROR: infinite loops","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":"loop","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: denote infinite loops with `loop { ... }`\n  --> /checkout/src/test/ui/lint/lint-impl-fn.rs:27:5\n   |\nLL |     while true {} //~ ERROR: infinite loops\n   |     ^^^^^^^^^^ help: use `loop`\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/lint-impl-fn.rs:25:8\n   |\nLL | #[deny(while_true)]\n   |        ^^^^^^^^^^\n\n"}
[01:00:19] {"message":"denote infinite loops with `loop { ... }`","code":{"code":"while_true","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-impl-fn.rs","byte_start":286,"byte_end":296,"line_start":18,"line_end":18,"column_start":25,"column_end":35,"is_primary":true,"text":[{"text":"        fn foo(&self) { while true {} } //~ ERROR: infinite loops","highlight_start":25,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-impl-fn.rs","byte_start":204,"byte_end":214,"line_start":13,"line_end":13,"column_start":8,"column_end":18,"is_primary":true,"text":[{"text":"#[deny(while_true)]","highlight_start":8,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"use `loop`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-impl-fn.rs","byte_start":286,"byte_end":296,"line_start":18,"line_end":18,"column_start":25,"column_end":35,"is_primary":true,"text":[{"text":"        fn foo(&self) { while true {} } //~ ERROR: infinite loops","highlight_start":25,"highlight_end":35}],"label":null,"suggested_replacement":"loop","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: denote infinite loops with `loop { ... }`\n  --> /checkout/src/test/ui/lint/lint-impl-fn.rs:18:25\n   |\nLL |         fn foo(&self) { while true {} } //~ ERROR: infinite loops\n   |                         ^^^^^^^^^^ help: use `loop`\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/lint-impl-fn.rs:13:8\n   |\nLL | #[deny(while_true)]\n   |        ^^^^^^^^^^\n\n"}
[01:00:19] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[01:00:19] ------------------------------------------
[01:00:19] 
[01:00:19] thread '[ui] ui/lint/lint-impl-fn.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:00:19] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:19] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:19] 
[01:00:19] ---- [ui] ui/lint/lint-group-nonstandard-style.rs stdout ----
[01:00:19] diff of stderr:
[01:00:19] 
[01:00:19] 37    |              ^^^^^^^^^^^^^^^^^
[01:00:19] 38    = note: #[forbid(non_snake_case)] implied by #[forbid(nonstandard_style)]
[01:00:19] 39 
[01:00:19] - error: static variable `bad` should have an upper case name such as `BAD`
[01:00:19] -    |
[01:00:19] -    |
[01:00:19] - LL |         static bad: isize = 1; //~ ERROR should have an upper
[01:00:19] -    |
[01:00:19] - note: lint level defined here
[01:00:19] -   --> $DIR/lint-group-nonstandard-style.rs:10:14
[01:00:19] -    |
[01:00:19] -    |
[01:00:19] - LL |     #[forbid(nonstandard_style)]
[01:00:19] -    |              ^^^^^^^^^^^^^^^^^
[01:00:19] -    = note: #[forbid(non_upper_case_globals)] implied by #[forbid(nonstandard_style)]
[01:00:19] - 
[01:00:19] 53 warning: function `CamelCase` should have a snake case name such as `camel_case`
[01:00:19] 55    |
[01:00:19] 
[01:00:19] 
[01:00:19] 62 LL |         #![warn(nonstandard_style)]
[01:00:19] 63    |                 ^^^^^^^^^^^^^^^^^
[01:00:19] 64    = note: #[warn(non_snake_case)] implied by #[warn(nonstandard_style)]
[01:00:19] + 
[01:00:19] + error: static variable `bad` should have an upper case name such as `BAD`
[01:00:19] +   --> $DIR/lint-group-nonstandard-style.rs:14:9
[01:00:19] +    |
[01:00:19] + LL |         static bad: isize = 1; //~ ERROR should have an upper
[01:00:19] +    |
[01:00:19] + note: lint level defined here
[01:00:19] +   --> $DIR/lint-group-nonstandard-style.rs:10:14
[01:00:19] +    |
[01:00:19] +    |
[01:00:19] + LL |     #[forbid(nonstandard_style)]
[01:00:19] +    |              ^^^^^^^^^^^^^^^^^
[01:00:19] +    = note: #[forbid(non_upper_case_globals)] implied by #[forbid(nonstandard_style)]
[01:00:19] 66 error: aborting due to 3 previous errors
[01:00:19] 67 
[01:00:19] 
[01:00:19] 
[01:00:19] 
[01:00:19] The actual stderr differed from the expected stderr.
[01:00:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-group-nonstandard-style/lint-group-nonstandard-style.stderr
[01:00:19] To update references, rerun the tests and pass the `--bless` flag
[01:00:19] To only update this specific test, also pass `--test-args lint/lint-group-nonstandard-style.rs`
[01:00:19] error: 1 errors occurred comparing output.
[01:00:19] status: exit code: 1
[01:00:19] status: exit code: 1
[01:00:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-group-nonstandard-style.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-group-nonstandard-style/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-group-nonstandard-style/auxiliary" "-A" "unused"
[01:00:19] ------------------------------------------
[01:00:19] 
[01:00:19] ------------------------------------------
[01:00:19] stderr:
[01:00:19] stderr:
[01:00:19] ------------------------------------------
[01:00:19] {"message":"type `snake_case` should have a camel case name such as `SnakeCase`","code":{"code":"non_camel_case_types","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-group-nonstandard-style.rs","byte_start":450,"byte_end":468,"line_start":22,"line_end":22,"column_start":9,"column_end":27,"is_primary":true,"text":[{"text":"        struct snake_case; //~ WARN should have a camel","highlight_start":9,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-group-nonstandard-style.rs","byte_start":365,"byte_end":382,"line_start":18,"line_end":18,"column_start":17,"column_end":34,"is_primary":true,"text":[{"text":"        #![warn(nonstandard_style)]","highlight_start":17,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"#[warn(non_camel_case_types)] implied by #[warn(nonstandard_style)]","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: type `snake_case` should have a camel case name such as `SnakeCase`\n  --> /checkout/src/test/ui/lint/lint-group-nonstandard-style.rs:22:9\n   |\nLL |         struct snake_case; //~ WARN should have a camel\n   |         ^^^^^^^^^^^^^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/lint-group-nonstandard-style.rs:18:17\n   |\nLL |         #![warn(nonstandard_style)]\n   |                 ^^^^^^^^^^^^^^^^^\n   = note: #[warn(non_camel_case_types)] implied by #[warn(nonstandard_style)]\n\n"}
[01:00:19] {"message":"function `CamelCase` should have a snake case name such as `camel_case`","code":{"code":"non_snake_case","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-group-nonstandard-style.rs","byte_start":50,"byte_end":67,"line_start":4,"line_end":4,"column_start":1,"column_end":18,"is_primary":true,"text":[{"text":"fn CamelCase() {} //~ ERROR should have a snake","highlight_start":1,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-group-nonstandard-style.rs","byte_start":8,"byte_end":25,"line_start":1,"line_end":1,"column_start":9,"column_end":26,"is_primary":true,"text":[{"text":"#![deny(nonstandard_style)]","highlight_start":9,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"#[deny(non_snake_case)] implied by #[deny(nonstandard_style)]","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: function `CamelCase` should have a snake case name such as `camel_case`\n  --> /checkout/src/test/ui/lint/lint-group-nonstandard-style.rs:4:1\n   |\nLL | fn CamelCase() {} //~ ERROR should have a snake\n   | ^^^^^^^^^^^^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/lint-group-nonstandard-style.rs:1:9\n   |\nLL | #![deny(nonstandard_style)]\n   |         ^^^^^^^^^^^^^^^^^\n   = note: #[deny(non_snake_case)] implied by #[deny(nonstandard_style)]\n\n"}
[01:00:19] {"message":"function `CamelCase` should have a snake case name such as `camel_case`","code":{"code":"non_snake_case","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-group-nonstandard-style.rs","byte_start":216,"byte_end":233,"line_start":12,"line_end":12,"column_start":9,"column_end":26,"is_primary":true,"text":[{"text":"        fn CamelCase() {} //~ ERROR should have a snake","highlight_start":9,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-group-nonstandard-style.rs","byte_start":174,"byte_end":191,"line_start":10,"line_end":10,"column_start":14,"column_end":31,"is_primary":true,"text":[{"text":"    #[forbid(nonstandard_style)]","highlight_start":14,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"#[forbid(non_snake_case)] implied by #[forbid(nonstandard_style)]","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: function `CamelCase` should have a snake case name such as `camel_case`\n  --> /checkout/src/test/ui/lint/lint-group-nonstandard-style.rs:12:9\n   |\nLL |         fn CamelCase() {} //~ ERROR should have a snake\n   |         ^^^^^^^^^^^^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/lint-group-nonstandard-style.rs:10:14\n   |\nLL |     #[forbid(nonstandard_style)]\n   |              ^^^^^^^^^^^^^^^^^\n   = note: #[forbid(non_snake_case)] implied by #[forbid(nonstandard_style)]\n\n"}
[01:00:19] {"message":"function `CamelCase` should have a snake case name such as `camel_case`","code":{"code":"non_snake_case","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-group-nonstandard-style.rs","byte_start":394,"byte_end":411,"line_start":20,"line_end":20,"column_start":9,"column_end":26,"is_primary":true,"text":[{"text":"        fn CamelCase() {} //~ WARN should have a snake","highlight_start":9,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-group-nonstandard-style.rs","byte_start":365,"byte_end":382,"line_start":18,"line_end":18,"column_start":17,"column_end":34,"is_primary":true,"text":[{"text":"        #![warn(nonstandard_style)]","highlight_start":17,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"#[warn(non_snake_case)] implied by #[warn(nonstandard_style)]","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: function `CamelCase` should have a snake case name such as `camel_case`\n  --> /checkout/src/test/ui/lint/lint-group-nonstandard-style.rs:20:9\n   |\nLL |         fn CamelCase() {} //~ WARN should have a snake\n   |         ^^^^^^^^^^^^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/lint-group-nonstandard-style.rs:18:17\n   |\nLL |         #![warn(nonstandard_style)]\n   |                 ^^^^^^^^^^^^^^^^^\n   = note: #[warn(non_snake_case)] implied by #[warn(nonstandard_style)]\n\n"}
[01:00:19] {"message":"static variable `bad` should have an upper case name such as `BAD`","code":{"code":"non_upper_case_globals","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-group-nonstandard-style.rs","byte_start":273,"byte_end":295,"line_start":14,"line_end":14,"column_start":9,"column_end":31,"is_primary":true,"text":[{"text":"        static bad: isize = 1; //~ ERROR should have an upper","highlight_start":9,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-group-nonstandard-style.rs","byte_start":174,"byte_end":191,"line_start":10,"line_end":10,"column_start":14,"column_end":31,"is_primary":true,"text":[{"text":"    #[forbid(nonstandard_style)]","highlight_start":14,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"#[forbid(non_upper_case_globals)] implied by #[forbid(nonstandard_style)]","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: static variable `bad` should have an upper case name such as `BAD`\n  --> /checkout/src/test/ui/lint/lint-group-nonstandard-style.rs:14:9\n   |\nLL |         static bad: isize = 1; //~ ERROR should have an upper\n   |         ^^^^^^^^^^^^^^^^^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/lint-group-nonstandard-style.rs:10:14\n   |\nLL |     #[forbid(nonstandard_style)]\n   |              ^^^^^^^^^^^^^^^^^\n   = note: #[forbid(non_upper_case_globals)] implied by #[forbid(nonstandard_style)]\n\n"}
[01:00:19] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[01:00:19] ------------------------------------------
[01:00:19] 
[01:00:19] thread '[ui] ui/lint/lint-group-nonstandard-style.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:00:19] 
---
[01:00:19] 
[01:00:19] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:00:19] 
[01:00:19] 
[01:00:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:19] 
[01:00:19] 
[01:00:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:19] Build completed unsuccessfully in 0:03:53
[01:00:19] Build completed unsuccessfully in 0:03:53
[01:00:19] make: *** [check] Error 1
[01:00:19] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06e391fa
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jan  3 01:07:26 UTC 2019
---
travis_time:end:24e7d250:start=1546477647587309601,finish=1546477647592089852,duration=4780251
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01e61eae
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2b17bb48
travis_time:start:2b17bb48
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:055f6d6b
$ dmesg | grep -i kill

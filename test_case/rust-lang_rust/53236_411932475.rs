plain
[00:48:23] .......i............................................................................................
[00:48:26] ....................................................................................................
[00:48:28] ....................................................................................................
[00:48:30] ....................................................................................................
[00:48:32] ..........................................................F.........................................
[00:48:39] ....................................................................................................
[00:48:41] .........................i..........................................................................
[00:48:43] ....................................................................................................
[00:48:46] ....................................................................................................
[00:48:46] ....................................................................................................
[00:48:48] ....................................................................................................
[00:48:52] ....................................................................................................
[00:48:55] ....................................................................................................
[00:48:58] ....................................................................................................
[00:49:01] ....................................................................................................
[00:49:05] ....................i...............................................................................
[00:49:08] ..............................i...................................................FF................
[00:49:11] ..................................................................................................F.
[00:49:18] ....................................................i...............................................
[00:49:20] ............................................i....
[00:49:20] failures:
[00:49:20] 
[00:49:20] 
[00:49:20] ---- [ui] ui/feature-gate-raw-identifiers.rs stdout ----
[00:49:20] 
[00:49:20] error: ui test compiled successfully!
[00:49:20] status: exit code: 0
[00:49:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gate-raw-identifiers.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate-raw-identifiers/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate-raw-identifiers/auxiliary" "-A" "unused"
[00:49:20] ------------------------------------------
[00:49:20] 
[00:49:20] ------------------------------------------
[00:49:20] stderr:
---
[00:49:20] 
[00:49:20] ---- [ui] ui/raw-literal-self.rs stdout ----
[00:49:20] diff of stderr:
[00:49:20] 
[00:49:20] 1 error: `r#self` is not currently supported.
[00:49:20] +   --> $DIR/raw-literal-self.rs:13:14
[00:49:20] 3    |
[00:49:20] 3    |
[00:49:20] 4 LL | fn self_test(r#self: u32) {
[00:49:20] 
[00:49:20] 
[00:49:20] The actual stderr differed from the expected stderr.
[00:49:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/raw-literal-self/raw-literal-self.stderr
[00:49:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/raw-literal-self/raw-literal-self.stderr
[00:49:20] To update references, rerun the tests and pass the `--bless` flag
[00:49:20] To only update this specific test, also pass `--test-args raw-literal-self.rs`
[00:49:20] error: 1 errors occurred comparing output.
[00:49:20] status: exit code: 1
[00:49:20] status: exit code: 1
[00:49:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/raw-literal-self.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/raw-literal-self/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "parse-only" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/raw-literal-self/auxiliary" "-A" "unused"
[00:49:20] ------------------------------------------
[00:49:20] 
[00:49:20] ------------------------------------------
[00:49:20] stderr:
[00:49:20] stderr:
[00:49:20] ------------------------------------------
[00:49:20] {"message":"`r#self` is not currently supported.","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/raw-literal-self.rs","byte_start":513,"byte_end":519,"line_start":13,"line_end":13,"column_start":14,"column_end":20,"is_primary":true,"text":[{"text":"fn self_test(r#self: u32) {","highlight_start":14,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `r#self` is not currently supported.\n  --> /checkout/src/test/ui/raw-literal-self.rs:13:14\n   |\nLL | fn self_test(r#self: u32) {\n   |              ^^^^^^\n\n"}
[00:49:20] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:49:20] ------------------------------------------
[00:49:20] 
[00:49:20] thread '[ui] ui/raw-literal-self.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:49:20] 
[00:49:20] 
[00:49:20] ---- [ui] ui/raw-literal-keywords.rs stdout ----
[00:49:20] diff of stderr:
[00:49:20] 
[00:49:20] 1 error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `true`
[00:49:20] +   --> $DIR/raw-literal-keywords.rs:14:10
[00:49:20] 3    |
[00:49:20] 3    |
[00:49:20] 4 LL |     r#if true { } //~ ERROR found `true`
[00:49:20] 5    |          ^^^^ expected one of 8 possible tokens here
[00:49:20] 6 
[00:49:20] 6 
[00:49:20] 7 error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `Test`
[00:49:20] +   --> $DIR/raw-literal-keywords.rs:18:14
[00:49:20] 9    |
[00:49:20] 9    |
[00:49:20] 10 LL |     r#struct Test; //~ ERROR found `Test`
[00:49:20] 11    |              ^^^^ expected one of 8 possible tokens here
[00:49:20] 12 
[00:49:20] 12 
[00:49:20] 13 error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `Test`
[00:49:20] +   --> $DIR/raw-literal-keywords.rs:22:13
[00:49:20] 15    |
[00:49:20] 15    |
[00:49:20] 16 LL |     r#union Test; //~ ERROR found `Test`
[00:49:20] 17    |             ^^^^ expected one of 8 possible tokens here
[00:49:20] 
[00:49:20] The actual stderr differed from the expected stderr.
[00:49:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/raw-literal-keywords/raw-literal-keywords.stderr
[00:49:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/raw-literal-keywords/raw-literal-keywords.stderr
[00:49:20] To update references, rerun the tests and pass the `--bless` flag
[00:49:20] To only update this specific test, also pass `--test-args raw-literal-keywords.rs`
[00:49:20] error: 1 errors occurred comparing output.
[00:49:20] status: exit code: 1
[00:49:20] status: exit code: 1
[00:49:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/raw-literal-keywords.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/raw-literal-keywords/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "parse-only" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/raw-literal-keywords/auxiliary" "-A" "unused"
[00:49:20] ------------------------------------------
[00:49:20] 
[00:49:20] ------------------------------------------
[00:49:20] stderr:
[00:49:20] stderr:
[00:49:20] ------------------------------------------
[00:49:20] {"message":"expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `true`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/raw-literal-keywords.rs","byte_start":524,"byte_end":528,"line_start":14,"line_end":14,"column_start":10,"column_end":14,"is_primary":true,"text":[{"text":"    r#if true { } //~ ERROR found `true`","highlight_start":10,"highlight_end":14}],"label":"expected one of 8 possible tokens here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `true`\n  --> /checkout/src/test/ui/raw-literal-keywords.rs:14:10\n   |\nLL |     r#if true { } //~ ERROR found `true`\n   |          ^^^^ expected one of 8 possible tokens here\n\n"}
[00:49:20] {"message":"expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `Test`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/raw-literal-keywords.rs","byte_start":591,"byte_end":595,"line_start":18,"line_end":18,"column_start":14,"column_end":18,"is_primary":true,"text":[{"text":"    r#struct Test; //~ ERROR found `Test`","highlight_start":14,"highlight_end":18}],"label":"expected one of 8 possible tokens here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `Test`\n  --> /checkout/src/test/ui/raw-literal-keywords.rs:18:14\n   |\nLL |     r#struct Test; //~ ERROR found `Test`\n   |              ^^^^ expected one of 8 possible tokens here\n\n"}
[00:49:20] {"message":"expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `Test`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/raw-literal-keywords.rs","byte_start":653,"byte_end":657,"line_start":22,"line_end":22,"column_start":13,"column_end":17,"is_primary":true,"text":[{"text":"    r#union Test; //~ ERROR found `Test`","highlight_start":13,"highlight_end":17}],"label":"expected one of 8 possible tokens here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `Test`\n  --> /checkout/src/test/ui/raw-literal-keywords.rs:22:13\n   |\nLL |     r#union Test; //~ ERROR found `Test`\n   |             ^^^^ expected one of 8 possible tokens here\n\n"}
[00:49:20] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:49:20] ------------------------------------------
[00:49:20] 
[00:49:20] thread '[ui] ui/raw-literal-keywords.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:49:20] 
[00:49:20] 
[00:49:20] ---- [ui] ui/rust-2018/async-ident.rs stdout ----
[00:49:20] diff of fixed:
[00:49:20] 
[00:49:20] 8 // option. This file may not be copied, modified, or distributed
[00:49:20] 9 // except according to those terms.
[00:49:20] 10 
[00:49:20] - #![feature(raw_identifiers)]
[00:49:20] 12 #![allow(dead_code, unused_variables, non_camel_case_types, non_upper_case_globals)]
[00:49:20] 13 #![deny(async_idents)]
[00:49:20] 
[00:49:20] 
[00:49:20] The actual fixed differed from the expected fixed.
[00:49:20] The actual fixed differed from the expected fixed.
[00:49:20] Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/async-ident/async-ident.fixed
[00:49:20] To update references, rerun the tests and pass the `--bless` flag
[00:49:20] To only update this specific test, also pass `--test-args rust-2018/async-ident.rs`
[00:49:20] error: 1 errors occurred comparing output.
[00:49:20] status: exit code: 1
[00:49:20] status: exit code: 1
[00:49:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/async-ident.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/async-ident/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/async-ident/auxiliary" "-A" "unused"
[00:49:20] ------------------------------------------
[00:49:20] 
[00:49:20] ------------------------------------------
[00:49:20] stderr:
[00:49:20] stderr:
[00:49:20] ------------------------------------------
[00:49:20] {"message":"`async` is a keyword in the 2018 edition","code":{"code":"async_idents","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":611,"byte_end":616,"line_start":17,"line_end":17,"column_start":4,"column_end":9,"is_primary":true,"text":[{"text":"fn async() {} //~ ERROR async","highlight_start":4,"highlight_end":9}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":560,"byte_end":572,"line_start":12,"line_end":12,"column_start":9,"column_end":21,"is_primary":true,"text":[{"text":"#![deny(async_idents)]","highlight_start":9,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"you can use a raw identifier to stay compatible","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":611,"byte_end":616,"line_start":17,"line_end":17,"column_start":4,"column_end":9,"is_primary":true,"text":[{"text":"fn async() {} //~ ERROR async","highlight_start":4,"highlight_end":9}],"label":null,"suggested_replacement":"r#async","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `async` is a keyword in the 2018 edition\n  --> /checkout/src/test/ui/rust-2018/async-ident.rs:17:4\n   |\nLL | fn async() {} //~ ERROR async\n   |    ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/rust-2018/async-ident.rs:12:9\n   |\nLL | #![deny(async_idents)]\n   |         ^^^^^^^^^^^^\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!\n   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>\n\n"}
[00:49:20] {"message":"`async` is a keyword in the 2018 edition","code":{"code":"async_idents","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":729,"byte_end":734,"line_start":22,"line_end":22,"column_start":7,"column_end":12,"is_primary":true,"text":[{"text":"    ($async:expr, async) => {};","highlight_start":7,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"you can use a raw identifier to stay compatible","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":729,"byte_end":734,"line_start":22,"line_end":22,"column_start":7,"column_end":12,"is_primary":true,"text":[{"text":"    ($async:expr, async) => {};","highlight_start":7,"highlight_end":12}],"label":null,"suggested_replacement":"r#async","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `async` is a keyword in the 2018 edition\n  --> /checkout/src/test/ui/rust-2018/async-ident.rs:22:7\n   |\nLL |     ($async:expr, async) => {};\n   |       ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!\n   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>\n\n"}
[00:49:20] {"message":"`async` is a keyword in the 2018 edition","code":{"code":"async_idents","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":741,"byte_end":746,"line_start":22,"line_end":22,"column_start":19,"column_end":24,"is_primary":true,"text":[{"text":"    ($async:expr, async) => {};","highlight_start":19,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"you can use a raw identifier to stay compatible","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":741,"byte_end":746,"line_start":22,"line_end":22,"column_start":19,"column_end":24,"is_primary":true,"text":[{"text":"    ($async:expr, async) => {};","highlight_start":19,"highlight_end":24}],"label":null,"suggested_replacement":"r#async","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `async` is a keyword in the 2018 edition\n  --> /checkout/src/test/ui/rust-2018/async-ident.rs:22:19\n   |\nLL |     ($async:expr, async) => {};\n   |                   ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!\n   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>\n\n"}
[00:49:20] {"message":"`async` is a keyword in the 2018 edition","code":{"code":"async_idents","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":975,"byte_end":980,"line_start":36,"line_end":36,"column_start":11,"column_end":16,"is_primary":true,"text":[{"text":"    trait async {}","highlight_start":11,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"you can use a raw identifier to stay compatible","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":975,"byte_end":980,"line_start":36,"line_end":36,"column_start":11,"column_end":16,"is_primary":true,"text":[{"text":"    trait async {}","highlight_start":11,"highlight_end":16}],"label":null,"suggested_replacement":"r#async","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `async` is a keyword in the 2018 edition\n  --> /checkout/src/test/ui/rust-2018/async-ident.rs:36:11\n   |\nLL |     trait async {}\n   |           ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!\n   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>\n\n"}
[00:49:20] {"message":"`async` is a keyword in the 2018 edition","code":{"code":"async_idents","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":1080,"byte_end":1085,"line_start":40,"line_end":40,"column_start":10,"column_end":15,"is_primary":true,"text":[{"text":"    impl async for MyStruct {}","highlight_start":10,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"you can use a raw identifier to stay compatible","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":1080,"byte_end":1085,"line_start":40,"line_end":40,"column_start":10,"column_end":15,"is_primary":true,"text":[{"text":"    impl async for MyStruct {}","highlight_start":10,"highlight_end":15}],"label":null,"suggested_replacement":"r#async","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `async` is a keyword in the 2018 edition\n  --> /checkout/src/test/ui/rust-2018/async-ident.rs:40:10\n   |\nLL |     impl async for MyStruct {}\n   |          ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`\n   |\n   = warning: this was previously accepted by theckout/src/test/ui/rust-2018/async-ident.rs:46:12\n   |\nLL |     static async: u32 = 0;\n   |            ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!\n   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>\n\n"}
[00:49:20] {"message":"`async` is a keyword in the 2018 edition","code":{"code":"async_idents","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":1314,"byte_end":1319,"line_start":52,"line_end":52,"column_start":11,"column_end":16,"is_primary":true,"text":[{"text":"    const async: u32 = 0;","highlight_start":11,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"you can use a raw identifier to stay compatible","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":1314,"byte_end":1319,"line_start":52,"line_end":52,"column_start":11,"column_end":16,"is_primary":true,"text":[{"text":"    const async: u32 = 0;","highlight_start":11,"highlight_end":16}],"label":null,"suggested_replacement":"r#async","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `async` is a keyword in the 2018 edition\n  --> /checkout/src/test/ui/rust-2018/async-ident.rs:52:11\n   |\nLL |     const async: u32 = 0;\n   |           ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!\n   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>\n\n"}
[00:49:20] {"message":"`async` is a keyword in the 2018 edition","code":{"code":"async_idents","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":1425,"byte_end":1430,"line_start":58,"line_end":58,"column_start":15,"column_end":20,"is_primary":true,"text":[{"text":"impl Foo { fn async() {} }","highlight_start":15,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"you can use a raw identifier to stay compatible","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":1425,"byte_end":1430,"line_start":58,"line_end":58,"column_start":15,"column_end":20,"is_primary":true,"text":[{"text":"impl Foo { fn async() {} }","highlight_start":15,"highlight_end":20}],"label":null,"suggested_replacement":"r#async","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `async` is a keyword in the 2018 edition\n  --> /checkout/src/test/ui/rust-2018/async-ident.rs:58:15\n   |\nLL | impl Foo { fn async() {} }\n   |               ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!\n   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>\n\n"}
[00:49:20] {"message":"`async` is a keyword in the 2018 edition","code":{"code":"async_idents","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":1528,"byte_end":1533,"line_start":63,"line_end":63,"column_start":12,"column_end":17,"is_primary":true,"text":[{"text":"    struct async {}","highlight_start":12,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"you can use a raw identifier to stay compatible","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":1528,"byte_end":1533,"line_start":63,"line_end":63,"column_start":12,"column_end":17,"is_primary":true,"text":[{"text":"    struct async {}","highlight_start":12,"highlight_end":17}],"label":null,"suggested_replacement":"r#async","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `async` is a keyword in the 2018 edition\n  --> /checkout/src/test/ui/rust-2018/async-ident.rs:63:12\n   |\nLL |     struct async {}\n   |            ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!\n   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>\n\n"}
[00:49:20] {"message":"`async` is a keyword in the 2018 edition","code":{"code":"async_idents","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":1611,"byte_end":1616,"line_start":66,"line_end":66,"column_start":9,"column_end":14,"is_primary":true,"text":[{"text":"    let async: async = async {};","highlight_start":9,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"you can use a raw identifier to stay compatible","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":1611,"byte_end":1616,"line_start":66,"line_end":66,"column_start":9,"column_end":14,"is_primary":true,"text":[{"text":"    let async: async = async {};","highlight_start":9,"highlight_end":14}],"label":null,"suggested_replacement":"r#async","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `async` is a keyword in the 2018 edition\n  --> /checkout/src/test/ui/rust-2018/async-ident.rs:66:9\n   |\nLL |     let async: async = async {};\n   |         ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!\n   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>\n\n"}
[00:49:20] {"message":"`async` is a keyword in the 2018 edition","code":{"code":"async_idents","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":1618,"byte_end":1623,"line_start":66,"line_end":66,"column_start":16,"column_end":21,"is_primary":true,"text":[{"text":"    let async: async = async {};","highlight_start":16,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"you can use a raw identifier to stay compatible","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":1618,"byte_end":1623,"line_start":66,"line_end":66,"column_start":16,"column_end":21,"is_primary":true,"text":[{"text":"    let async: async = async {};","highlight_start":16,"highlight_end":21}],"label":null,"suggested_replacement":"r#async","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `async` is a keyword in the 2018 edition\n  --> /checkout/src/test/ui/rust-2018/async-ident.rs:66:16\n   |\nLL |     let async: async = async {};\n   |                ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!\n   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>\n\n"}
[00:49:20] {"message":"`async` is a keyword in the 2018 edition","code":{"code":"async_idents","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":1626,"byte_end":1631,"line_start":66,"line_end":66,"column_start":24,"column_end":29,"is_primary":true,"text":[{"text":"    let async: async = async {};","highlight_start":24,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"you can use a raw identifier to stay compatible","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":1626,"byte_end":1631,"line_start":66,"line_end":66,"column_start":24,"column_end":29,"is_primary":true,"text":[{"text":"    let async: async = async {};","highlight_start":24,"highlight_end":29}],"label":null,"suggested_replacement":"r#async","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `async` is a keyword in the 2018 edition\n  --> /checkout/src/test/ui/rust-2018/async-ident.rs:66:24\n   |\nLL |     let async: async = async {};\n   |                        ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!\n   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>\n\n"}
[00:49:20] {"message":"`async` is a keyword in the 2018 edition","code":{"code":"async_idents","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":1901,"byte_end":1906,"line_start":77,"line_end":77,"column_start":19,"column_end":24,"is_primary":true,"text":[{"text":"    () => (pub fn async() {})","highlight_start":19,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"you can use a raw identifier to stay compatible","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":1901,"byte_end":1906,"line_start":77,"line_end":77,"column_start":19,"column_end":24,"is_primary":true,"text":[{"text":"    () => (pub fn async() {})","highlight_start":19,"highlight_end":24}],"label":null,"suggested_replacement":"r#async","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `async` is a keyword in the 2018 edition\n  --> /checkout/src/test/ui/rust-2018/async-ident.rs:77:19\n   |\nLL |     () => (pub fn async() {})\n   |                   ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!\n   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>\n\n"}
[00:49:20] {"message":"`async` is a keyword in the 2018 edition","code":{"code":"async_idents","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":2033,"byte_end":2038,"line_start":84,"line_end":84,"column_start":6,"column_end":11,"is_primary":true,"text":[{"text":"    (async) => (1)","highlight_start":6,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"you can use a raw identifier to stay compatible","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":2033,"byte_end":2038,"line_start":84,"line_end":84,"column_start":6,"column_end":11,"is_primary":true,"text":[{"text":"    (async) => (1)","highlight_start":6,"highlight_end":11}],"label":null,"suggested_replacement":"r#async","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `async` is a keyword in the 2018 edition\n  --> /checkout/src/test/ui/rust-2018/async-ident.rs:84:6\n   |\nLL |     (async) => (1)\n   |      ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!\n   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>\n\n"}
[00:49:20] {"message":"aborting due to 14 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 14 previous errors\n\n"}
[00:49:20] ------------------------------------------
[00:49:20] 
[00:49:20] thread '[ui] ui/rust-2018/async-ident.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:49:20] 
---
[00:49:20] 
[00:49:20] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:49:20] 
[00:49:20] 
[00:49:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:49:20] 
[00:49:20] 
[00:49:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:49:20] Build completed unsuccessfully in 0:02:16
[00:49:20] Build completed unsuccessfully in 0:02:16
[00:49:20] make: *** [check] Error 1
[00:49:20] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01f152c2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:27477dc5
$ sudo tail -n 500 /var/log/syslog
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] e820: last_pfn = 0xbfff3 max_arch_pfn = 0x400000000
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff8800000f2800]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] Using GB pages for direct mapping
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] kvm-clock: using sched offset of 1456537219 cycles
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] Zone ranges:
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000]   Device   empty
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] Movable zone start for each node
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] Early memory node ranges
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug -bd0f-4baa36da527d kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] Policy zone: Normal
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel6:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.450071] pid_max: default: 32768 minimum: 301
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.451628] ACPI: Core revision 20150930
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.458688] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.461475] Security Framework initialized
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.462896] Yama: becoming mindful.
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.464176] AppArmor: AppArmor disabled by boot time parameter
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.468315] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.479426] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.485197] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.488006] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.490878] Initializing cgroup subsys io
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.492255] Initializing cgroup subsys memory
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.494058] Initializing cgroup subsys devices
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.495882] Initializing cgroup subsys freezer
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.497563] Initializing cgroup subsys net_cls
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.498874] Initializing cgroup subsys perf_event
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.500229] Initializing cgroup subsys net_prio
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.501684] Initializing cgroup subsys hugetlb
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.502990] Initializing cgroup subsys pids
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.504454] CPU: Physical Processor ID: 0
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.505954] CPU: Processor Core ID: 0
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.507837] mce: CPU supports 32 MCE banks
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.509337] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.511395] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.516254] Freeing SMP alternatives memory: 32K
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.527024] ftrace: allocating 32185 entries in 126 pages
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.582139] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.584615] smpboot: Max logical packages: 2
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.586924] x2apic enabled
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.589379] Switched APIC routing to physical x2apic.
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.594223] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.701072] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.704875] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.708651] x86: Booting SMP configuration:
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.709981] .... node  #0, CPUs:      #1
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.711522] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.717035]  #2
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.717872] kvm-clock: cpu 2, msr 3:ffff1081, secondaa527d kernel: [    0.752166] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.754554] pinctrl core: initialized pinctrl subsystem
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.756320] RTC time: 22:56:23, date: 08/09/18
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.758612] NET: Registered protocol family 16
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.769139] cpuidle: using governor ladder
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.781125] cpuidle: using governor menu
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.782635] PCCT header not found.
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.783925] ACPI: bus type PCI registered
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.785396] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.788271] PCI: Using configuration type 1 for base access
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.802142] ACPI: Added _OSI(Module Device)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.803671] ACPI: Added _OSI(Processor Device)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.805103] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.806593] ACPI: Added _OSI(Processor Aggregator Device)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.811326] ACPI: Executed 2 blocks of module-level executable AML code
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.835752] ACPI: Interpreter enabled
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.837561] ACPI: (supports S0 S3 S4 S5)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.838894] ACPI: Using IOAPIC for interrupt routing
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.840738] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.871645] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.874399] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.876759] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.878924] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.883652] PCI host bridge to bus 0000:00
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.885200] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 wibd0f-4baa36da527d kernel: [    0.981861] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.989369] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    0.995614] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.012768] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.016644] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.020153] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.023757] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.027704] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.048517] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.050694] vgaarb: loaded
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.051839] SCSI subsystem initialized
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.053143] libata version 3.00 loaded.
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.053167] ACPI: bus type USB registered
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.054679] usbcore: registered new interface driver usbfs
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.056617] usbcore: registered new interface driver hub
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.058578] usbcore: registered new device driver usb
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.060466] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.062654] dmi: Firmware registration failed.
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.064316] PCI: Using ACPI for IRQ routing
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.065824] PCI: pci_cache_line_size set to 64 bytes
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.065922] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.065923] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.066026] NetLabel: Initializing
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.067212] NetLabel:  domain hash size = 128
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.068677] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.070337] NetLabel:  unlabeled traffic allowed by default
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.072329] amd_nb: Cannot enumerate AMD northbridges
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.074034] clocksource: Switched to clocksource kvm-clock
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.082294] pnp: PnP ACPI init
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.083647] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.083715] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.083759] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.083808] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.083868] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.083908] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.083948] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.084100] pnp: PnP ACPI: found 7 devices
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.093258] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.096871] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.096874] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.096876] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.096877] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.096907] NET: Registered protocol family 2
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.098748] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.101247] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.103656] TCP: Hash tables configured (established 131072 bind 65536)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.105836] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.107833] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.110803] NET: Registered protocol family 1
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.112334] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.114495] PCI: CLS 0 bytes, default 64
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    1.114546] Unpacking initramfs...
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.151175] Freeing initrd memory: 21432K
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.152850] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.154898] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.158271] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.161305] hw unit of domain pp0-core 2^-0 Joules
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.162918] hw unit of domain package 2^-0 Joules
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.164492] hw unit of domain dram 2^-16 Joules
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.166052] Scanning for low memory corruption every 60 seconds
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.168629] audit: initializing netlink subsys (disabled)
Aug  9 22:56:34 traviug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.200915] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.203905] io scheduler noop registered
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.205114] io scheduler deadline registered (default)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.207096] io scheduler cfq registered
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.208585] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.210631] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.212962] intel_idle: does not run on family 6 model 63
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.213048] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.215362] ACPI: Power Button [PWRF]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.216975] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.219214] ACPI: Sleep Button [SLPF]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.220929] GHES: HEST is not enabled!
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.225493] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.227591] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.234448] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.236547] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.244480] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.268117] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.293363] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.318327] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.343009] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.347339] Linux agpgart interface v0.103
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.352632] loop: module loaded
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.354153] libphy: Fixed MDIO Bus: probed
Aug  9 22:56:34 trver registered
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.448317] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.450214] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.452056] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.454647] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.456974] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.460901] registered taskstats version 1
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.462693] Loading compiled-in X.509 certificates
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.465390] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.469261] zswap: loaded using pool lzo/zbud
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.473643] Key type trusted registered
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.479260] Key type encrypted registered
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.480774] ima: No TPM chip found, activating TPM-bypass!
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.482910] evm: HMAC attrs: 0x1
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.484522]   Magic number: 14:912:956
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.485993] virtio-pci 0000:00:04.0: hash matches
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.487923] memory memory9: hash matches
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.489630] rtc_cmos 00:00: setting system clock to 2018-08-09 22:56:26 UTC (1533855386)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.492886] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.495305] EDD information not available.
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.497137] PM: Hibernation image not present or could not be loaded.
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.498646] Freeing unused kernel memory: 1496K
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.500188] Write protecting the kernel read-only data: 14336k
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.503293] Freeing unused kernel memory: 1956K
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.505461] Freeing unused kernel memory: 92K
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.522059] systemd-udevd[118]: starting version 204
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.585295] scsi host0: Virtio SCSI HBA
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.589871] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.597858] AVX2 version of gcm_enc/dec engaged.
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.600213] AES CTR mode by8 optimization enabled
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.622642] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.646603] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.646667] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.646668] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.647113] sd 0:0:1:0: [sda] Write Protect is off
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.647116] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.647178] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.649075]  sda: sda1
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    3.650437] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    4.162241] tsc: Refined TSC clocksource calibration: 2300.001 MHz
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    4.165695] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x212735f0517, max_idle_ns: 440795237604 ns
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    4.459208] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    6.574374] floppy0: no floppy controllers found
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    7.734111] raid6: sse2x1   gen()  8757 MB/s
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    7.802092] raid6: sse2x1   xor()  6571 MB/s
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    7.870088] raid6: sse2x2   gen() 10771 MB/s
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    7.938089] raid6: sse2x2   xor()  7183 MB/s
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    8.006098] raid6: sse2x4   gen() 12487 MB/s
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    8.074126] raid6: sse2x4   xor()  8710 MB/s
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    8.142115] raid6: avx2x1   gen() 16875 MB/s
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    8.210096] raid6: avx2x2   gen() 19960 MB/s
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    8.278092] raid6: avx2x4   gen() 22029 MB/s
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    8.280465] raid6: using algorithm avx2x4 gen() 22029 MB/s
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    8.283313] raid6: using avx2x2 recovery algorithm
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    8.287456] xor: automatically using best checksumming function:
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    8.330101]    avx       : 26756.000 MB/sec
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    8.348083] Btrfs loaded
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    8.407698] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    8.411369] EXT4-fs (sda1): write access will be enabled during recovery
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    8.510950] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    8.521181] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    8.524150] EXT4-fs (sda1): recovery complete
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [    8.532332] EXT4-fs (sda1): mounted file  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [   10.278143] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [   10.346386] random: cloud-init: uninitialized urandom read (32 bytes read, 57 bits of entropy available)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [   10.510711] random: cloud-init: uninitialized urandom read (32 bytes read, 57 bits of entropy available)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [   10.763814] random: mktemp: uninitialized urandom read (12 bytes read, 60 bits of entropy available)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [   10.838534] random: mktemp: uninitialized urandom read (6 bytes read, 61 bits of entropy available)
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [   10.907655] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [   10.942779] EXT4-fs (sda1): resized filesystem to 7864064
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [   11.271363] init: failsafe main process (1094) killed by TERM signal
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d instance-setup: INFO Running set_multiqueue.
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d instance-setup: INFO Set channels for eth0 to 4.
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  9 22:56:34 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug .
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d google-accounts: INFO Creating a new user account for me.
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d google-accounts: INFO Created user account me.
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d google-accounts: INFO Creating a new user account for bogdana.
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d google-clock-skew: INFO Clock drift token has changed: 0.
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d google-clock-skew: INFO Clock drift token has changed: 0.
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d google-accounts: INFO Created user account bogdana.
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d google-accounts: INFO Creating a new user account for aj.
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d google-accounts: INFO Created user account aj.
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d google-accounts: INFO Creating a new user account for asari.
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d google-accounts: INFO Created user account asari.
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d google-accounts: INFO Removing user packer.
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d cron[1429]: (CRON) INFO (pidfile fd = 3)
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d cron[1473]: (CRON) STARTUP (fork ok)
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d cron[1473]: (CRON) INFO (Running @reboot jobs)
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d acpid: starting up with netlink and the input layer
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d acpid: 1 rule loaded
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d acpid: waiting for events: event logging is off
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d haveged: haveged starting up
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d pollinate: To re-seed this system again, use the -r|--reseed option
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [   12.434652] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [   12.445193] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [   12.668892] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [   12.672013] Bridge firewalling registered
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [   12.680494] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [   12.742734] Initializing XFRM netlink socket
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [   12.748818] Netfilter messages via NETLINK v0.30.
Aug  9 22:56:35 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [   12.751305] ctnetlink v0.93: registering with nfnetlink.
Aug  9 22:56:36 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d google-clock-skew: INFO Synced system time with hardware clock.
Aug  9 22:56:36 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [   13.078143] floppy0: no floppy controllers found
Aug  9 22:56:58 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d ntpdate[1763]: adjust time server 169.254.169.254 offset 0.000554 sec
Aug  9 22:57:05 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d ntpd[1798]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  9 22:57:05 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d ntpd[1799]: proto: precision = 0.102 usec
Aug  9 22:57:05 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d ntpd[1799]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 22:57:05 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d ntpd[1799]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  9 22:57:05 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d ntpd[1799]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  9 22:57:05 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d ntpd[1799]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  9 22:57:05 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d ntpd[1799]: Lia0764684-f552-4cd8-bd0f-4baa36da527d ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  9 22:57:05 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d ec2: 1024 fc:f3:f4:a1:c7:da:35:53:67:b0:94:43:fc:aa:49:af  root@travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d (DSA)
Aug  9 22:57:05 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d ec2: 256 11:80:4b:9f:99:5f:49:2b:00:20:77:e1:66:32:37:14  root@travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d (ECDSA)
Aug  9 22:57:05 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d ec2: 256 60:3d:4c:7d:15:d3:c5:d0:cd:f2:a7:af:fa:0f:82:e9  root@travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d (ED25519)
Aug  9 22:57:05 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d ec2: 2048 0e:d4:60:c3:1b:16:d5:78:6b:c1:ed:5f:2e:fd:0c:95  root@travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d (RSA)
Aug  9 22:57:05 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  9 22:57:05 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d ec2: #############################################################
Aug  9 22:57:39 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [   75.913087] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  9 22:58:47 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [  144.451853] device vethc4dacfa entered promiscuous mode
Aug  9 22:58:47 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [  144.451919] docker0: port 1(vethc4dacfa) entered forwarding state
Aug  9 22:58:47 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [  144.451927] docker0: port 1(vethc4dacfa) entered forwarding state
Aug  9 22:58:47 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [  144.452317] docker0: port 1(vethc4dacfa) entered disabled state
Aug  9 22:58:47 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [  144.542844] cgroup: docker-runc (4779) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  9 22:58:47 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [  144.542849] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  9 22:58:47 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [  144.634005] eth0: renamed from veth778b7bd
Aug  9 22:58:47 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [  144.670211] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  9 22:58:47 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [  144.671348] docker0: port 1(vethc4dacfa) entered forwarding state
Aug  9 22:58:47 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [  144.671365] docker0: port 1(vethc4dacfa) entered forwarding state
Aug  9 22:58:47 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d kernel: [  144.671385] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  9 22:58:51 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d ntpd[1799]: Listen normally on 5 docker0 fe80::42:83ff:fe72:1eaf UDP 123
Aug  9 22:58:51 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d ntpd[1799]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug  9 22:58:51 travis-job-a0764684-f552-4cd8-bd0f-4baa36da527d ntpd[1799]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123

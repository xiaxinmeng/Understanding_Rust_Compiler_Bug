plain
[00:48:17] 
[00:48:17] running 2249 tests
[00:48:21] ....................................................................................................
[00:48:25] ...............i....................................................................................
[00:48:28] .......i..................................................................FF.F......................
[00:48:33] ....................................................................................................
[00:48:35] ....................................................................................................
[00:48:35] ....................................................................................................
[00:48:37] .........................................................F..........................................
[00:48:44] ....................................................................................................
[00:48:46] .........................i..........................................................................
[00:48:49] ....................................................................................................
[00:48:51] ....................................................................................................
[00:48:51] ....................................................................................................
[00:48:54] ....................................................................................................
[00:48:57] ....................................................................................................
[00:49:01] ....................................................................................................
[00:49:04] ....................................................................................................
[00:49:07] ....................................................................................................
[00:49:11] ....................i...............................................................................
[00:49:14] ..............................i...................................................FF................
[00:49:17] .................................................................................................FF.
[00:49:27] ............................................i....
[00:49:27] failures:
[00:49:27] 
[00:49:27] ---- [ui] ui/edition-keywords-2015-2015-parsing.rs stdout ----
[00:49:27] ---- [ui] ui/edition-keywords-2015-2015-parsing.rs stdout ----
[00:49:27] diff of stderr:
[00:49:27] 
[00:49:27] 1 error: no rules expected the token `r#async`
[00:49:27] -   --> $DIR/edition-keywords-2015-2015-parsing.rs:24:31
[00:49:27] +   --> $DIR/edition-keywords-2015-2015-parsing.rs:22:31
[00:49:27] 3    |
[00:49:27] 4 LL |     r#async = consumes_async!(r#async); //~ ERROR no rules expected the token `r#async`
[00:49:27] 
[00:49:27] 6 
[00:49:27] 6 
[00:49:27] 7 error: no rules expected the token `async`
[00:49:27] -   --> $DIR/edition-keywords-2015-2015-parsing.rs:25:35
[00:49:27] +   --> $DIR/edition-keywords-2015-2015-parsing.rs:23:35
[00:49:27] 9    |
[00:49:27] 10 LL |     r#async = consumes_async_raw!(async); //~ ERROR no rules expected the token `async`
[00:49:27] 
[00:49:27] 
[00:49:27] The actual stderr differed from the expected stderr.
[00:49:27] The actual stderr differed from the expected stderr.
[00:49:27] Actual strt":31,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no rules expected the token `r#async`\n  --> /checkout/src/test/ui/edition-keywords-2015-2015-parsing.rs:22:31\n   |\nLL |     r#async = consumes_async!(r#async); //~ ERROR no rules expected the token `r#async`\n   |                               ^^^^^^^\n\n"}
[00:49:27] {"message":"no rules expected the token `async`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/edition-keywords-2015-2015-parsing.rs","byte_start":822,"byte_end":827,"line_start":23,"line_end":23,"column_start":35,"column_end":40,"is_primary":true,"text":[{"text":"    r#async = consumes_async_raw!(async); //~ ERROR no rules expected the token `async`","highlight_start":35,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no rules expected the token `async`\n  --> /checkout/src/test/ui/edition-keywords-2015-2015-parsing.rs:23:35\n   |\nLL |     r#async = consumes_async_raw!(async); //~ ERROR no rules expected the token `async`\n   |                                   ^^^^^\n\n"}
[00:49:27] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:49:27] ------------------------------------------
[00:49:27] 
[00:49:27] thread '[ui] ui/edition-keywords-2015-2015-parsing.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:49:27] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:49:27] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:49:27] 
[00:49:27] ---- [ui] ui/edition-keywords-2015-2018-expansion.rs stdout ----
[00:49:27] diff of stderr:
[00:49:27] 
[00:49:27] 1 error: expected identifier, found reserved keyword `async`
[00:49:27] -   --> $DIR/edition-keywords-2015-2018-expansion.rs:20:5
[00:49:27] +   --> $DIR/edition-keywords-2015-2018-expansion.rs:18:5
[00:49:27] 3    |
[00:49:27] 4 LL |     produces_async! {} //~ ERROR expected identifier, found reserved keyword
[00:49:27] 5    |     ^^^^^^^^^^^^^^^^^^ expected identifier, found reserved keyword
[00:49:27] 
[00:49:27] The actual stderr differed from the expected stderr.
[00:49:27] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/edition-keywords-2015-2018-expansion/edition-keywords-2015-2018-expansion.stderr
[00:49:27] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/edition-keywords-2015-2018-expansion/edition-keywords-2015-2018-expansion.stderr
[00:49:27] To update references, rerun the tests and pass the `--bless` flag
[00:49:27] To only update this specific test, also pass `--test-args edition-keywords-2015-2018-expansion.rs`
[00:49:27] error: 1 errors occurred comparing output.
[00:49:27] status: exit code: 1
[00:49:27] status: exit code: 1
[00:49:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/edition-keywords-2015-2018-expansion.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/edition-keywords-2015-2018-expansion/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/edition-keywords-2015-2018-expansion/auxiliary" "-A" "unused"
[00:49:27] ------------------------------------------
[00:49:27] 
[00:49:27] ------------------------------------------
[00:49:27] stderr:
[00:49:27] stderr:
[00:49:27] ------------------------------------------
[00:49:27] {"message":"expected identifier, found reserved keyword `async`","code":null,"level":"error","spans":[{"file_name":"<produces_async macros>","byte_start":17,"byte_end":22,"line_start":1,"line_end":1,"column_start":18,"column_end":23,"is_primary":true,"text":[{"text":"(  ) => ( pub fn async (  ) {  } )","highlight_start":18,"highlight_end":23}],"label":"expected identifier, found reserved keyword","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/edition-keywords-2015-2018-expansion.rs","byte_start":592,"byte_end":610,"line_start":18,"line_end":18,"column_start":5,"column_end":23,"is_primary":false,"text":[{"text":"    produces_async! {} //~ ERROR expected identifier, found reserved keyword","highlight_start":5,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"produces_async!","def_site_span":{"file_name":"<produces_async macros>","byte_start":0,"byte_end":34,"line_start":1,"line_end":1,"column_start":1,"column_end":35,"is_primary":false,"text":[{"text":"(  ) => ( pub fn async (  ) {  } )","highlight_start":1,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: expected identifier, found reserved keyword `async`\n  --> /checkout/src/test/ui/edition-keywords-2015-2018-expansion.rs:18:5\n   |\nLL |     produces_async! {} //~ ERROR expected identifier, found reserved keyword\n   |     ^^^^^^^^^^^^^^^^^^ expected identifier, found reserved keyword\n   |\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[00:49:27] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:49:27] ------------------------------------------
[00:49:27] 
[00:49:27] thread '[ui] ui/edition-keywords-2015-2018-expansion.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:49:27] 
[00:49:27] 
[00:49:27] ---- [ui] ui/edition-keywords-2015-2018-parsing.rs stdout ----
[00:49:27] diff of stderr:
[00:49:27] 
[00:49:27] 1 error: no rules expected the token `r#async`
[00:49:27] -   --> $DIR/edition-keywords-2015-2018-parsing.rs:24:31
[00:49:27] +   --> $DIR/edition-keywords-2015-2018-parsing.rs:22:31
[00:49:27] 3    |
[00:49:27] 4 LL |     r#async = consumes_async!(r#async); //~ ERROR no rules expected the token `r#async`
[00:49:27] 
[00:49:27] 6 
[00:49:27] 6 
[00:49:27] 7 error: no rules expected the token `async`
[00:49:27] -   --> $DIR/edition-keywords-2015-2018-parsing.rs:25:35
[00:49:27] +   --> $DIR/edition-keywords-2015-2018-parsing.rs:23:35
[00:49:27] 9    |
[00:49:27] 10 LL |     r#async = consumes_async_raw!(async); //~ ERROR no rules expected the token `async`
[00:49:27] 
[00:49:27] 
[00:49:27] The actual stderr differed from the expected stderr.
[00:49:27] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/edition-keywords-2015-2018-parsing/edition-keywords-2015-2018-parsing.stderr
[00:49:27] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/edition-keywords-2015-2018-parsing/edition-keywords-2015-2018-parsing.stderr
[00:49:27] To update references, rerun the tests and pass the `--bless` flag
[00:49:27] To only update this specific test, also pass `--test-args edition-keywords-2015-2018-parsing.rs`
[00:49:27] error: 1 errors occurred comparing output.
[00:49:27] status: exit code: 1
[00:49:27] status: exit code: 1
[00:49:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/edition-keywords-2015-2018-parsing.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/edition-keywords-2015-2018-parsing/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/edition-keywords-2015-2018-parsing/auxiliary" "-A" "unused"
[00:49:27] ------------------------------------------
[00:49:27] 
[00:49:27] ------------------------------------------
[00:49:27] stderr:
[00:49:27] stderr:
[00:49:27] ------------------------------------------
[00:49:27] {"message":"no rules expected the token `r#async`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/edition-keywords-2015-2018-parsing.rs","byte_start":730,"byte_end":737,"line_start":22,"line_end":22,"column_start":31,"column_end":38,"is_primary":true,"text":[{"text":"    r#async = consumes_async!(r#async); //~ ERROR no rules expected the token `r#async`","highlight_start":31,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no rules expected the token `r#async`\n  --> /checkout/src/test/ui/edition-keywords-2015-2018-parsing.rs:22:31\n   |\nLL |     r#async = consumes_async!(r#async); //~ ERROR no rules expected the token `r#async`\n   |                               ^^^^^^^\n\n"}
[00:49:27] {"message":"no rules expected the token `async`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/edition-keywords-2015-2018-parsing.rs","byte_start":822,"byte_end":827,"line_start":23,"line_end":23,"column_start":35,"column_end":40,"is_primary":true,"text":[{"text":"    r#async = consumes_async_raw!(async); //~ ERROR no rules expected the token `async`","highlight_start":35,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no rules expected the token `async`\n  --> /checkout/src/test/ui/edition-keywords-2015-2018-parsing.rs:23:35\n   |\nLL |     r#async = consumes_async_raw!(async); //~ ERROR no rules expected the token `async`\n   |                                   ^^^^^\n\n"}
[00:49:27] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:49:27] 
[00:49:27] ----------------------------------rue`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/raw-literal-keywords.rs","byte_start":524,"byte_end":528,"line_start":14,"line_end":14,"column_start":10,"column_end":14,"is_primary":true,"text":[{"text":"    r#if true { } //~ ERROR found `true`","highlight_start":10,"highlight_end":14}],"label":"expected one of 8 possible tokens here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `true`\n  --> /checkout/src/test/ui/raw-literal-keywords.rs:14:10\n   |\nLL |     r#if true { } //~ ERROR found `true`\n   |          ^^^^ expected one of 8 possible tokens here\n\n"}
[00:49:27] {"message":"expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `Test`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/raw-literal-keywords.rs","byte_start":591,"byte_end":595,"line_start":18,"line_end":18,"column_start":14,"column_end":18,"is_primary":true,"text":[{"text":"    r#struct Test; //~ ERROR found `Test`","highlight_start":14,"highlight_end":18}],"label":"expected one of 8 possible tokens here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `Test`\n  --> /checkout/src/test/ui/raw-literal-keywords.rs:18:14\n   |\nLL |     r#struct Test; //~ ERROR found `Test`\n   |              ^^^^ expected one of 8 possible tokens here\n\n"}
[00:49:27] {"message":"expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `Test`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/raw-literal-keywords.rs","byte_start":653,"byte_end":657,"line_start":22,"line_end":22,"column_start":13,"column_end":17,"is_primary":true,"text":[{"text":"    r#union Test; //~ ERROR found `Test`","highlight_start":13,"highlight_end":17}],"label":"expected one of 8 possible tokens here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `Test`\n  --> /checkout/src/test/ui/raw-literal-keywords.rs:22:13\n   |\nLL |     r#union Test; //~ ERROR found `Test`\n   |             ^^^^ expected one of 8 possible tokens here\n\n"}
[00:49:27] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:49:27] ------------------------------------------
[00:49:27] 
[00:49:27] thread '[ui] ui/raw-literal-keywords.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:49:27] 
[00:49:27] 
[00:49:27] ---- [ui] ui/rust-2018/async-ident-allowed.rs stdout ----
[00:49:27] diff of stderr:
[00:49:27] 
[00:49:27] 2   --> $DIR/async-ident-allowed.rs:19:9
[00:49:27] 3    |
[00:49:27] 4 LL |     let async = 3; //~ ERROR: is a keyword
[00:49:27] -    |         ^^^^^
[00:49:27] +    |         ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
[00:49:27] 7 note: lint level defined here
[00:49:27] 8   --> $DIR/async-ident-allowed.rs:13:9
[00:49:27] 8   --> $DIR/async-ident-allowed.rs:13:9
ext":"    let async = 3; //~ ERROR: is a keyword","highlight_start":9,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident-allowed.rs","byte_start":509,"byte_end":532,"line_start":13,"line_end":13,"column_start":9,"column_end":32,"is_primary":true,"text":[{"text":"#![deny(rust_2018_compatibility)]","highlight_start":9,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"#[deny(async_idents)] implied by #[deny(rust_2018_compatibility)]","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"you can use a raw identifier to stay compatible","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident-allowed.rs","byte_start":655,"byte_end":660,"line_start":19,"line_end":19,"column_start":9,"column_end":14,"is_primary":true,"text":[{"text":"    let async = 3; //~ ERROR: is a keyword","highlight_start":9,"highlight_end":14}],"label":null,"suggested_replacement":"r#async","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `async` is a keyword in the 2018 edition\n  --> /checkout/src/test/ui/rust-2018/async-ident-allowed.rs:19:9\n   |\nLL |     let async = 3; //~ ERROR: is a keyword\n   |         ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/rust-2018/async-ident-allowed.rs:13:9\n   |\nLL | #![deny(rust_2018_compatibility)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^\n   = note: #[deny(async_idents)] implied by #[deny(rust_2018_compatibility)]\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!\n   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>\n\n"}
[00:49:27] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:49:27] ------------------------------------------
[00:49:27] 
[00:49:27] thread '[ui] ui/rust-2018/async-ident-allowed.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:49:27] 
[00:49:27] 
[00:49:27] ---- [ui] ui/rust-2018/async-ident.rs stdout ----
[00:49:27] diff of stderr:
[00:49:27] 
[00:49:27] 1 error: `async` is a keyword in the 2018 edition
[00:49:27] -   --> $DIR/async-ident.rs:18:4
[00:49:27] +   --> $DIR/async-ident.rs:17:4
[00:49:27] 3    |
[00:49:27] 4 LL | fn async() {} //~ ERROR async
[00:49:27] 5    |    ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
[00:49:27] 6    |
[00:49:27] 7 note: lint level defined here
[00:49:27] -   --> $DIR/async-ident.rs:13:9
[00:49:27] +   --> $DIR/async-ident.rs:12:9
[00:49:27] +   --> $DIR/async-ident.rs:12:9
[00:49:27] 9    |
[00:49:27] 10 LL | #![deny(async_idents)]
[00:49:27] 
[00:49:27] 13    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
[00:49:27] 14 
[00:49:27] 14 
[00:49:27] 15 error: `async` is a keyword in the 2018 edition
[00:49:27] -   --> $DIR/async-ident.rs:23:7
[00:49:27] +   --> $DIR/async-ident.rs:22:7
[00:49:27] 17    |
[00:49:27] 18 LL |     ($async:expr, async) => {};
[00:49:27] 19    |       ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
[00:49:27] 22    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
[00:49:27] 23 
[00:49:27] 23 
[00:49:27] 24 error: `async` is a keyword in the 2018 edition
[00:49:27] -   --> $DIR/async-ident.rs:23:19
[00:49:27] +   --> $DIR/async-ident.rs:22:19
[00:49:27] 26    |
[00:49:27] 27 LL |     ($async:expr, async) => {};
[00:49:27] 28    |                   ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
[00:49:27] 31    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
[00:49:27] 32 
[00:49:27] 32 
[00:49:27] 33 error: `async` is a keyword in the 2018 edition
[00:49:27] -   --> $DIR/async-ident.rs:37:11
[00:49:27] +   --> $DIR/async-ident.rs:36:11
[00:49:27] 35    |
[00:49:27] 36 LL |     trait async {}
[00:49:27] 37    |           ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
[00:49:27] 40    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
[00:49:27] 41 
[00:49:27] 41 
[00:49:27] 42 error: `async` is a keyword in the 2018 edition
[00:49:27] -   --> $DIR/async-ident.rs:41:10
[00:49:27] +   --> $DIR/async-ident.rs:40:10
[00:49:27] 44    |
[00:49:27] 45 LL |     impl async for MyStruct {}
[00:49:27] 46    |          ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
[00:49:27] 49    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
[00:49:27] 50 
[00:49:27] 50 
[00:49:27] 51 error: `async` is a keyword in the 2018 edition
[00:49:27] -   --> $DIR/async-ident.rs:47:12
[00:49:27] +   --> $DIR/async-ident.rs:46:12
[00:49:27] 53    |
[00:49:27] 54 LL |     static async: u32 = 0;
[00:49:27] 55    |            ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
[00:49:27] 58    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
[00:49:27] 59 
[00:49:27] 59 
[00:49:27] 60 error: `async` is a keyword in the 2018 edition
[00:49:27] -   --> $DIR/async-ident.rs:53:11
[00:49:27] +   --> $DIR/async-ident.rs:52:11
[00:49:27] 62    |
[00:49:27] 63 LL |     const async: u32 = 0;
[00:49:27] 64    |           ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`
[00:49:27] 67    = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>
[00:49:27] 68 
[00:49:27] 68 
[00:49:27] 69 error: `async` is a keyword in the 2018 edition
[00:49:27] -   --> $DIR/async-identight_end":9}],"label":null,"suggested_replacement":"r#async","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `async` is a keyword in the 2018 edition\n  --> /checkout/src/test/ui/rust-2018/async-ident.rs:17:4\n   |\nLL | fn async() {} //~ ERROR async\n   |    ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/rust-2018/async-ident.rs:12:9\n   |\nLL | #![deny(async_idents)]\n   |         ^^^^^^^^^^^^\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!\n   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>\n\n"}
[00:49:27] {"message":"`async` is a keyword in the 2018 edition","code":{"code":"async_idents","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":729,"byte_end":734,"line_start":22,"line_end":22,"column_start":7,"column_end":12,"is_primary":true,"text":[{"text":"    ($async:expr, async) => {};","highlight_start":7,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"you can use a raw identifier to stay compatible","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":729,"byte_end":734,"line_start":22,"line_end":22,"column_start":7,"column_end":12,"is_primary":true,"text":[{"text":"    ($async:expr, async) => {};","highlight_start":7,"highlight_end":12}],"label":null,"suggested_replacement":"r#async","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `async` is a keyword in the 2018 edition\n  --> /checkout/src/test/ui/rust-2018/async-ident.rs:22:7\n   |\nLL |     ($async:expr, async) => {};\n   |       ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!\n   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>\n\n"}
[00:49:27] {"message":"`async` is a keyword in the 2018 edition","code":{"code":"async_idents","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":741,"byte_end":746,"line_start":22,"line_end":22,"column_start":19,"column_end":24,"is_primary":true,"text":[{"text":"    ($async:expr, async) => {};","highlight_start":19,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard erro11,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"you can use a raw identifier to stay compatible","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":975,"byte_end":980,"line_start":36,"line_end":36,"column_start":11,"column_end":16,"is_primary":true,"text":[{"text":"    trait async {}","highlight_start":11,"highlight_end":16}],"label":null,"suggested_replacement":"r#async","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `async` is a keyword in the 2018 edition\n  --> /checkout/src/test/ui/rust-2018/async-ident.rs:36:11\n   |\nLL |     trait async {}\n   |           ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!\n   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>\n\n"}
[00:49:27] {"message":"`async` is a keyword in the 2018 edition","code":{"code":"async_idents","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/you can use a raw identifier to stay compatible: `r#async`\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!\n   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>\n\n"}
[00:49:27] {"message":"`async` is a keyword in the 2018 edition","code":{"code":"async_idents","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":1314,"byte_end":1319,"line_start":52,"line_end":52,"column_start":11,"column_end":16,"is_primary":true,"text":[{"text":"    const async: u32 = 0;","highlight_start":11,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"you can use a raw identifier to stay compatible","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":1314,"byte_end":1319,"line_start":52,"line_end":52,"column_start":11,"column_end":16,"is_primary":true,"text":[{"text":"    const async: u32 = 0;","highlight_start":11,"highlight_end":16}],"label":null,"suggested_replacement":"r#async","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `async` is a keyword in the 2018 edition\n  --> /checkout/src/test/ui/rust-2018/async-ident.rs:52:11\n   |\nLL |     const async: u32 = 0;\n   |           ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!\n   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>\n\n"}
[00:49:27] {"message":"`async` is a keyword in the 2018 edition","code":{"code":"async_idents","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":1425,"byte_end":1430,"line_start":58,"line_end":58,"column_start":15,"column_end":20,"is_primary":true,"text":[{"text":"impl Foo { fn async() {} }","highlight_start":15,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"you can use a raw identifier to stay compatible","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":1425,"byte_end":1430,"line_start":58,"line_end":58,"column_start":15,"column_end":20,"is_primary":true,"text":[{"text":"impl Foo { fn async() {} }","highlight_start":15,"highlight_end":20}],"label":null,"suggested_replacement":"r#async","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `async` is a keyword in the 2018 edition\n  --> /checkout/src/test/ui/rust-2018/async-ident.rs:58:15\n   |\nLL | impl Foo { fn async() {} }\n   |               ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!\n   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>\n\n"}
[00:49:27] {"message":"`async` is a keyword in the 2018 edition","code":{"code":"async_idents","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":1528,"byte_end":1533,"line_start":63,"line_end":63,"column_start":12,"column_end":17,"is_primary":true,"text":[{"text":"    struct async {}","highlight_start":12,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"you can use a raw identifier to stay compatible","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":1528,"byte_end":1533,"line_start":63,"line_end":63,"column_start":12,"column_end":17,"is_primary":true,"text":[{"text":"    struct async {}","highlight_start":12,"highlight_end":17}],"label":null,"suggested_replacement":"r#async","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `async` is a keyword in the 2018 edition\n  --> /checkout/src/test/ui/rust-2018/async-ident.rs:63:12\n   |\nLL |     struct async {}\n   |            ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!\n   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>\n\n"}
[00:49:27] {"message":"`async` is a keyword in the 2018 edition","code":{"code":"async_idents","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":1611,"byte_end":1616,"line_start":66,"line_end":66,"column_start":9,"column_end":14,"is_primary":true,"text":[{"text":"    let async: async = async {};","highlight_start":9,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"you can use a raw identifier to stay compatible","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":1611,"byte_end":1616,"line_start":66,"line_end":66,"column_start":9,"column_end":14,"is_primary":true,"text":[{"text":"    let async: async = async {};","highlight_start":9,"highlight_end":14}],"label":null,"suggested_replacement":"r#async","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `async` is a keyword in the 2018 edition\n  --> /checkout/src/test/ui/rust-2018/async-ident.rs:66:9\n   |\nLL |     let async: async = async {};\n   |         ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!\n   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>\n\n"}
[00:49:27] {"message":"`async` is a keyword in the 2018 edition","code":{"code":"async_idents","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/async-ident.rs","byte_start":1618,"byte_end":1623,"line_start":66,"line_end":66,"column_start":16,"column_end":21,"is_primary":true,"text":[{"text":"    let async: async = async {};","highlight_start":16,"highlight_end":21}],"label":null,"sugges],"label":null,"suggested_replacement":"r#async","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `async` is a keyword in the 2018 edition\n  --> /checkout/src/test/ui/rust-2018/async-ident.rs:84:6\n   |\nLL |     (async) => (1)\n   |      ^^^^^ help: you can use a raw identifier to stay compatible: `r#async`\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!\n   = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>\n\n"}
[00:49:27] {"message":"aborting due to 14 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 14 previous errors\n\n"}
[00:49:27] ------------------------------------------
[00:49:27] 
[00:49:27] thread '[ui] ui/rust-2018/async-ident.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:49:27] 
---
[00:49:27] test result: FAILED. 2234 passed; 8 failed; 7 ignored; 0 measured; 0 filtered out
[00:49:27] 
[00:49:27] 
[00:49:27] 
[00:49:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:49:27] 
[00:49:27] 
[00:49:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:49:27] Build completed unsuccessfully in 0:02:19
[00:49:27] Build completed unsuccessfully in 0:02:19
[00:49:27] make: *** [check] Error 1
[00:49:27] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:214562d0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:14f25038
$ sudo tail -n 500 /var/log/syslog
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000]   5 disabled
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000]   6 disabled
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000]   7 disabled
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] e820: last_pfn = 0xbfff3 max_arch_pfn = 0x400000000
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff8800000f2800]
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] Using GB pages for direct mapping
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  9 21:25:16 travis-job-fbffffff]
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] kvm-clock: using sched offset of 1378112321 cycles
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] Zone ranges:
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000fff-0xffffffff]
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] Policy zone: Normal
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  9 21:25:16 travis-job-fb947de0-1623-4Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.307014] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.308228] pid_max: default: 32768 minimum: 301
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.309040] ACPI: Core revision 20150930
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.315204] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.316239] Security Framework initialized
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.317014] Yama: becoming mindful.
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.317684] AppArmor: AppArmor disabled by boot time parameter
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.320340] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.329419] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.333621] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.334679] Mountpoint-cache hash table entries: 32768 (order: 6, 8c-84c3-70fc86366ac7 kernel: [    0.345715] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.349088] Freeing SMP alternatives memory: 32K
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.358672] ftrace: allocating 32185 entries in 126 pages
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.410707] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.411976] smpboot: Max logical packages: 2
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.413395] x2apic enabled
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.415068] Switched APIC routing to physical x2apic.
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.418935] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.526310] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.528141] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.530385] x86: Booting SMP configuration:
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.530997] .... node  #0, CPUs:      #1
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc8636 configuration space under this bridge.
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.667177] PCI host bridge to bus 0000:00
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.667759] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.668701] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.669809] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.671409] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.672672] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.673657] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.674060] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.688248] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.701447] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.702972] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.707699] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.711868] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.722441] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.728125] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.731494] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.743065] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.745347] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.747337] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.750765] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.753007] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.772624] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.774031] vgaarb: loaded
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.774650] SCSI subsystem initialized
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.775353] libata version 3.00 loaded.
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.775395] ACPI: bus type USB registered
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.776011] usbcore: registered new interface driver usbfs
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.776921] usbcore: registered new interface driver hub
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.777722] usbcore: registered new device driver usb
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.778681] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.779657] dmi: Firmware registration failed.
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.780477] PCI: Using ACPI for IRQ routing
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.781120] PCI: pci_cache_line_size set to 64 bytes
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.781217] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.781218] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.781353] NetLabel: Initializing
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.781897] NetLabel:  domain hash size = 128
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.782597] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.783286] NetLabel:  unlabeled traffic allowed by default
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.784250] amd_nb: Cannot enumerate AMD northbridges
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.785076] clocksource: Switched to clocksource kvm-clock
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.792785] pnp: PnP ACPI init
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.793640] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.793716] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.793761] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.793810] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.793851] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    0.793898] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel:c7 kernel: [    3.052275] Linux agpgart interface v0.103
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    3.057483] loop: module loaded
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    3.059217] libphy: Fixed MDIO Bus: probed
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    3.061278] tun: Universal TUN/TAP device driver, 1.6
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    3.063997] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    3.106660] PPP generic driver version 2.4.2
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    3.108944] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    3.112021] ehci-pci: EHCI PCI platform driver
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    3.114127] ehci-platform: EHCI generic platform driver
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    3.116680] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    3.120055] ohci-pci: OHCI PCI platform driver
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    3.122199] ohci-platform: OHCI generic platform driver
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    3.124676] uhci_hcd: USB Universal Host Controller Interface driver
Aug  9 21:25:16 travis-job-fbc3-70fc86366ac7 kernel: [    3.164614] NET: Registered protocol family 10
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    3.167750] NET: Registered protocol family 17
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    3.170052] Key type dns_resolver registered
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    3.172870] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    3.175648] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    3.178521] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    3.181460] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    3.184221] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    3.190156] registered taskstats version 1
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    3.192283] Loading compiled-in X.509 certificates
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    3.195366] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    3.200224] zswap: loaded using pool lzo/zbud
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc    7.869130] raid6: avx2x1   gen() 16780 MB/s
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    7.937127] raid6: avx2x2   gen() 19666 MB/s
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    8.005134] raid6: avx2x4   gen() 21820 MB/s
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    8.007521] raid6: using algorithm avx2x4 gen() 21820 MB/s
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    8.010167] raid6: using avx2x2 recovery algorithm
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    8.014717] xor: automatically using best checksumming function:
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    8.057124]    avx       : 26864.000 MB/sec
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    8.073882] Btrfs loaded
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    8.139228] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    8.143779] EXT4-fs (sda1): write access will be enabled during recovery
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    8.224312] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    8.235352] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    8.238183] EXT4-fs (sda1): recovery complete
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    8.246194] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    8.510228] random: init: uninitialized urandom read (12 bytes read, 23 bits of entropy available)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    8.650431] random: mountall: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    8.710121] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    8.945357] random: cloud-init: uninitialized urandom read (32 bytes read, 32 bits of entropy available)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    9.561263] random: cloud-init: uninitialized urandom read (32 bytes read, 40 bits of entropy available)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    9.706374] systemd-udevd[701]: starting version 204
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    9.836187] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    9.883397] intel_rapl: no valid rapl domains found in package 0
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [    9.938547] ppdev: user-space parallel port driver
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [   10.053680] random: mktemp: uninitialized urandom read (6 bytes read, 51 bits of entropy available)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [   10.111454] random: mktemp: uninitialized urandom read (6 bytes read, 51 bits of entropy available)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [   10.174744] random: cloud-init: uninitialized urandom read (32 bytes read, 52 bits of entropy available)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [   10.339757] random: cloud-init: uninitialized urandom read (32 bytes read, 52 bits of entropy available)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [   10.609226] random: mktemp: uninitialized urandom read (12 bytes read, 55 bits of entropy available)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [   10.696213] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [   10.781154] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [   10.833763] EXT4-fs (sda1): resized filesystem to 7864064
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [   11.272489] init: failsafe main process (1092) killed by TERM signal
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 instance-setup: INFO Running set_multiqueue.
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 instance-setup: INFO Set channels for eth0 to 4.
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  9 21:25:16 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 instance-setup: INFO Setting /PRINTS-----
Aug  9 21:25:23 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 ec2: 1024 93:1b:23:31:61:55:94:32:a4:d4:bd:1f:23:07:c5:bc  root@travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 (DSA)
Aug  9 21:25:23 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 ec2: 256 0b:15:a9:5b:dd:ea:7a:c5:b1:c1:02:d9:b9:33:5b:63  root@travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 (ECDSA)
Aug  9 21:25:23 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 ec2: 256 84:06:ca:8a:31:f9:e3:63:c6:2d:ec:34:cb:fb:bd:f7  root@travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 (ED25519)
Aug  9 21:25:23 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 ec2: 2048 c2:4e:a5:71:91:3f:28:fa:fe:54:90:31:84:be:13:f9  root@travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 (RSA)
Aug  9 21:25:23 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  9 21:25:23 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 ec2: #############################################################
Aug  9 21:25:33 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 ntpdate[2168]: the NTP socket is in use, exiting
Aug  9 21:26:11 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [   66.214503] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  9 21:27:10 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [  125.666547] device vethfa6017e entered promiscuous mode
Aug  9 21:27:10 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [  125.666617] docker0: port 1(vethfa6017e) entered forwarding state
Aug  9 21:27:10 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [  125.666625] docker0: port 1(vethfa6017e) entered forwardi14 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 ntpd[1764]: peers refreshed
Aug  9 21:27:14 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 ntpd[1764]: new interface(s) found: waking up resolver
Aug  9 21:27:25 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [  140.894111] docker0: port 1(vethfa6017e) entered forwarding state
Aug  9 22:15:39 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [ 3034.339982] docker0: port 1(vethfa6017e) entered disabled state
Aug  9 22:15:39 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [ 3034.340046] veth09398cd: renamed from eth0
Aug  9 22:15:39 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [ 3034.430608] docker0: port 1(vethfa6017e) entered disabled state
Aug  9 22:15:39 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [ 3034.432366] device vethfa6017e left promiscuous mode
Aug  9 22:15:39 travis-job-fb947de0-1623-438c-84c3-70fc86366ac7 kernel: [ 3034.432369] docker0: port 1(vethfa6017e) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:0e8082ef

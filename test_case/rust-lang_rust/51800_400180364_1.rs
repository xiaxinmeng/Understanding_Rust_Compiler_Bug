\n\nIf you don't know the basics of Rust, you can go look to the Rust Book to get\nstarted: https://doc.rust-lang.org/book/\n"},"level":"error","spans":[],"children":[{"message":"consider adding a `main` function to `/checkout/src/test/ui/edition-keywords-2015-2018-expansion.rs`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0601]: `main` function not found in crate `edition_keywords_2015_2018_expansion`\n   |\n   = note: consider adding a `main` function to `/checkout/src/test/ui/edition-keywords-2015-2018-expansion.rs`\n\n"}
[00:47:46] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:47:46] {"message":"For more information about this error, try `rustc --explain E0601`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0601`.\n"}
[00:47:46] ------------------------------------------
[00:47:46] 
[00:47:46] thread '[ui] ui/edition-keywords-2015-2018-expansion.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3154:9
[00:47:46] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:46] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:46] 
[00:47:46] ---- [ui] ui/edition-keywords-2018-2015-parsing.rs stdout ----
[00:47:46] diff of stderr:
[00:47:46] 
[00:47:46] - error: expected identifier, found reserved keyword `async`
[00:47:46] -   --> $DIR/edition-keywords-2018-2015-parsing.rs:20:13
[00:47:46] -    |
[00:47:46] - LL |     let mut async = 1; //~ ERROR expected identifier, found reserved keyword `async`
[00:47:46] -    |             ^^^^^ expected identifier, found reserved keyword
[00:47:46] - 
[00:47:46] - error: expected identifier, found reserved keyword `async`
[00:47:46] -   --> $DIR/edition-keywords-2018-2015-parsing.rs:30:13
[00:47:46] -    |
[00:47:46] - LL |     module::async(); //~ ERROR expected identifier, found reserved keyword `async`
[00:47:46] -    |             ^^^^^ expected identifier, found reserved keyword
[00:47:46] - 
[00:47:46] 13 error: no rules expected the token `r#async`
[00:47:46] 14   --> $DIR/edition-keywords-2018-2015-parsing.rs:24:31
[00:47:46] 
[00:47:46] 
[00:47:46] 22 LL |     r#async = consumes_async_raw!(async); //~ ERROR no rules expected the token `async`
[00:47:46] 24 
[00:47:46] 24 
[00:47:46] - error: expected one of `move`, `|`, or `||`, found `<eof>`
[00:47:46] -   --> <passes_ident macros>:1:22
[00:47:46] -    |
[00:47:46] - LL | ( $ i : ident ) => ( $ i )
[00:47:46] -    |                      ^^^ expected one of `move`, `|`, or `||` here
[00:47:46] - error: aborting due to 5 previous errors
[00:47:46] + error: aborting due to 2 previous errors
[00:47:46] 32 
[00:47:46] 33 
[00:47:46] 33 
[00:47:46] 
[00:47:46] 
[00:47:46] The actual stderr differed from the expected stderr.
[00:47:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/edition-keywords-2018-2015-parsing/edition-keywords-2018-2015-parsing.stderr
[00:47:46] To update references, rerun the tests and pass the `--bless` flag
[00:47:46] To only update this specific test, also pass `--test-args edition-keywords-2018-2015-parsing.rs`
[00:47:46] error: 1 errors occurred comparing output.
[00:47:46] status: exit code: 101
[00:47:46] status: exit code: 101
[00:47:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/edition-keywords-2018-2015-parsing.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/edition-keywords-2018-2015-parsing/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/edition-keywords-2018-2015-parsing/auxiliary" "-A" "unused"
[00:47:46] ------------------------------------------
[00:47:46] 
[00:47:46] ------------------------------------------
[00:47:46] stderr:
[00:47:46] stderr:
[00:47:46] ------------------------------------------
[00:47:46] {"message":"no rules expected the token `r#async`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/edition-keywords-2018-2015-parsing.rs","byte_start":816,"byte_end":823,"line_start":24,"line_end":24,"column_start":31,"column_end":38,"is_primary":true,"text":[{"text":"    r#async = consumes_async!(r#async); //~ ERROR no rules expected the token `r#async`","highlight_start":31,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no rules expected the token `r#async`\n  --> /checkout/src/test/ui/edition-keywords-2018-2015-parsing.rs:24:31\n   |\nLL |     r#async = consumes_async!(r#async); //~ ERROR no rules expected the token `r#async`\n   |                               ^^^^^^^\n\n"}
[00:47:46] {"message":"no rules expected the token `async`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/edition-keywords-2018-2015-parsing.rs","byte_start":908,"byte_end":913,"line_start":25,"line_end":25,"column_start":35,"column_end":40,"is_primary":true,"text":[{"text":"    r#async = consumes_async_raw!(async); //~ ERROR no rules expected the token `async`","highlight_start":35,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no rules expected the token `async`\n  --> /checkout/src/test/ui/edition-keywords-2018-2015-parsing.rs:25:35\n   |\nLL |     r#async = consumes_async_raw!(async); //~ ERROR no rules expected the token `async`\n   |                                   ^^^^^\n\n"}
[00:47:46] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:47:46] ------------------------------------------
[00:47:46] 
[00:47:46] thread '[ui] ui/edition-keywords-2018-2015-parsing.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3154:9
[00:47:46] 
[00:47:46] 
[00:47:46] ---- [ui] ui/edition-keywords-2018-2018-expansion.rs stdout ----
[00:47:46] diff of stderr:
[00:47:46] 
[00:47:46] - error: expected identifier, found reserved keyword `async`
[00:47:46] -   --> $DIR/edition-keywords-2018-2018-expansion.rs:20:5
[00:47:46] + error[E0601]: `main` function not found in crate `edition_keywords_2018_2018_expansion`
[00:47:46] 3    |
[00:47:46] - LL |     produces_async! {} // ERROR expected identifier, found reserved keyword `async`
[00:47:46] -    |     ^^^^^^^^^^^^^^^^^^ expected identifier, found reserved keyword
[00:47:46] -    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:47:46] -    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:47:46] +    = note: consider adding a `main` function to `$DIR/edition-keywords-2018-2018-expansion.rs`
[00:47:46] 9 error: aborting due to previous error
[00:47:46] 10 
[00:47:46] 
[00:47:46] + For more information about this error, try `rustc --explain E0601`.
[00:47:46] + For more information about this error, try `rustc --explain E0601`.
[00:47:46] 11 
[00:47:46] 
[00:47:46] 
[00:47:46] The actual stderr differed from the expected stderr.
[00:47:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/edition-keywords-2018-2018-expansion/edition-keywords-2018-2018-expansion.stderr
[00:47:46] To update references, rerun the tests and pass the `--bless` flag
[00:47:46] To only update this specific test, also pass `--test-args edition-keywords-2018-2018-expansion.rs`
[00:47:46] error: 1 errors occurred comparing output.
[00:47:46] status: exit code: 101
[00:47:46] status: exit code: 101
[00:47:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/edition-keywords-2018-2018-expansion.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/edition-keywords-2018-2018-expansion/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/edition-keywords-2018-2018-expansion/auxiliary" "-A" "unused"
[panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3154:9
[00:47:46] 
[00:47:46] ---- [ui] ui/edition-keywords-2018-2018-parsing.rs stdout ----
[00:47:46] diff of stderr:
[00:47:46] diff of stderr:
[00:47:46] 
[00:47:46] - error: expected identifier, found reserved keyword `async`
[00:47:46] -   --> $DIR/edition-keywords-2018-2018-parsing.rs:20:13
[00:47:46] -    |
[00:47:46] - LL |     let mut async = 1; //~ ERROR expected identifier, found reserved keyword `async`
[00:47:46] -    |             ^^^^^ expected identifier, found reserved keyword
[00:47:46] - 
[00:47:46] - error: expected identifier, found reserved keyword `async`
[00:47:46] -   --> $DIR/edition-keywords-2018-2018-parsing.rs:30:13
[00:47:46] -    |
[00:47:46] - LL |     module::async(); //~ ERROR expected identifier, found reserved keyword `async`
[00:47:46] -    |             ^^^^^ expected identifier, found reserved keyword
[00:47:46] - 
[00:47:46] 13 error: no rules expected the token `r#async`
[00:47:46] 14   --> $DIR/edition-keywords-2018-2018-parsing.rs:24:31
[00:47:46] 
[00:47:46] 
[00:47:46] 22 LL |     r#async = consumes_async_raw!(async); //~ ERROR no rules expected the token `async`
[00:47:46] 24 
[00:47:46] 24 
[00:47:46] - error: expected one of `move`, `|`, or `||`, found `<eof>`
[00:47:46] -   --> <passes_ident macros>:1:22
[00:47:46] -    |
[00:47:46] - LL | ( $ i : ident ) => ( $ i )
[00:47:46] -    |                      ^^^ expected one of `move`, `|`, or `||` here
[00:47:46] - error: aborting due to 5 previous errors
[00:47:46] + error: aborting due to 2 previous errors
[00:47:46] 32 
[00:47:46] 33 
[00:47:46] 33 
[00:47:46] 
[00:47:46] 
[00:47:46] The actual stderr differed from the expected stderr.
[00:47:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/edition-keywords-2018-2018-parsing/edition-keywords-2018-2018-parsing.stderr
[00:47:46] To update references, rerun the tests and pass the `--bless` flag
[00:47:46] To only update this specific test, also pass `--test-args edition-keywords-2018-2018-parsing.rs`
[00:47:46] error: 1 errors occurred comparing output.
[00:47:46] status: exit code: 101
[00:47:46] status: exit code: 101
[00:47:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/edition-keywords-2018-2018-parsing.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/edition-keywords-2018-2018-parsing/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/edition-keywords-2018-2018-parsing/auxiliary" "-A" "unused"
[00:47:46] ------------------------------------------
[00:47:46] 
[00:47:46] ------------------------------------------
[00:47:46] stderr:
[00:47:46] stderr:
[00:47:46] ------------------------------------------
[00:47:46] {"message":"no rules expected the token `r#async`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/edition-keywords-2018-2018-parsing.rs","byte_start":816,"byte_end":823,"line_start":24,"line_end":24,"column_start":31,"column_end":38,"is_primary":true,"text":[{"text":"    r#async = consumes_async!(r#async); //~ ERROR no rules expected the token `r#async`","highlight_start":31,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no rules expected the token `r#async`\n  --> /checkout/src/test/ui/edition-keywords-2018-2018-parsing.rs:24:31\n   |\nLL |     r#async = consumes_async!(r#async); //~ ERROR no rules expected the token `r#async`\n   |                               ^^^^^^^\n\n"}
[00:47:46] {"message":"no rules expected the token `async`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/edition-keywords-2018-2018-parsing.rs","byte_start":908,"byte_end":913,"line_start":25,"line_end":25,"column_start":35,"column_end":40,"is_primary":true,"text":[{"text":"    r#async = consumes_async_raw!(async); //~ ERROR no rules expected the token `async`","highlight_start":35,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: no rules expected the token `async`\n  --> /checkout/src/test/ui/edition-keywords-2018-2018-parsing.rs:25:35\n   |\nLL |     r#async = consumes_async_raw!(async); //~ ERROR no rules expected the token `async`\n   |                                   ^^^^^\n\n"}
[00:47:46] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:47:46] ------------------------------------------
[00:47:46] 
[00:47:46] 
[00:47:4ld/x86_64-unknown-linux-gnu/test/ui/feature-gate-async-await/auxiliary" "-A" "unused"
[00:47:46] ------------------------------------------
[00:47:46] 
[00:47:46] ------------------------------------------
[00:47:46] stderr:
[00:47:46] stderr:
[00:47:46] ------------------------------------------
[00:47:46] {"message":"cannot find struct, variant or union type `async` in this scope","code":{"code":"E0422","explanation":"\nYou are trying to use an identifier that is either undefined or not a struct.\nErroneous code example:\n\n
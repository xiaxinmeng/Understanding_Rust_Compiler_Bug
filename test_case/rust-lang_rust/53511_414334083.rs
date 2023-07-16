plain
[00:46:03] ....................................................................................................
[00:46:05] ...........................................i........................................................
[00:46:08] ....................................................................................................
[00:46:12] ....................................................................................................
[00:46:14] ..........................................................................................F.........
[00:46:21] ....................................................................................................
[00:46:24] ....................................................................................................
[00:46:27] ....................................................................................................
[00:46:30] ....................................................................................................
---
[00:46:52] ....................................................................................................
[00:46:55] ....................................................................................................
[00:46:58] .............i......................................................................................
[00:47:01] ....................................................................................................
[00:47:04] ........F...........................................................................................
[00:47:12] ....................................................................................................
[00:47:12] ....................................................................................................
[00:47:14] .....................F..............................................................................
32995.rs:26:34
[00:47:18] +   --> $DIR/issue-32995.rs:26:36
[00:47:18] 31    |
[00:47:18] 31    |
[00:47:18] 32 LL |     let p = ::std::str::from_utf8::()(b"foo").unwrap();
[00:47:18] +    |                                    ^^
[00:47:18] 34    |
[00:47:18] 35    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:47:18] 36    = note: for more information, see issue #42238 <https://github.com/rust-lang/rust/issues/42238>
[00:47:18] 36    = note: for more information, see issue #42238 <https://github.com/rust-lang/rust/issues/42238>
[00:47:18] 
[00:47:18] 
[00:47:18] The actual stderr differed from the expected stderr.
[00:47:18] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32995/issue-32995.stderr
[00:47:18] To update references, rerun the tests and pass the `--bless` flag
[00:47:18] To only update this specific test, also pass `--test-args issues/issue-32995.rs`
[00:47:18] error: 1 errors occurred comparing output.
[00:47:18] status: exit code: 1
[00:47:18] status: exit code: 1
[00:47:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-32995.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32995/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32995/auxiliary" "-A" "unused"
[00:47:18] ------------------------------------------
[00:47:18] ------------------------------------------
[00:47sue #42238 <https://github.com/rust-lang/rust/issues/42238>\n\n"}
[00:47:18] {"message":"parenthesized parameters may only be used with a trait","code":{"code":"parenthesized_params_in_types_and_modules","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-32995.rs","byte_start":650,"byte_end":652,"line_start":18,"line_end":18,"column_start":24,"column_end":26,"is_primary":true,"text":[{"text":"    let b: ::std::boxed()::Box<_> = Box::new(1);","highlight_start":24,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #42238 <https://github.com/rust-lang/rust/issues/42238>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: parenthesized parameters may only be used with a trait\n  --> /checkout/src/test/ui/issues/issue-32995.rs:18:24\n   |\nLL |     let b: ::std::boxed()::Box<_> = Box::new(1);\n   |                        ^^\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #42238 <https://github.com/rust-lang/rust/issues/42238>\n\n"}
[00:47:18] {"message":"parenthesized parameters may only be used with a trait","code":{"code":"parenthesized_params_in_types_and_modules","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-32995.rs","byte_start":805,"byte_end":807,"line_start":22,"line_end":22,"column_start":25,"column_end":27,"is_primary":true,"text":[{"text":"    let p = ::std::str::()::from_utf8(b\"foo\").unwrap();","highlight_start":25,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #42238 <https://github.com/rust-lang/rust/issues/42238>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: parenthesized parameters may only be used with a trait\n  --> /checkout/src/test/ui/issues/issue-32995.rs:22:25\n   |\nLL |     let p = ::std::str::()::from_utf8(b\"foo\").unwrap();\n   |                         ^^\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #42238 <https://github.com/rust-lang/rust/issues/42238>\n\n"}
[00:47:18] {"message":"parenthesized parameters may only be used with a trait","code":{"code":"parenthesized_params_in_types_and_modules","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-32995.rs","byte_start":977,"byte_end":979,"line_start":26,"line_end":26,"column_start":36,"column_end":38,"is_primary":true,"text":[{"text":"    let p = ::std::str::from_utf8::()(b\"foo\").unwrap();","highlight_start":36,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #42238 <https://github.com/rust-lang/rust/issues/42238>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: parenthesized parameters may only be used with a trait\n  --> /checkout/src/test/ui/issues/issue-32995.rs:26:36\n   |\nLL |     let p = ::std::str::from_utf8::()(b\"foo\").unwrap();\n   |                                    ^^\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #42238 <https://github.com/rust-lang/rust/issues/42238>\n\n"}
[00:47:18] {"message":"parenthesized parameters may only be used with a trait","code":{"code":"parenthesized_params_in_types_and_modules","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-32995.rs","byte_start":1132,"byte_end":1134,"line_start":30,"line_end":30,"column_start":30,"column_end":32,"is_primary":true,"text":[{"text":"    let o : Box<::std::marker()::Send> = Box::new(1);","highlight_start":30,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #42238 <https://github.com/rust-lang/rust/issues/42238>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: parenthesized parameters may only be used with a trait\n  --> /checkout/src/test/ui/issues/issue-32995.rs:30:30\n   |\nLL |     let o : Box<::std::marker()::Send> = Box::new(1);\n   |                              ^^\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #42238 <https://github.com/rust-lang/rust/issues/42238>\n\n"}
[00:47:18] {"message":"parenthesized parameters may only be used with a trait","code":{"code":"parenthesized_params_in_types_and_modules","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-32995.rs","byte_start":1298,"byte_end":1300,"line_start":34,"line_end":34,"column_start":37,"column_end":39,"is_primary":true,"text":[{"text":"    let o : Box<Send + ::std::marker()::Sync> = Box::new(1);","highlight_start":37,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #42238 <https://github.com/rust-lang/rust/issues/42238>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: parenthesized parameters may only be used with a trait\n  --> /checkout/src/test/ui/issues/issue-32995.rs:34:37\n   |\nLL |     let o : Box<Send + ::std::marker()::Sync> = Box::new(1);\n   |                                     ^^\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #42238 <https://github.com/rust-lang/rust/issues/42238>\n\n"}
[00:47:18] {"message":"parenthesized parameters may only be used with a trait","code":{"code":"parenthesized_params_in_types_and_modules","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-32995.rs","byte_start":1465,"byte_end":1467,"line_start":40,"line_end":40,"column_start":14,"column_end":16,"is_primary":true,"text":[{"text":"    let d : X() = Default::default();","highlight_start":14,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #42238 <https://github.com/rust-lang/rust/issues/42238>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: parenthesized parameters may only be used with a trait\n  --> /checkout/src/test/ui/issues/issue-32995.rs:40:14\n   |\nLL |     let d : X() = Default::default();\n   |              ^^\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #42238 <https://github.com/rust-lang/rust/issues/42238>\n\n"}
[00:47:18] {"message":"aborting due to 7 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 7 previous errors\n\n"}
[00:47:18] ------------------------------------------
[00:47:18] 
[00:47:18] thread '[ui] ui/issues/issue-32995.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3189:9
[00:47:18] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:18] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:18] 
[00:47:18] ---- [ui] ui/span/macro-ty-params.rs stdout ----
[00:47:18] diff of stderr:
[00:47:18] 
[00:47:18] 5    |        ^^^^^^^^^
[00:47:18] 6 
[00:47:18] 7 error: generic arguments in macro path
[00:47:18] +   --> $DIR/macro-ty-params.rs:18:10
[00:47:18] 9    |
[00:47:18] 9    |
[00:47:18] 10 LL |     foo::<T>!(); //~ ERROR generic arguments in macro path
[00:47:18] +    |          ^^^
[00:47:18] 12 
[00:47:18] 12 
[00:47:18] 13 error: generic arguments in macro path
[00:47:18] +   --> $DIR/macro-ty-params.rs:19:10
[00:47:18] 15    |
[00:47:18] 15    |
[00:47:18] 16 LL |     foo::<>!(); //~ ERROR generic arguments in macro path
[00:47:18] +    |          ^^
[00:47:18] 18 
[00:47:18] 18 
[00:47:18] 19 error: generic arguments in macro path
[00:47:18] 
[00:47:18] 
[00:47:18] The actual stderr differed from the expected stderr.
[00:47:18] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/macro-ty-params/macro-ty-params.stderr
[00:47:18] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/macro-ty-params/macro-ty-params.stderr
[00:47:18] To update references, rerun the tests and pass the `--bless` flag
[00:47:18] To only update this specific test, also pass `--test-args span/macro-ty-params.rs`
[00:47:18] error: 1 errors occurred comparing output.
[00:47:18] status: exit code: 1
[00:47:18] status: exit code: 1
[00:47:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/macro-ty-params.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/macro-ty-params/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/macro-ty-params/auxiliary" "-A" "unused"
[00:47:18] ------------------------------------------
[00:47:18] 
[00:47:18] ------------------------------------------
[00:47:18] stderr:
[00:47:18] stderr:
[00:47:18] ------------------------------------------
[00:47:18] {"message":"unexpected generic arguments in path","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/span/macro-ty-params.rs","byte_start":683,"byte_end":692,"line_start":20,"line_end":20,"column_start":8,"column_end":17,"is_primary":true,"text":[{"text":"    m!(MyTrait<>); //~ ERROull,"expansion":null}],"children":[],"rendered":"error: generic arguments in macro path\n  --> /checkout/src/test/ui/span/macro-ty-params.rs:19:10\n   |\nLL |     foo::<>!(); //~ ERROR generic arguments in macro path\n   |          ^^\n\n"}
[00:47:18] {"message":"generic arguments in macro path","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/span/macro-ty-params.rs","byte_start":690,"byte_end":692,"line_start":20,"line_end":20,"column_start":15,"column_end":17,"is_primary":true,"text":[{"text":"    m!(MyTrait<>); //~ ERROR generic arguments in macro path","highlight_start":15,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: generic arguments in macro path\n  --> /checkout/src/test/ui/span/macro-ty-params.rs:20:15\n   |\nLL |     m!(MyTrait<>); //~ ERROR generic arguments in macro path\n   |               ^^\n\n"}
[00:47:18] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:47:18] ------------------------------------------
[00:47:18] 
[00:47:18] thread '[ui] ui/span/macro-ty-params.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3189:9
[00:47:18] 
[00:47:18] 
[00:47:18] ---- [ui] ui/unboxed-closures/unboxed-closure-sugar-used-on-struct-3.rs stdout ----
[00:47:18] diff of stderr:
[00:47:18] 
[00:47:18] 1 error[E0214]: parenthesized parameters may only be used with a trait
[00:47:18] -   --> $DIR/unboxed-closure-sugar-used-on-struct-3.rs:24:16
[00:47:18] +   --> $DIR/unboxed-closure-sugar-used-on-struct-3.rs:24:18
[00:47:18] 3    |
[00:47:18] 4 LL |     let b = Bar::(isize, usize)::new(); // OK too (for the parser)
[00:47:18] -    |                ^^^^^^^^^^^^^^^^ only traits may use parentheses
[00:47:18] +    |                  ^^^^^^^^^^^^^^ only traits may use parentheses
[00:47:18] 7 error: aborting due to previous error
[00:47:18] 8 
[00:47:18] 
[00:47:18] 
[00:47:18] 
[00:47:18] The actual stderr differed from the expected stderr.
[00:47:18] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closure-sugar-used-on-struct-3/unboxed-closure-sugar-used-on-struct-3.stderr
[00:47:18] To update references, rerun the tests and pass the `--bless` flag
[00:47:18] To only update this specific test, also pass `--test-args unboxed-closures/unboxed-closure-sugar-used-on-struct-3.rs`
[00:47:18] error: 1 errors occurred comparing output.
[00:47:18] status: exit code: 1
[00:47:18] status: exit code: 1
[00:47:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unboxed-closures/unboxed-closure-sugar-used-on-struct-3.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closure-sugar-used-on-struct-3/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closure-sugar-used-on-struct-3/auxiliary" "-A" "unused"
[00:47:18] ------------------------------------------
[00:47:18] 
[00:47:18] ------------------------------------------
[00:47:18] stderr:
[00:47:18] stderr:
[00:47:18] ------------------------------------------
[00:47:18] {"message":"parenthesized parameters may only be used with a trait","code":{"code":"E0214","explanation":"\nA generic type was described using parentheses rather than angle brackets.\nFor example:\n\n
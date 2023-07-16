plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:44:24] 
[00:44:24] running 2118 tests
[00:44:28] .......................................................................................F..........i.
[00:44:32] ................................................................i..F................................
[00:44:35] .............................F...F..................................................................
[00:44:39] ....................................................................................................
[00:44:41] ....................................................................................................
[00:44:41] ....................................................................................................
[00:44:44] ............................F.....................................FF................................
[00:44:48] ........................................................................................F...........
[00:44:50] ......................................Fi............................................................
[00:44:55] ....................................................................................................
[00:44:55] ....................................................................................................
[00:44:58] .............................................................F......................................
[00:45:01] ....................................................................................................
[00:45:04] .....................................................................................F..............
[00:45:07] ...................................................................................................F
[00:45:14] ..................................................................................i.................
[00:45:14] ..................................................................................i.................
[00:45:17] ........................................F.F.......F.................................................
[00:45:21] ....................................................................................................
[00:45:24] .....................................................F..............................................
[00:45:30] failures:
[00:45:30] 
[00:45:30] ---- [ui] ui/codemap_tests/bad-format-args.rs stdout ----
[00:45:30] diff of stderr:
[00:45:30] diff of stderr:
[00:45:30] 
[00:45:30] 9 error: expected token: `,`
[00:45:30] 10   --> $DIR/bad-format-args.rs:13:5
[00:45:30] 11    |
[00:45:30] - LL |     format!("" 1);
[00:45:30] + LL |     format!("" 1); // ERROR expected token: `,`
[00:45:30] 14    |
[00:45:30] 15    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:45:30] 
[00:45:30] 17 error: expected token: `,`
[00:45:30] 17 error: expected token: `,`
[00:45:30] 18   --> $DIR/bad-format-args.rs:14:5
[00:45:30] 19    |
[00:45:30] - LL |     format!("", 1 1);
[00:45:30] + LL |     format!("", 1 1); //~ ERROR expected token: `,`
[00:45:30] 22    |
[00:45:30] 23    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:45:30] 
[00:45:30] 
[00:45:30] 
[00:45:30] The actual stderr differed from the expected stderr.
[00:45:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/bad-format-args/bad-format-args.stderr
[00:45:30] To update references, rerun the tests and pass the `--bless` flag
[00:45:30] To only update this specific test, also pass `--test-args codem"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/codemap_tests/bad-format-args.rs","byte_start":483,"byte_end":493,"line_start":12,"line_end":12,"column_start":5,"column_end":15,"is_primary":false,"text":[{"text":"    format!();","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"format!","def_site_span":{"file_name":"<format macros>","byte_start":0,"byte_end":90,"line_start":1,"line_end":2,"column_start":1,"column_end":63,"is_primary":false,"text":[{"text":"( $ ( $ arg : tt ) * ) => (","highlight_start":1,"highlight_end":28},{"text":"$ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )","highlight_start":1,"highlight_end":63}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"format_args!","def_site_span":null}}],"children":[],"rendered":"error: requires at least a format string argument\n  --> /checkout/src/test/ui/codemap_tests/bad-format-args.rs:12:5\n   |\nLL |     format!();\n   |     ^^^^^^^^^^\n   |\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[00:45:30] {"message":"expected token: `,`","code":null,"level":"error","spans":[{"file_name":"<format macros>","byte_start":55,"byte_end":86,"line_start":2,"line_end":2,"column_start":28,"column_end":59,"is_primary":true,"text":[{"text":"$ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )","highlight_start":28,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"<format macros>","byte_start":55,"byte_end":86,"line_start":2,"line_end":2,"column_start":28,"column_end":59,"is_primary":false,"text":[{"text":"$ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )","highlight_start":28,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/codemap_tests/bad-format-args.rs","byte_start":498,"byte_end":512,"line_start":13,"line_end":13,"column_start":5,"column_end":19,"is_primary":false,"text":[{"text":"    format!(\"\" 1); // ERROR expected token: `,`","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"format!","def_site_span":{"file_name":"<format macros>","byte_start":0,"byte_end":90,"line_start":1,"line_end":2,"column_start":1,"column_end":63,"is_primary":false,"text":[{"text":"( $ ( $ arg : tt ) * ) => (","highlight_start":1,"highlight_end":28},{"text":"$ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )","highlight_start":1,"highlight_end":63}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"format_args!","def_site_span":null}}],"children":[],"rendered":"error: expected token: `,`\n  --> /checkout/src/test/ui/codemap_tests/bad-format-args.rs:13:5\n   |\nLL |     format!(\"\" 1); // ERROR expected token: `,`\n   |     ^^^^^^^^^^^^^^\n   |\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[00:45:30] {"message":"expected token: `,`","code":null,"level":"error","spans":[{"file_name":"<format macros>","byte_start":55,"byte_end":86,"line_start":2,"line_end":2,"column_start":28,"column_end":59,"is_primary":true,"text":[{"text":"$ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )","highlight_start":28,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"<format macros>","byte_start":55,"byte_end":86,"line_start":2,"line_end":2,"column_start":28,"column_end":59,"is_primary":false,"text":[{"text":"$ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )","highlight_start":28,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/codemap_tests/bad-format-args.rs","byte_start":546,"byte_end":563,"line_start":14,"line_end":14,"column_start":5,"column_end":22,"is_primary":false,"text":[{"text":"    format!(\"\", 1 1); //~ ERROR expected token: `,`","highlight_start":5,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"format!","def_site_span":{"file_name":"<format macros>","byte_start":0,"byte_end":90,"line_start":1,"line_end":2,"column_start":1,"column_end":63,"is_primary":false,"text":[{"text":"( $ ( $ arg : tt ) * ) => (","highlight_start":1,"highlight_end":28},{"text":"$ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )","highlight_start":1,"highlight_end":63}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"format_args!","def_site_span":null}}],"children":[],"rendered":"error: expected token: `,`\n  --> /checkout/src/test/ui/codemap_tests/bad-format-args.rs:14:5\n   |\nLL |     format!(\"\", 1 1); //~ ERROR expected token: `,`\n   |     ^^^^^^^^^^^^^^^^^\n   |\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[00:45:30] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:45:30] ------------------------------------------
[00:45:30] 
[00:45:30] thread '[ui] ui/codemap_tests/bad-format-args.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:45:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:45:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:45:30] 
[00:45:30] ---- [ui] ui/cross-crate-macro-backtrace/main.rs stdout ----
[00:45:30] diff of stderr:
[00:45:30] 
[00:45:30] 1 error: 1 positional argument in format string, but no arguments were given
[00:45:30] +   --> $DIR/main.rs:16:5
[00:45:30] 3    |
[00:45:30] 3    |
[00:45:30] 4 LL |     myprintln!("{}");
[00:45:30] 
[00:45:30] 
[00:45:30] The actual stderr differed from the expected stderr.
[00:45:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cross-crate-macro-backtrace/main/main.stderr
[00:45:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cross-crate-macro-backtrace/main/main.stderr
[00:45:30] To update references, rerun the tests and n_start":18,"column_end":23,"is_primary":true,"text":[{"text":"(  ) => ( pub fn async (  ) {  } )","highlight_start":18,"highlight_end":23}],"label":"expected identifier, found reserved keyword","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/edition-keywords-2015-2018-expansion.rs","byte_start":622,"byte_end":640,"line_start":20,"line_end":20,"column_start":5,"column_end":23,"is_primary":false,"text":[{"text":"    produces_async! {} //~ ERROR expected identifier, found reserved keyword","highlight_start":5,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"produces_async!","def_site_span":{"file_name":"<produces_async macros>","byte_start":0,"byte_end":34,"line_start":1,"line_end":1,"column_start":1,"column_end":35,"is_primary":false,"text":[{"text":"(  ) => ( pub fn async (  ) {  } )","highlight_start":1,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: expected identifier, found reserved keyword `async`\n  --> /checkout/src/test/ui/edition-keywords-2015-2018-expansion.rs:20:5\n   |\nLL |     produces_async! {} //~ ERROR expected identifier, found reserved keyword\n   |     ^^^^^^^^^^^^^^^^^^ expected identifier, found reserved keyword\n   |\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[00:45:30] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:45:30] ------------------------------------------
[00:45:30] 
[00:45:30] thread '[ui] ui/edition-keywords-2015-2018-expansion.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:45:30] 
[00:45:30] 
[00:45:30] ---- [ui] ui/edition-keywords-2018-2018-expansion.rs stdout ----
[00:45:30] diff of stderr:
[00:45:30] 
[00:45:30] 1 error: expected identifier, found reserved keyword `async`
[00:45:30] 2   --> $DIR/edition-keywords-2018-2018-expansion.rs:20:5
[00:45:30] 3    |
[00:45:30] - LL |     produces_async! {} // ERROR expected identifier, found reserved keyword `async`
[00:45:30] + LL |     produces_async! {} //~ ERROR expected identifier, found reserved keyword `async`
[00:45:30] 5    |     ^^^^^^^^^^^^^^^^^^ expected identifier, found reserved keyword
[00:45:30] 7    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:45:30] 
[00:45:30] 
[00:45:30] The actual stderr differed from the expected stderr.
[00:45:30] The actual stderr differed from the expected stderr.
[00:45:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/edition-keywords-2018-2018-expansion/edition-keywords-2018-2018-expansion.stderr
[00:45:30] To update references, rerun the tests and pass the `--bless` flag
[00:45:30] To only update this specific test, also pass `--test-args edition-keywords-2018-2018-expansion.rs`
[00:45:30] error: 1 errors occurred comparing output.
[00:45:30] status: exit code: 101
[00:45:30] status: exit code: 101
[00:45:30] command: "/checkout/obj/build/tion_applicability":null,"expansion":null},"macro_decl_name":"produces_async!","def_site_span":{"file_name":"<produces_async macros>","byte_start":0,"byte_end":34,"line_start":1,"line_end":1,"column_start":1,"column_end":35,"is_primary":false,"text":[{"text":"(  ) => ( pub fn async (  ) {  } )","highlight_start":1,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: expected identifier, found reserved keyword `async`\n  --> /checkout/src/test/ui/edition-keywords-2018-2018-expansion.rs:20:5\n   |\nLL |     produces_async! {} //~ ERROR expected identifier, found reserved keyword `async`\n   |     ^^^^^^^^^^^^^^^^^^ expected identifier, found reserved keyword\n   |\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[00:45:30] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:45:30] ------------------------------------------
[00:45:30] 
[00:45:30] thread '[ui] ui/edition-keywords-2018-2018-expansion.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:45:30] 
[00:45:30] 
[00:45:30] ---- [ui] ui/fmt/format-string-error.rs stdout ----
[00:45:30] diff of stderr:
[00:45:30] 
[00:45:30] 8    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:45:30] 9 
[00:45:30] 10 error: invalid format string: unmatched `}` found
[00:45:30] +   --> $DIR/format-string-error.rs:15:5
[00:45:30] 12    |
[00:45:30] 12    |
[00:45:30] 13 LL |     println!("}");
[00:45:30] 14    |     ^^^^^^^^^^^^^^ unmatched `}` in format string
[00:45:30] 17    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:45:30] 18 
[00:45:30] 18 
[00:45:30] 19 error: invalid format string: invalid argument name `_foo`
[00:45:30] +   --> $DIR/format-string-error.rs:17:23
[00:45:30] 21    |
[00:45:30] 21    |
[00:45:30] 22 LL |     let _ = format!("{_foo}", _foo = 6usize);
[00:45:30] 23    |                       ^^^^ invalid argument name in format string
[00:45:30] 
[00:45:30] 25    = note: argument names cannot start with an underscore
[00:45:30] 27 error: invalid format string: invalid argument name `_`
[00:45:30] -   --> $DIR/format-string-error.rs:17:23
[00:45:30] +   --> $DIR/format-string-error.rs:19:23
[00:45:30] 29    |
[00:45:30] 29    |
[00:45:30] 30 LL |     let _ = format!("{_}", _ = 6usize);
[00:45:30] 31    |                       ^ invalid argument name in format string
[00:45:30] 
[00:45:30] 33    = note: argument names cannot start with an underscore
[00:45:30] 34 
[00:45:30] 35 error: invalid format string: expected `'}'` but string was terminated
[00:45:30] +   --> $DIR/format-string-error.rs:21:23
[00:45:30] 37    |
[00:45:30] 37    |
[00:45:30] 38 LL |     let _ = format!("{");
[00:45:30] 39    |                       ^ expected `'}'` in format string
[00:45:30] 
[00:45:30] 41    = note: if you intended to print `{`, you can escape it using `{{`
[00:45:30] 42 
[00:45:30] 43 error: invalid format string: unmatched `}` found
[00:45:30] +   --> $DIR/format-string-error.rs:23:22
[00:45:30] 45    |
[00:45:30] 45    |
[00:45:30] 46 LL |     let _ = format!("}");
[00:45:30] 47    |                      ^ unmatched `}` in format string
[00:45:30] 
[00:45:30] 49    = note: if you intended to print `}`, you can escape it using `}}`
[00:45:30] 50 
[00:45:30] 51 error: invalid format string: expected `'}'`, found `'/'`
[00:45:30] +   --> $DIR/format-string-error.rs:25:23
[00:45:30] 53    |
[00:45:30] 53    |
[00:45:30] 54 LL |     let _ = format!("{/}");
[00:45:30] 55    |                       ^ expected `}` in format string
[00:45:30] 
[00:45:30] The actual stderr differed from the expected stderr.
[00:45:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-string-error/format-string-error.stderr
[00:45:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-string-error/format-string-error.stderr
[00:45:30] To update references, rerun the tests and pass the `--bless` flag
[00:45:30] To only update this specific test, also pass `--test-args fmt/format-string-error.rs`
[00:45:30] error: 1 errors occurred comparing output.
[00:45:30] status: exit code: 101
[00:45:30] status: exit code: 101
[00:45:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fmt/format-string-error.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-string-error/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-string-error/auxiliary" "-A" "unused"
[00:45:30] ------------------------------------------
[00:45:30] 
[00:45:30] ------------------------------------------
[00:45:30] stderr:
[00:45:30] stderr:
[00:45:30] ------------------------------------------
[00:45:30] {"message":"invalid format string: expected `'}'` but string was terminated","code":null,"level":"error","spans":[{"file_name":"<println macros>","byte_start":66,"byte_end":66,"line_start":2,"line_end":2,"column_start":14,"column_end":14,"is_primary":true,"text":[{"text":"print ! ( concat ! ( $ fmt , \"\\n\" ) ) ) ; ( $ fmt : expr , $ ( $ arg : tt ) *","highlight_start":14,"highlight_end":14}],"label":"expected `'}'` in format string","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"<print macros>","byte_start":54,"byte_end":85,"line_start":2,"line_end":2,"column_start":27,"column_end":58,"is_primary":false,"text":[{"text":"$ crate :: io :: _print ( format_args ! ( $ ( $ arg ) * ) ) ) ;","highlight_start":27,"highlight_end":58}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"<println macros>","byte_start":53,"byte_end":90,"line_start":2,"line_end":2,"column_start":1,"column_end":38,"is_primary":false,"text":[{"text":"print ! ( concat ! ( $ fmt , \"\\n\" ) ) ) ; ( $ fmt : expr , $ ( $ arg : tt ) *","highlight_start":1,"hig"message":"if you intended to print `{`, you can escape it using `{{`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: invalid format string: expected `'}'` but string was terminated\n  --> /checkout/src/test/ui/fmt/format-string-error.rs:12:5\n   |\nLL |     println!(\"{\");\n   |     ^^^^^^^^^^^^^^ expected `'}'` in format string\n   |\n   = note: if you intended to print `{`, you can escape it using `{{`\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[00:45:30] {"message":"invalid format string: unmatched `}` found","code":null,"level":"error","spans":[{"file_name":"<println macros>","byte_start":64,"byte_end":64,"line_start":2,"line_end":2,"column_start":12,"column_end":12,"is_primary":true,"text":[{"text":"print ! ( concat ! ( $ fmt , \"\\n\" ) ) ) ; ( $ fmt : expr , $ ( $ arg : tt ) *","highlight_start":12,"highlight_end":12}],"label":"unmatched `}` in format string","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"<print macros>","byte_start":54,"byte_end":85,"line_start":2,"line_end":2,"column_start":27,"column_end":58,"is_primary":false,"text":[{"text":"$ crate :: io :: _print ( format_args ! ( $ ( $ arg ) * ) ) ) ;","highlight_start":27,"highlight_end":58}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"<println macros>","byte_start":53,"byte_end":90,"line_start":2,"line_end":2,"column_start":1,"column_end":38,"is_primary":false,"text":[{"text":"print ! ( concat ! ( $ fmt , \"\\n\" ) ) ) ; ( $ fmt : expr , $ ( $ arg : tt ) *","highlight_start":1,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/fmt/format-string-error.rs","byte_start":605,"byte_end":619,"line_start":15,"line_end":15,"column_start":5,"column_end":19,"is_primary":false,"text":[{"text":"    println!(\"}\");","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"println!","def_site_span":{"file_name":"<println macros>","byte_start":0,"byte_end":195,"line_start":1,"line_end":3,"column_start":1,"column_end":65,"is_primary":false,"text":[{"text":"(  ) => ( print ! ( \"\\n\" ) ) ; ( $ fmt : expr ) => (","highlight_start":1,"highlight_end":53},{"text":"print ! ( concat ! ( $ fmt , \"\\n\" ) ) ) ; ( $ fmt : expr , $ ( $ arg : tt ) *","highlight_start":1,"highlight_end":78},{"text":") => ( print ! ( concat ! ( $ fmt , \"\\n\" ) , $ ( $ arg ) * ) ) ;","highlight_start":1,"highlight_end":65}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"print!","def_site_span":{"file_name":"<print macros>","byte_start":0,"byte_end":91,"line_start":1,"line_end":2,"column_start":1,"column_end":64,"is_primary":false,"text":[{"text":"( $ ( $ arg : tt ) * ) => (","highlight_start":1,"highlight_end":28},{"text":"$ crate :: io :: _print ( format_args ! ( $ ( $ arg ) * ) ) ) ;","highlight_start":1,"highlight_end":64}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"format_args!","def_site_span":null}}],"children":[{"message":"if you intended to print `}`, you can escape it using `}}`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: invalid format string: unmatched `}` found\n  --> /checkout/src/test/ui/fmt/format-string-error.rs:15:5\n   |\nLL |     println!(\"}\");\n   |     ^^^^^^^^^^^^^^ unmatched `}` in format string\n   |\n   = note: if you intended to print `}`, you can escape it using `}}`\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[00:45:30] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:45:30] {"message":"invalid format string: invalid argument name `_foo`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/fmt/format-string-error.rs","byte_start":700,"byte_end":704,"line_start":17,"line_end":17,"column_start":23,"column_end":27,"is_primary":true,"text":[{"text":"    let _ = format!(\"{_foo}\", _foo = 6usize);","highlight_start":23,"highlight_end":27}],"label":"invalid argument name in format string","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"<format macros>","byte_start":55,"byte_end":86,"line_start":2,"line_end":2,"column_start":28,"column_end":59,"is_primary":false,"text":[{"text":"$ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )","highlight_start":28,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/fmt/format-string-error.rs","byte_start":690,"byte_end":722,"line_start":17,"line_end":17,"column_start":13,"column_end":45,"is_primary":false,"text":[{"text":"    let _ = format!(\"{_foo}\", _foo = 6usize);","highlight_start":13,"highlight_end":45}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"format!","def_site_span":{"file_name":"<format macros>","byte_start":0,"byte_end":90,"line_start":1,"line_end":2,"column_start":1,"column_end":63,"is_primary":false,"text":[{"text":"( $ ( $ arg : tt ) * ) => (","highlight_start":1,"highlight_end":28},{"text":"$ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )","highlight_start":1,"highlight_end":63}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"format_args!","def_site_span":null}}],"children":[{"message":"argument names cannot start with an underscore","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: invalid format string: invalid argument name `_foo`\n  --> /checkout/src/test/ui/fmt/format-string-error.rs:17:23\n   |\nLL |     let _ = format!(\"{_foo}\", _foo = 6usize);\n   |                       ^^^^ invalid argument name in format string\n   |\n   = note: argument names cannot start with an underscore\n\n"}
[00:45:30] {"message":"invalid format string: invalid argument name `_`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/fmt/format-string-error.rs","byte_start":813,"byte_end":814,"line_start":19,"line_end":19,"column_start":23,"columnormat-string-error.rs","byte_start":1106,"byte_end":1106,"line_start":25,"line_end":25,"column_start":23,"column_end":23,"is_primary":true,"text":[{"text":"    let _ = format!(\"{\\\\}\");","highlight_start":23,"highlight_end":23}],"label":"expected `}` in format string","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"<format macros>","byte_start":55,"byte_end":86,"line_start":2,"line_end":2,"column_start":28,"column_end":59,"is_primary":false,"text":[{"text":"$ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )","highlight_start":28,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/fmt/format-string-error.rs","byte_start":1096,"byte_end":1111,"line_start":25,"line_end":25,"column_start":13,"column_end":28,"is_primary":false,"text":[{"text":"    let _ = format!(\"{\\\\}\");","highlight_start":13,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"format!","def_site_span":{"file_name":"<format macros>","byte_start":0,"byte_end":90,"line_start":1,"line_end":2,"column_start":1,"column_end":63,"is_primary":false,"text":[{"text":"( $ ( $ arg : tt ) * ) => (","highlight_start":1,"highlight_end":28},{"text":"$ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )","highlight_start":1,"highlight_end":63}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"format_args!","def_site_span":null}}],"children":[],"rendered":"error: invalid format string: expected `'}'`, found `'\\\\'`\n  --> /checkout/src/test/ui/fmt/format-string-error.rs:25:23\n   |\nLL |     let _ = format!(\"{\\\\}\");\n   |                       ^ expected `}` in format string\n\n"}
[00:45:30] {"message":"aborting due to 7 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 7 previous errors\n\n"}
[00:45:30] ------------------------------------------
[00:45:30] 
[00:45:30] thread '[ui] ui/fmt/format-string-error.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:45:30] 
[00:45:30] 
[00:45:30] ---- [ui] ui/hygiene/local_inner_macros_disabled.rs stdout ----
[00:45:30] diff of stderr:
[00:45:30] 
[00:45:30] 1 error: cannot find macro `helper2!` in this scope
[00:45:30] +   --> $DIR/local_inner_macros_disabled.rs:18:1
[00:45:30] 3    |
[00:45:30] 3    |
[00:45:30] - LL | public_macro!();
[00:45:30] + LL | public_macro!(); //~ ERROR cannot find macro `helper2!` in this scope
[00:45:30] 6    |
[00:45:30] 7    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:45:30] 
[00:45:30] 
[00:45:30] 
[00:45:30] The actual stderr differed from the expected stderr.
[00:45:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/local_inner_macros_disabled/local_inner_macros_disabled.stderr
[00:45:30] To update references, rerun the tests and pass the `--bless` flag
[00:45:30] To only update this specific test, also p!` in this scope","highlight_start":1,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"public_macro!","def_site_span":{"file_name":"<public_macro macros>","byte_start":0,"byte_end":28,"line_start":1,"line_end":1,"column_start":1,"column_end":29,"is_primary":false,"text":[{"text":"(  ) => ( helper2 ! (  ) ; )","highlight_start":1,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: cannot find macro `helper2!` in this scope\n  --> /checkout/src/test/ui/hygiene/local_inner_macros_disabled.rs:18:1\n   |\nLL | public_macro!(); //~ ERROR cannot find macro `helper2!` in this scope\n   | ^^^^^^^^^^^^^^^^\n   |\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[00:45:30] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:45:30] ------------------------------------------
[00:45:30] 
[00:45:30] thread '[ui] ui/hygiene/local_inner_macros_disabled.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:45:30] 
[00:45:30] 
[00:45:30] ---- [ui] ui/hygiene/intercrate.rs stdout ----
[00:45:30] diff of stderr:
[00:45:30] 
[00:45:30] 1 error: type `fn() -> u32 {intercrate::foo::bar::f}` is private
[00:45:30] +   --> $DIR/intercrate.rs:20:16
[00:45:30] 3    |
[00:45:30] 3    |
[00:45:30] 4 LL |     assert_eq!(intercrate::foo::m!(), 1);
[00:45:30] 
[00:45:30] 
[00:45:30] The actual stderr differed from the expected stderr.
[00:45:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/intercrate/intercrate.stderr
[00:45:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/intercrate/intercrate.stderr
[00:45:30] To update references, rerun the tests and pass the `--bless` flag
[00:45:30] To only update this specific test, also pass `--test-args hygiene/intercrate.rs`
[00:45:30] error: 1 errors occurred comparing output.
[00:45:30] status: exit code: 101
[00:45:30] status: exit code: 101
[00:45:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/intercrate.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/intercrate/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/intercrate/auxiliary" "-A" "unused"
[00:45:30] ------------------------------------------
[00:45:30] 
[00:45:30] ------------------------------------------
[00:45:30] stderr:
[00:45:30] stderr:
[00:45:30] ------------------------------------------
[00:45:30] {"message":"type `fn() -> u32 {intercrate::foo::bar::f}` is private","code":null,"level":"error","spans":[{"file_name":"<m macros>","byte_start":10,"byte_end":11,"line_start":1,"line_end":1,"column_start":11,"column_end":12,"is_primary":true,"text":[{"text":"(  ) => { f (  ) ; }","highlight_start":11,"highlight_enget=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-16966/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-16966/auxiliary" "-A" "unused"
[00:45:30]     Error {
[00:45:30]         line_num: 13,
[00:45:30]         kind: Some(
[00:45:30]             Error
---
[00:45:30] thread '[ui] ui/issue-16966.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[00:45:30] 
[00:45:30] ---- [ui] ui/issue-32829.rs stdout ----
[00:45:30] 
[00:45:30] error: /checkout/src/test/ui/issue-32829.rs:13: unexpected error: '13:22: 13:36: calls in statics are limited to constant functions, tuple structs and tuple variants [E0015]'
[00:45:30] error: 1 unexpected errors found, 0 expected errors not found
[00:45:30] status: exit code: 101
[00:45:30] status: exit code: 101
[00:45:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-32829.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-32829/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu6_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/borrowck-let-suggestion.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/borrowck-let-suggestion/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/borrowck-let-suggestion/auxiliary" "-A" "unused"
[00:45:30] ------------------------------------------
[00:45:30] 
[00:45:30] ------------------------------------------
[00:45:30] stderr:
[00:45:30] stderr:
[00:45:30] ------------------------------------------
[00:45:30] {"message":"borrowed value does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n
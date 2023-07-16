plain
travis_time:end:1ebf24ac:start=1550223573941862237,finish=1550223576682769422,duration=2740907185
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:05:18] .................................................................................................... 1400/5390
[01:05:21] .................................................................................................... 1500/5390
[01:05:24] .................................................................................................... 1600/5390
[01:05:27] ...i..........................................................................i..................... 1700/5390
[01:05:31] ......F............................................................................................. 1800/5390
[01:05:39] .................................................................................................... 2000/5390
[01:05:42] ...................................i................................................................ 2100/5390
[01:05:46] .................................................................................................... 2200/5390
[01:05:51] .................................................................................................... 2300/5390
---
[01:07:25] ......................................i............................................................. 4600/5390
[01:07:31] .................................................................................................... 4700/5390
[01:07:35] .................................................................................................... 4800/5390
[01:07:39] .................................................................................................... 4900/5390
[01:07:43] ...................................F................................................................ 5000/5390
[01:07:50] .................................................................................................... 5200/5390
[01:07:53] .................................................................................................... 5300/5390
[01:07:56] .............................i............................................................
[01:07:56] failures:
[01:07:56] failures:
[01:07:56] 
[01:07:56] ---- [ui] ui/hygiene/no_implicit_prelude.rs stdout ----
[01:07:56] 
[01:07:56] error: /checkout/src/test/ui/hygiene/no_implicit_prelude.rs:16: unexpected error: '16:9: 16:20: cannot find macro `print!` in this scope'
[01:07:56] error: 1 unexpected errors found, 0 expected errors not found
[01:07:56] status: exit code: 1
[01:07:56] status: exit code: 1
[01:07:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/no_implicit_prelude.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/no_implicit_prelude/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/no_implicit_prelude/auxiliary" "-A" "unused"
[01:07:56]     Error {
[01:07:56]         line_num: 16,
[01:07:56]         kind: Some(
[01:07:56]             Error
[01:07:56]             Error
[01:07:56]         ),
[01:07:56]         msg: "16:9: 16:20: cannot find macro `print!` in this scope"
[01:07:56] ]
[01:07:56] 
[01:07:56] thread '[ui] ui/hygiene/no_implicit_prelude.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1378:13
[01:07:56] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:07:56] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:07:56] 
[01:07:56] ---- [ui] ui/tag-that-dare-not-speak-its-name.rs stdout ----
[01:07:56] diff of stderr:
[01:07:56] 
[01:07:56] + error: cannot find macro `panic!` in this scope
[01:07:56] +    |
[01:07:56] + LL |     std::panic!();
[01:07:56] +    |     ^^^^^^^^^^^^^^
[01:07:56] +    |
[01:07:56] +    |
[01:07:56] +    = help: have you added the `#[macro_use]` on the module/import?
[01:07:56] + 
[01:07:56] 1 error[E0308]: mismatched types
[01:07:56] +   --> $DIR/tag-that-dare-not-speak-its-name.rs:6:28
[01:07:56] +    |
[01:07:56] +    |
[01:07:56] + LL | fn last<T>(v: Vec<&T> ) -> std::option::Option<T> {
[01:07:56] +    |    ----                    ^^^^^^^^^^^^^^^^^^^^^^ expected enum `std::option::Option`, found ()
[01:07:56] +    |    |
[01:07:56] +    |    this function's body doesn't return
[01:07:56] + LL |     std::panic!();
[01:07:56] +    |                  - help: consider removing this semicolon
[01:07:56] +    = note: expected type `std::option::Option<T>`
[01:07:56] +               found type `()`
[01:07:56] + 
[01:07:56] + error[E0308]: mismatched types
[01:07:56] + error[E0308]: mismatched types
[01:07:56] 2   --> $DIR/tag-that-dare-not-speak-its-name.rs:12:20
[01:07:56] 3    |
[01:07:56] 4 LL |     let x : char = last(y);
[01:07:56] 7    = note: expected type `char`
[01:07:56] 8               found type `std::option::Option<_>`
[01:07:56] 9 
[01:07:56] - error: aborting due to previous error
[01:07:56] - error: aborting due to previous error
[01:07:56] + error: aborting due to 3 previous errors
[01:07:56] 11 
[01:07:56] 12 For more information about this error, try `rustc --explain E0308`.
[01:07:56] 13 
[01:07:56] 
[01:07:56] 
[01:07:56] The actual stderr differed from the expected stderr.
[01:07:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tag-that-dare-not-speak-its-name/tag-that-dare-not-speak-its-name.stderr
[01:07:56] To update references, rerun the tests and pass the `--bless` flag
[01:07:56] To only update this specific test, also pass `--test-args tag-that-dare-not-speak-its-name.rs`
[01:07:56] error: 1 errors occurred comparing output.
[01:07:56] status: exit code: 1
[01:07:56] status: exit code: 1
[01:07:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/tag-that-dare-not-speak-its-name.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tag-that-dare-not-speak-its-name/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tag-that-dare-not-speak-its-name/auxiliary" "-A" "unused"
[01:07:56] ------------------------------------------
[01:07:56] 
[01:07:56] ------------------------------------------
[01:07:56] stderr:
[01:07:56] stderr:
[01:07:56] ------------------------------------------
[01:07:56] {"message":"cannot find macro `panic!` in this scope","code":null,"level":"error","spans":[{"file_name":"<::std::macros::panic macros>","byte_start":12,"byte_end":17,"line_start":1,"line_end":1,"column_start":13,"column_end":18,"is_primary":true,"text":[{"text":"(  ) => ( { panic ! ( \"explicit panic\" ) } ) ; ( $ msg : expr ) => (","highlight_start":13,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/tag-that-dare-not-speak-its-name.rs","byte_start":115,"byte_end":129,"line_start":7,"line_end":7,"column_start":5,"column_end":19,"is_primary":false,"text":[{"text":"    std::panic!();","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"std::panic!","def_site_span":{"file_name":"<::std::macros::panic macros>","byte_start":0,"byte_end":427,"line_start":1,"line_end":10,"column_start":1,"column_end":68,"is_primary":false,"text":[{"text":"(  ) => ( { panic ! ( \"explicit panic\" ) } ) ; ( $ msg : expr ) => (","highlight_start":1,"highlight_end":69},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"$ crate :: rt :: begin_panic (","highlight_start":1,"highlight_end":31},{"text":"$ msg , & ( file ! (  ) , line ! (  ) , __rust_unstable_column ! (  ) ) ) } )","highlight_start":1,"highlight_end":78},{"text":"; ( $ msg : expr , ) => ( { panic ! ( $ msg ) } ) ; (","highlight_start":1,"highlight_end":54},{"text":"$ fmt : expr , $ ( $ arg : tt ) + ) => (","highlight_start":1,"highlight_end":41},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"$ crate :: rt :: begin_panic_fmt (","highlight_start":1,"highlight_end":35},{"text":"& format_args ! ( $ fmt , $ ( $ arg ) + ) , & (","highlight_start":1,"highlight_end":48},{"text":"file ! (  ) , line ! (  ) , __rust_unstable_column ! (  ) ) ) } ) ;","highlight_start":1,"highlight_end":68}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"have you added the `#[macro_use]` on the module/import?","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: cannot find macro `panic!` in this scope\n  --> /checkout/src/test/ui/tag-that-dare-not-speak-its-name.rs:7:5\n   |\nLL |     std::panic!();\n   |     ^^^^^^^^^^^^^^\n   |\n   = help: have you added the `#[macro_use]` on the module/import?\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[01:07:56] {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n
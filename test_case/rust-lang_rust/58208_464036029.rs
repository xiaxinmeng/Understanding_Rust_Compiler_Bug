plain
travis_time:end:0b1d5126:start=1550230414971846447,finish=1550230417244110807,duration=2272264360
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:06:41] ......................................i............................................................. 4600/5390
[01:06:47] .................................................................................................... 4700/5390
[01:06:50] .................................................................................................... 4800/5390
[01:06:54] .................................................................................................... 4900/5390
[01:06:58] .................................F.................................................................. 5000/5390
[01:07:05] .................................................................................................... 5200/5390
[01:07:07] .................................................................................................... 5300/5390
[01:07:10] .............................i............................................................
[01:07:10] failures:
[01:07:10] failures:
[01:07:10] 
[01:07:10] ---- [ui] ui/tag-that-dare-not-speak-its-name.rs stdout ----
[01:07:10] diff of stderr:
[01:07:10] 
[01:07:10] + error: cannot find macro `panic!` in this scope
[01:07:10] +    |
[01:07:10] + LL |     ::std::panic!();
[01:07:10] +    |     ^^^^^^^^^^^^^^^^
[01:07:10] +    |
[01:07:10] +    |
[01:07:10] +    = help: have you added the `#[macro_use]` on the module/import?
[01:07:10] + 
[01:07:10] 1 error[E0308]: mismatched types
[01:07:10] +   --> $DIR/tag-that-dare-not-speak-its-name.rs:6:28
[01:07:10] +    |
[01:07:10] +    |
[01:07:10] + LL | fn last<T>(v: Vec<&T> ) -> std::option::Option<T> {
[01:07:10] +    |    ----                    ^^^^^^^^^^^^^^^^^^^^^^ expected enum `std::option::Option`, found ()
[01:07:10] +    |    |
[01:07:10] +    |    this function's body doesn't return
[01:07:10] + LL |     ::std::panic!();
[01:07:10] +    |                    - help: consider removing this semicolon
[01:07:10] +    = note: expected type `std::option::Option<T>`
[01:07:10] +               found type `()`
[01:07:10] + 
[01:07:10] + error[E0308]: mismatched types
[01:07:10] + error[E0308]: mismatched types
[01:07:10] 2   --> $DIR/tag-that-dare-not-speak-its-name.rs:12:20
[01:07:10] 3    |
[01:07:10] 4 LL |     let x : char = last(y);
[01:07:10] 7    = note: expected type `char`
[01:07:10] 8               found type `std::option::Option<_>`
[01:07:10] 9 
[01:07:10] - error: aborting due to previous error
[01:07:10] - error: aborting due to previous error
[01:07:10] + error: aborting due to 3 previous errors
[01:07:10] 11 
[01:07:10] 12 For more information about this error, try `rustc --explain E0308`.
[01:07:10] 13 
[01:07:10] 
[01:07:10] 
[01:07:10] The actual stderr differed from the expected stderr.
[01:07:10] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tag-that-dare-not-speak-its-name/tag-that-dare-not-speak-its-name.stderr
[01:07:10] To update references, rerun the tests and pass the `--bless` flag
[01:07:10] To only update this specific test, also pass `--test-args tag-that-dare-not-speak-its-name.rs`
[01:07:10] error: 1 errors occurred comparing output.
[01:07:10] status: exit code: 1
[01:07:10] status: exit code: 1
[01:07:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/tag-that-dare-not-speak-its-name.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tag-that-dare-not-speak-its-name/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tag-that-dare-not-speak-its-name/auxiliary" "-A" "unused"
[01:07:10] ------------------------------------------
[01:07:10] 
[01:07:10] ------------------------------------------
[01:07:10] stderr:
[01:07:10] stderr:
[01:07:10] ------------------------------------------
[01:07:10] {"message":"cannot find macro `panic!` in this scope","code":null,"level":"error","spans":[{"file_name":"<::std::macros::panic macros>","byte_start":12,"byte_end":17,"line_start":1,"line_end":1,"column_start":13,"column_end":18,"is_primary":true,"text":[{"text":"(  ) => ( { panic ! ( \"explicit panic\" ) } ) ; ( $ msg : expr ) => (","highlight_start":13,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/tag-that-dare-not-speak-its-name.rs","byte_start":115,"byte_end":131,"line_start":7,"line_end":7,"column_start":5,"column_end":21,"is_primary":false,"text":[{"text":"    ::std::panic!();","highlight_start":5,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"::std::panic!","def_site_span":{"file_name":"<::std::macros::panic macros>","byte_start":0,"byte_end":427,"line_start":1,"line_end":10,"column_start":1,"column_end":68,"is_primary":false,"text":[{"text":"(  ) => ( { panic ! ( \"explicit panic\" ) } ) ; ( $ msg : expr ) => (","highlight_start":1,"highlight_end":69},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"$ crate :: rt :: begin_panic (","highlight_start":1,"highlight_end":31},{"text":"$ msg , & ( file ! (  ) , line ! (  ) , __rust_unstable_column ! (  ) ) ) } )","highlight_start":1,"highlight_end":78},{"text":"; ( $ msg : expr , ) => ( { panic ! ( $ msg ) } ) ; (","highlight_start":1,"highlight_end":54},{"text":"$ fmt : expr , $ ( $ arg : tt ) + ) => (","highlight_start":1,"highlight_end":41},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"$ crate :: rt :: begin_panic_fmt (","highlight_start":1,"highlight_end":35},{"text":"& format_args ! ( $ fmt , $ ( $ arg ) + ) , & (","highlight_start":1,"highlight_end":48},{"text":"file ! (  ) , line ! (  ) , __rust_unstable_column ! (  ) ) ) } ) ;","highlight_start":1,"highlight_end":68}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"have you added the `#[macro_use]` on the module/import?","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: cannot find macro `panic!` in this scope\n  --> /checkout/src/test/ui/tag-that-dare-not-speak-its-name.rs:7:5\n   |\nLL |     ::std::panic!();\n   |     ^^^^^^^^^^^^^^^^\n   |\n   = help: have you added the `#[macro_use]` on the module/import?\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[01:07:10] {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n
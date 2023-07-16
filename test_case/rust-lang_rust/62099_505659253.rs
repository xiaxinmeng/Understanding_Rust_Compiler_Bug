plain
travis_time:end:0067b1ec:start=1561501701785641069,finish=1561501703954592102,duration=2168951033
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:56:00] 
[00:56:00] running 5706 tests
[00:56:05] .................................................................................................... 100/5706
[00:56:12] ...........................................................................F........................ 200/5706
[00:56:18] .................................................................................................... 400/5706
[00:56:22] .................................................................................................... 500/5706
[00:56:25] ...................................i................................................................ 600/5706
[00:56:29] .................................................................................................... 700/5706
[00:56:29] .................................................................................................... 700/5706
[00:56:34] .................................................................................................... 800/5706
[00:56:39] .................................................................................................... 900/5706
[00:56:44] ...................................................i...........i.................................... 1000/5706
[00:56:47] .........................................F......................................iiiii............... 1100/5706
[00:56:51] .....................................................F...FF..F...................................... 1200/5706
[00:56:56] .................................................................................................... 1400/5706
[00:56:59] .................................................................................................... 1500/5706
[00:57:02] .................................................................................................... 1600/5706
[00:57:04] .................................................................................................... 1700/5706
[00:57:04] .................................................................................................... 1700/5706
[00:57:07] ..................i................................................................................. 1800/5706
[00:57:11] ........................................................................F........................... 1900/5706
[00:57:18] .................................................................................................... 2100/5706
[00:57:22] ...........................................................i........................................ 2200/5706
[00:57:26] .................................................................................................... 2300/5706
[00:57:30] .................................................................................................... 2400/5706
[00:57:30] .................................................................................................... 2400/5706
[00:57:34] .................................................................................................... 2500/5706
[00:57:38] .................................................................................................... 2600/5706
[00:57:42] .................................................................................................... 2700/5706
[00:57:46] ...................................................F................................................ 2800/5706
[00:57:54] .................................................................................................... 3000/5706
[00:57:54] .................................................................................................... 3000/5706
[00:57:58] .................................................................................................F.. 3100/5706
[00:58:02] ......................................................................................F............. 3200/5706
[00:58:05] .................................................................................................... 3300/5706
[00:58:09] ..........................................................................................F......... 3400/5706
[00:58:13] ...........i............................................F...F..F....F............................... 3500/5706
[00:58:16] ...........................................................................F.........ii...i..ii..... 3600/5706
[00:58:24] .................................................................................................... 3800/5706
[00:58:27] .................................................................................................ii. 3900/5706
[00:58:31] .................................................................................................... 4000/5706
[00:58:33] ..................i................................................................................. 4100/5706
[00:58:33] ..................i................................................................................. 4100/5706
[00:58:35] ..................................................................................i................. 4200/5706
[00:58:37] ......................................................................F............................. 4300/5706
[00:58:43] .F............................................................................................F...F. 4400/5706
[00:58:56] .......F.F....................................................................F..F.................. 4500/5706
[00:59:02] .................................................................................................... 4700/5706
[00:59:06] .................................................................................................... 4800/5706
[00:59:06] .................................................................................................... 4800/5706
[00:59:13] .........F..........................F............................................................... 4900/5706
[00:59:22] .................................................................................................... 5100/5706
[00:59:26] .................................................................................................... 5200/5706
[00:59:30] .................................................................................................... 5300/5706
[00:59:30] .................................................................................................... 5300/5706
[00:59:34] ..........................................................F......................................... 5400/5706
[00:59:40] .................................................................................................... 5600/5706
[00:59:43] ............................................i....................................................... 5700/5706
[00:59:43] ......
[00:59:43] failures:
[00:59:43] failures:
[00:59:43] 
[00:59:43] ---- [ui] ui/async-await/issues/issue-60674.rs stdout ----
[00:59:43] diff of stdout:
[00:59:43] 
[00:59:43] - async fn f(mut x: u8) { }
[00:59:43] - async fn g((mut x, y, mut z): (u8, u8, u8)) { }
[00:59:43] - async fn g(mut x: u8, (a, mut b, c): (u8, u8, u8), y: u8) { }
[00:59:43] + async fn f(mut x: u8 )   {  }
[00:59:43] + async fn g((mut x, y ,  mut     z  )  :   (   u8   ,    u8    ,     u8     )     )           {      }
[00:59:43] + async fn g(mut x: u8 ,  (  a  ,   mut       b   ,    c    )    :     (     u8     ,      u8      ,       u8       )       ,        y        :         u8         )                   {          }
[00:59:43] 
[00:59:43] 
[00:59:43] The actual stdout differed from the expected stdout.
[00:59:43] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-60674/issue-60674.stdout
[00:59:43] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-60674/issue-60674.stdout
[00:59:43] To update references, rerun the tests and pass the `--bless` flag
[00:59:43] To only update this specific test, also pass `--test-args async-await/issues/issue-60674.rs`
[00:59:43] error: 1 errors occurred comparing output.
[00:59:43] status: exit code: 0
[00:59:43] status: exit code: 0
[00:59:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-60674.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-60674" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-60674/auxiliary" "-A" "unused"
[00:59:43] ------------------------------------------
[00:59:43] ------------------------------------------
[00:59:43] async fn f(mut x: u8 )   {  }
[00:59:43] async fn g((mut x, y ,  mut     z  )  :   (   u8   ,    u8    ,     u8     )     )           {      }
[00:59:43] async fn g(mut x: u8 ,  (  a  ,   mut       b   ,    c    )    :     (     u8     ,      u8      ,       u8       )       ,        y        :         u8         )                   {          }
[00:59:43] ------------------------------------------
[00:59:43] stderr:
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] 
[00:59:43] ---- [ui] ui/did_you_mean/E0178.rs stdout ----
[00:59:43] diff of stderr:
[00:59:43] 
[00:59:43] 16 LL |     y: &'a mut Foo + 'a,
[00:59:43] 17    |        ^^^^^^^^^^^^^^^^ help: try adding parentheses: `&'a mut (Foo + 'a)`
[00:59:43] 18 
[00:59:43] - error[E0178]: expected a path on the left-hand side of `+`, not `fn() -> Foo`
[00:59:43] + error[E0178]: expected a path on the left-hand side of `+`, not `fn() ->  Foo`
[00:59:43] 20   --> $DIR/E0178.rs:9:8
[00:59:43] 21    |
[00:59:43] 22 LL |     z: fn() -> Foo + 'a,
[00:59:43] 
[00:59:43] The actual stderr differed from the expected stderr.
[00:59:43] The actual stderr differed from the expected stderr.
[00:59:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/E0178/E0178.stderr
[00:59:43] To update references, rerun the tests and pass the `--bless` flag
[00:59:43] To only update this specific test, also pass `--test-args did_you_mean/E0178.rs`
[00:59:43] error: 1 errors occurred comparing output.
[00:59:43] status: exit code: 1
[00:59:43] status: exit code: 1
[00:59:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/E0178.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/E0178" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/E0178/auxiliary" "-A" "unused"
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] stderr:
[00:59:43] stderr:
[00:59:43] ------------------------------------------
[00:59:43] error[E0178]: expected a path on the left-hand side of `+`, not `&'a Foo`
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |     w: &'a Foo + Copy, //~ ERROR expected a path
[00:59:43]    |        ^^^^^^^^^^^^^^ help: try adding parentheses: `&'a (Foo + Copy)`
[00:59:43] 
[00:59:43] error[E0178]: expected a path on the left-hand side of `+`, not `&'a Foo`
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |     x: &'a Foo + 'a, //~ ERROR expected a path
[00:59:43]    |        ^^^^^^^^^^^^ help: try adding parentheses: `&'a (Foo + 'a)`
[00:59:43] 
[00:59:43] error[E0178]: expected a path on the left-hand side of `+`, not `&'a mut Foo`
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |     y: &'a mut Foo + 'a, //~ ERROR expected a path
[00:59:43]    |        ^^^^^^^^^^^^^^^^ help: try adding parentheses: `&'a mut (Foo + 'a)`
[00:59:43] 
[00:59:43] error[E0178]: expected a path on the left-hand side of `+`, not `fn() ->  Foo`
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |     z: fn() -> Foo + 'a, //~ ERROR expected a path
[00:59:43]    |        ^^^^^^^^^^^^^^^^ perhaps you forgot parentheses?
[00:59:43] error: aborting due to 4 previous errors
[00:59:43] 
[00:59:43] For more information about this error, try `rustc --explain E0178`.
[00:59:43] 
---
[00:59:43] 
[00:59:43] 7    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:59:43] 8 help: you can escape reserved keywords to use them as identifiers
[00:59:43] 9    |
[00:59:43] - LL | (  ) => ( pub fn r#async (  ) {  } )
[00:59:43] -    |                  ^^^^^^^
[00:59:43] + LL | (  )   =>    (     pub      fn       r#async        (          )           {             }              )
[00:59:43] 12 
[00:59:43] 13 error: aborting due to previous error
[00:59:43] 14 
[00:59:43] 
[00:59:43] 
[00:59:43] 
[00:59:43] The actual stderr differed from the expected stderr.
[00:59:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-keywords-2015-2018-expansion/edition-keywords-2015-2018-expansion.stderr
[00:59:43] To update references, rerun the tests and pass the `--bless` flag
[00:59:43] To only update this specific test, also pass `--test-args editions/edition-keywords-2015-2018-expansion.rs`
[00:59:43] error: 1 errors occurred comparing output.
[00:59:43] status: exit code: 1
[00:59:43] status: exit code: 1
[00:59:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/editions/edition-keywords-2015-2018-expansion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-keywords-2015-2018-expansion" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-keywords-2015-2018-expansion/auxiliary" "-A" "unused"
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] stderr:
[00:59:43] stderr:
[00:59:43] ------------------------------------------
[00:59:43] error: expected identifier, found reserved keyword `async`
[00:59:43]   --> /checkout/src/test/ui/editions/edition-keywords-2015-2018-expansion.rs:8:5
[00:59:43]    |
[00:59:43] LL |     produces_async! {} //~ ERROR expected identifier, found reserved keyword
[00:59:43]    |
[00:59:43]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:59:43] help: you can escape reserved keywords to use them as identifiers
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL | (  )   =>    (     pub      fn       r#async        (          )           {             }              )
[00:59:43] 
[00:59:43] error: aborting due to previous error
[00:59:43] 
[00:59:43] 
---
[00:59:43] diff of stderr:
[00:59:43] 
[00:59:43] 31    |                                   ^^^^^ no rules expected this token in macro call
[00:59:43] 32 
[00:59:43] 33 error: macro expansion ends with an incomplete expression: expected one of `move`, `|`, or `||`
[00:59:43] -   --> <::edition_kw_macro_2015::passes_ident macros>:1:25
[00:59:43] +   --> <::edition_kw_macro_2015::passes_ident macros>:1:61
[00:59:43] 35    |
[00:59:43] - LL | ( $ i : ident ) => ( $ i )
[00:59:43] -    |                         ^ expected one of `move`, `|`, or `||` here
[00:59:43] + LL | ( $  i   :    ident     )      =>       (        $         i          )
[00:59:43] +    |                                                             ^ expected one of `move`, `|`, or `||` here
[00:59:43] 39   ::: $DIR/edition-keywords-2018-2015-parsing.rs:16:8
[00:59:43] 40    |
[00:59:43] 
[00:59:43] 
[00:59:43] 
[00:59:43] The actual stderr differed from the expected stderr.
[00:59:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-keywords-2018-2015-parsing/edition-keywords-2018-2015-parsing.stderr
[00:59:43] To update references, rerun the tests and pass the `--bless` flag
[00:59:43] To only update this specific test, also pass `--test-args editions/edition-keywords-2018-2015-parsing.rs`
[00:59:43] error: 1 errors occurred comparing output.
[00:59:43] status: exit code: 1
[00:59:43] status: exit code: 1
[00:59:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/editions/edition-keywords-2018-2015-parsing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-keywords-2018-2015-parsing" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-keywords-2018-2015-parsing/auxiliary" "-A" "unused"
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] stderr:
[00:59:43] stderr:
[00:59:43] ------------------------------------------
[00:59:43] error: expected identifier, found reserved keyword `async`
[00:59:43]   --> /checkout/src/test/ui/editions/edition-keywords-2018-2015-parsing.rs:8:13
[00:59:43]    |
[00:59:43] LL |     let mut async = 1; //~ ERROR expected identifier, found reserved keyword `async`
[00:59:43] help: you can escape reserved keywords to use them as identifiers
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |     let mut r#async = 1; //~ ERROR expected identifier, found reserved keyword `async`
[00:59:43] 
[00:59:43] error: expected identifier, found reserved keyword `async`
[00:59:43]   --> /checkout/src/test/ui/editions/edition-keywords-2018-2015-parsing.rs:18:13
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |     module::async(); //~ ERROR expected identifier, found reserved keyword `async`
[00:59:43] help: you can escape reserved keywords to use them as identifiers
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |     module::r#async(); //~ ERROR expected identifier, found reserved keyword `async`
[00:59:43] 
[00:59:43] 
[00:59:43] error: no rules expected the token `r#async`
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |     r#async = consumes_async!(r#async); //~ ERROR no rules expected the token `r#async`
[00:59:43]    |                               ^^^^^^^ no rules expected this token in macro call
[00:59:43] 
[00:59:43] error: no rules expected the token `async`
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |     r#async = consumes_async_raw!(async); //~ ERROR no rules expected the token `async`
[00:59:43]    |                                   ^^^^^ no rules expected this token in macro call
[00:59:43] 
[00:59:43] error: macro expansion ends with an incomplete expression: expected one of `move`, `|`, or `||`
[00:59:43]   --> <::edition_kw_macro_2015::passes_ident macros>:1:61
[00:59:43]    |
[00:59:43] LL | ( $  i   :    ident     )      =>       (        $         i          )
[00:59:43]    |                                                             ^ expected one of `move`, `|`, or `||` here
[00:59:43]   ::: /checkout/src/test/ui/editions/edition-keywords-2018-2015-parsing.rs:16:8
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |     if passes_ident!(async) == 1 {}
[00:59:43] 
[00:59:43] error: aborting due to 5 previous errors
[00:59:43] 
[00:59:43] 
---
[00:59:43] 
[00:59:43] 7    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:59:43] 8 help: you can escape reserved keywords to use them as identifiers
[00:59:43] 9    |
[00:59:43] - LL | (  ) => ( pub fn r#async (  ) {  } )
[00:59:43] -    |                  ^^^^^^^
[00:59:43] + LL | (  )   =>    (     pub      fn       r#async        (          )           {             }              )
[00:59:43] 12 
[00:59:43] 13 error: aborting due to previous error
[00:59:43] 14 
[00:59:43] 
[00:59:43] 
[00:59:43] 
[00:59:43] The actual stderr differed from the expected stderr.
[00:59:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-keywords-2018-2018-expansion/edition-keywords-2018-2018-expansion.stderr
[00:59:43] To update references, rerun the tests and pass the `--bless` flag
[00:59:43] To only update this specific test, also pass `--test-args editions/edition-keywords-2018-2018-expansion.rs`
[00:59:43] error: 1 errors occurred comparing output.
[00:59:43] status: exit code: 1
[00:59:43] status: exit code: 1
[00:59:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/editions/edition-keywords-2018-2018-expansion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-keywords-2018-2018-expansion" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-keywords-2018-2018-expansion/auxiliary" "-A" "unused"
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] stderr:
[00:59:43] stderr:
[00:59:43] ------------------------------------------
[00:59:43] error: expected identifier, found reserved keyword `async`
[00:59:43]   --> /checkout/src/test/ui/editions/edition-keywords-2018-2018-expansion.rs:8:5
[00:59:43]    |
[00:59:43] LL |     produces_async! {} //~ ERROR expected identifier, found reserved keyword `async`
[00:59:43]    |
[00:59:43]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:59:43] help: you can escape reserved keywords to use them as identifiers
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL | (  )   =>    (     pub      fn       r#async        (          )           {             }              )
[00:59:43] 
[00:59:43] error: aborting due to previous error
[00:59:43] 
[00:59:43] 
---
[00:59:43] diff of stderr:
[00:59:43] 
[00:59:43] 31    |                                   ^^^^^ no rules expected this token in macro call
[00:59:43] 32 
[00:59:43] 33 error: macro expansion ends with an incomplete expression: expected one of `move`, `|`, or `||`
[00:59:43] -   --> <::edition_kw_macro_2018::passes_ident macros>:1:25
[00:59:43] +   --> <::edition_kw_macro_2018::passes_ident macros>:1:61
[00:59:43] 35    |
[00:59:43] - LL | ( $ i : ident ) => ( $ i )
[00:59:43] -    |                         ^ expected one of `move`, `|`, or `||` here
[00:59:43] + LL | ( $  i   :    ident     )      =>       (        $         i          )
[00:59:43] +    |                                                             ^ expected one of `move`, `|`, or `||` here
[00:59:43] 39   ::: $DIR/edition-keywords-2018-2018-parsing.rs:16:8
[00:59:43] 40    |
[00:59:43] 
[00:59:43] 
[00:59:43] 
[00:59:43] The actual stderr differed from the expected stderr.
[00:59:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-keywords-2018-2018-parsing/edition-keywords-2018-2018-parsing.stderr
[00:59:43] To update references, rerun the tests and pass the `--bless` flag
[00:59:43] To only update this specific test, also pass `--test-args editions/edition-keywords-2018-2018-parsing.rs`
[00:59:43] error: 1 errors occurred comparing output.
[00:59:43] status: exit code: 1
[00:59:43] status: exit code: 1
[00:59:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/editions/edition-keywords-2018-2018-parsing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-keywords-2018-2018-parsing" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-keywords-2018-2018-parsing/auxiliary" "-A" "unused"
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] stderr:
[00:59:43] stderr:
[00:59:43] ------------------------------------------
[00:59:43] error: expected identifier, found reserved keyword `async`
[00:59:43]   --> /checkout/src/test/ui/editions/edition-keywords-2018-2018-parsing.rs:8:13
[00:59:43]    |
[00:59:43] LL |     let mut async = 1; //~ ERROR expected identifier, found reserved keyword `async`
[00:59:43] help: you can escape reserved keywords to use them as identifiers
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |     let mut r#async = 1; //~ ERROR expected identifier, found reserved keyword `async`
[00:59:43] 
[00:59:43] error: expected identifier, found reserved keyword `async`
[00:59:43]   --> /checkout/src/test/ui/editions/edition-keywords-2018-2018-parsing.rs:18:13
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |     module::async(); //~ ERROR expected identifier, found reserved keyword `async`
[00:59:43] help: you can escape reserved keywords to use them as identifiers
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |     module::r#async(); //~ ERROR expected identifier, found reserved keyword `async`
[00:59:43] 
[00:59:43] 
[00:59:43] error: no rules expected the token `r#async`
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |     r#async = consumes_async!(r#async); //~ ERROR no rules expected the token `r#async`
[00:59:43]    |                               ^^^^^^^ no rules expected this token in macro call
[00:59:43] 
[00:59:43] error: no rules expected the token `async`
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |     r#async = consumes_async_raw!(async); //~ ERROR no rules expected the token `async`
[00:59:43]    |                                   ^^^^^ no rules expected this token in macro call
[00:59:43] 
[00:59:43] error: macro expansion ends with an incomplete expression: expected one of `move`, `|`, or `||`
[00:59:43]   --> <::edition_kw_macro_2018::passes_ident macros>:1:61
[00:59:43]    |
[00:59:43] LL | ( $  i   :    ident     )      =>       (        $         i          )
[00:59:43]    |                                                             ^ expected one of `move`, `|`, or `||` here
[00:59:43]   ::: /checkout/src/test/ui/editions/edition-keywords-2018-2018-parsing.rs:16:8
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |     if passes_ident!(async) == 1 {}
[00:59:43] 
[00:59:43] error: aborting due to 5 previous errors
[00:59:43] 
[00:59:43] 
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] 
[00:59:43] ---- [ui] ui/impl-trait/impl-trait-plus-priority.rs stdout ----
[00:59:43] diff of stderr:
[00:59:43] 
[00:59:43] 16 LL | type A = fn() -> dyn A + B;
[00:59:43] 17    |                  ^^^^^^^^^ help: use parentheses to disambiguate: `(dyn A + B)`
[00:59:43] 18 
[00:59:43] - error[E0178]: expected a path on the left-hand side of `+`, not `fn() -> A`
[00:59:43] + error[E0178]: expected a path on the left-hand side of `+`, not `fn() ->  A`
[00:59:43] 21    |
[00:59:43] 21    |
[00:59:43] 22 LL | type A = fn() -> A + B;
[00:59:43] 
[00:59:43] The actual stderr differed from the expected stderr.
[00:59:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-trait-plus-priority/impl-trait-plus-priority.stderr
[00:59:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-trait-plus-priority/impl-trait-plus-priority.stderr
[00:59:43] To update references, rerun the tests and pass the `--bless` flag
[00:59:43] To only update this specific test, also pass `--test-args impl-trait/impl-trait-plus-priority.rs`
[00:59:43] error: 1 errors occurred comparing output.
[00:59:43] status: exit code: 1
[00:59:43] status: exit code: 1
[00:59:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/impl-trait-plus-priority.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-trait-plus-priority" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "parse-only" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-trait-plus-priority/auxiliary" "-A" "unused"
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] stderr:
[00:59:43] stderr:
[00:59:43] ------------------------------------------
[00:59:43] error: ambiguous `+` in a type
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL | type A = fn() -> impl A +;
[00:59:43]    |                  ^^^^^^^^ help: use parentheses to disambiguate: `(impl A)`
[00:59:43] 
[00:59:43] error: ambiguous `+` in a type
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL | type A = fn() -> impl A + B;
[00:59:43]    |                  ^^^^^^^^^^ help: use parentheses to disambiguate: `(impl A + B)`
[00:59:43] 
[00:59:43] error: ambiguous `+` in a type
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL | type A = fn() -> dyn A + B;
[00:59:43]    |                  ^^^^^^^^^ help: use parentheses to disambiguate: `(dyn A + B)`
[00:59:43] 
[00:59:43] error[E0178]: expected a path on the left-hand side of `+`, not `fn() ->  A`
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL | type A = fn() -> A + B;
[00:59:43]    |          ^^^^^^^^^^^^^ perhaps you forgot parentheses?
[00:59:43] 
[00:59:43] error: ambiguous `+` in a type
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL | type A = Fn() -> impl A +;
[00:59:43]    |                  ^^^^^^^^ help: use parentheses to disambiguate: `(impl A)`
[00:59:43] 
[00:59:43] error: ambiguous `+` in a type
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL | type A = Fn() -> impl A + B;
[00:59:43]    |                  ^^^^^^^^^^ help: use parentheses to disambiguate: `(impl A + B)`
[00:59:43] 
[00:59:43] error: ambiguous `+` in a type
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL | type A = Fn() -> dyn A + B;
[00:59:43]    |                  ^^^^^^^^^ help: use parentheses to disambiguate: `(dyn A + B)`
[00:59:43] 
[00:59:43] error: ambiguous `+` in a type
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL | type A = &impl A +;
[00:59:43]    |           ^^^^^^^^ help: use parentheses to disambiguate: `(impl A)`
[00:59:43] 
[00:59:43] error: ambiguous `+` in a type
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL | type A = &impl A + B;
[00:59:43]    |           ^^^^^^^^^^ help: use parentheses to disambiguate: `(impl A + B)`
[00:59:43] 
[00:59:43] error: ambiguous `+` in a type
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL | type A = &dyn A + B;
[00:59:43]    |           ^^^^^^^^^ help: use parentheses to disambiguate: `(dyn A + B)`
[00:59:43] 
[00:59:43] error[E0178]: expected a path on the left-hand side of `+`, not `&A`
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL | type A = &A + B;
[00:59:43]    |          ^^^^^^ help: try adding parentheses: `&(A + B)`
[00:59:43] error: aborting due to 11 previous errors
[00:59:43] 
[00:59:43] For more information about this error, try `rustc --explain E0178`.
[00:59:43] 
---
[00:59:43] diff of stderr:
[00:59:43] 
[00:59:43] 2   --> $DIR/issue-39848.rs:8:19
[00:59:43] 3    |
[00:59:43] 4 LL |         if $tgt.has_$field() {}
[00:59:43] -    |         --                 - help: try placing this code inside a block: `{ foo(); }`
[00:59:43] +    |         --                 - help: try placing this code inside a block: `{ foo ( ) ;  }`
[00:59:43] 6    |         |
[00:59:43] 7    |         this `if` statement has a condition, but no block
[00:59:43] 8 ...
[00:59:43] 
[00:59:43] The actual stderr differed from the expected stderr.
[00:59:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39848/issue-39848.stderr
[00:59:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39848/issue-39848.stderr
[00:59:43] To update references, rerun the tests and pass the `--bless` flag
[00:59:43] To only update this specific test, also pass `--test-args issues/issue-39848.rs`
[00:59:43] error: 1 errors occurred comparing output.
[00:59:43] status: exit code: 1
[00:59:43] status: exit code: 1
[00:59:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-39848.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39848" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39848/auxiliary" "-A" "unused"
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] stderr:
[00:59:43] stderr:
[00:59:43] ------------------------------------------
[00:59:43] error: expected `{`, found `foo`
[00:59:43]   --> /checkout/src/test/ui/issues/issue-39848.rs:8:19
[00:59:43]    |
[00:59:43] LL |         if $tgt.has_$field() {}
[00:59:43]    |         --                 - help: try placing this code inside a block: `{ foo ( ) ;  }`
[00:59:43]    |         |
[00:59:43]    |         this `if` statement has a condition, but no block
[00:59:43] ...
[00:59:43] LL |     get_opt!(bar, foo);
[00:59:43]    |                   ^^^ expected `{`
[00:59:43] error: aborting due to previous error
[00:59:43] 
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] 
[00:59:43] ---- [ui] ui/issues/issue-60662.rs stdout ----
[00:59:43] diff of stdout:
[00:59:43] 
[00:59:43] 10 trait Animal { }
[00:59:43] 12 fn main() {
[00:59:43] 12 fn main() {
[00:59:43] -               pub existential type ServeFut : Animal;
[00:59:43] +               pub existential type                             ServeFut               :                               Animal               ;
[00:59:43] 15 
[00:59:43] 
[00:59:43] 
[00:59:43] The actual stdout differed from the expected stdout.
[00:59:43] The actual stdout differed from the expected stdout.
[00:59:43] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60662/issue-60662.stdout
[00:59:43] To update references, rerun the tests and pass the `--bless` flag
[00:59:43] To only update this specific test, also pass `--test-args issues/issue-60662.rs`
[00:59:43] error: 1 errors occurred comparing output.
[00:59:43] status: exit code: 0
[00:59:43] status: exit code: 0
[00:59:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-60662.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60662" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unpretty=hir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60662/auxiliary" "-A" "unused"
[00:59:43] ------------------------------------------
[00:59:43] // compile-pass
[00:59:43] // compile-pass
[00:59:43] // compile-flags: -Z unpretty=hir
[00:59:43] #![feature(existential_type)]
[00:59:43] #![feature(existential_type)]
[00:59:43] #[prelude_import]
[00:59:43] use ::std::prelude::v1::*;
[00:59:43] #[macro_use]
[00:59:43] extern crate std;
[00:59:43] trait Animal { }
[00:59:43] 
[00:59:43] fn main() {
[00:59:43] fn main() {
[00:59:43]               pub existential type                             ServeFut               :                               Animal               ;
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] stderr:
[00:59:43] ------------------------------------------
---
[00:59:43] diff of stderr:
[00:59:43] 
[00:59:43] 11    |     --      ^^----
[00:59:43] 12    |     |       |
[00:59:43] 13    |     |       expected `{`
[00:59:43] -    |     |       help: try placing this code inside a block: `{ 'b: { } }`
[00:59:43] +    |     |       help: try placing this code inside a block: `{ 'b :  {   }    }`
[00:59:43] 15    |     this `if` statement has a condition, but no block
[00:59:43] 17 error: expected `{`, found `'b`
[00:59:43] 
[00:59:43] 21    |                     ^^----
[00:59:43] 22    |                     |
[00:59:43] 22    |                     |
[00:59:43] 23    |                     expected `{`
[00:59:43] -    |                     help: try placing this code inside a block: `{ 'b: { } }`
[00:59:43] +    |                     help: try placing this code inside a block: `{ 'b :  {   }    }`
[00:59:43] 26 error: expected one of `.`, `?`, `{`, or an operator, found `'b`
[00:59:43] 27   --> $DIR/label_break_value_illegal_uses.rs:18:17
[00:59:43] 
[00:59:43] 
[00:59:43] 
[00:59:43] The actual stderr differed from the expected stderr.
[00:59:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/label/label_break_value_illegal_uses/label_break_value_illegal_uses.stderr
[00:59:43] To update references, rerun the tests and pass the `--bless` flag
[00:59:43] To only update this specific test, also pass `--test-args label/label_break_value_illegal_uses.rs`
[00:59:43] error: 1 errors occurred comparing output.
[00:59:43] status: exit code: 1
[00:59:43] status: exit code: 1
[00:59:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/label/label_break_value_illegal_uses.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/label/label_break_value_illegal_uses" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/label/label_break_value_illegal_uses/auxiliary" "-A" "unused"
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] stderr:
[00:59:43] stderr:
[00:59:43] ------------------------------------------
[00:59:43] error: expected one of `extern`, `fn`, or `{`, found `'b`
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |     unsafe 'b: {} //~ ERROR expected one of `extern`, `fn`, or `{`
[00:59:43]    |            ^^ expected one of `extern`, `fn`, or `{` here
[00:59:43] error: expected `{`, found `'b`
[00:59:43]   --> /checkout/src/test/ui/label/label_break_value_illegal_uses.rs:10:13
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |     if true 'b: {} //~ ERROR expected `{`, found `'b`
[00:59:43]    |     --      ^^----
[00:59:43]    |     |       expected `{`
[00:59:43]    |     |       expected `{`
[00:59:43]    |     |       help: try placing this code inside a block: `{ 'b :  {   }    }`
[00:59:43]    |     this `if` statement has a condition, but no block
[00:59:43] error: expected `{`, found `'b`
[00:59:43]   --> /checkout/src/test/ui/label/label_break_value_illegal_uses.rs:14:21
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |     if true {} else 'b: {} //~ ERROR expected `{`, found `'b`
[00:59:43]    |                     ^^----
[00:59:43]    |                     expected `{`
[00:59:43]    |                     expected `{`
[00:59:43]    |                     help: try placing this code inside a block: `{ 'b :  {   }    }`
[00:59:43] error: expected one of `.`, `?`, `{`, or an operator, found `'b`
[00:59:43]   --> /checkout/src/test/ui/label/label_break_value_illegal_uses.rs:18:17
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |     match false 'b: {} //~ ERROR expected one of `.`, `?`, `{`, or an operator
[00:59:43]    |     -----       ^^ expected one of `.`, `?`, `{`, or an operator here
[00:59:43]    |     while parsing this match expression
[00:59:43] 
[00:59:43] error: aborting due to 4 previous errors
[00:59:43] 
---
[00:59:43] ---- [ui] ui/macro_backtrace/main.rs stdout ----
[00:59:43] diff of stderr:
[00:59:43] 
[00:59:43] 24    | 
[00:59:43] 25   ::: <::ping::ping macros>:1:1
[00:59:43] 26    |
[00:59:43] - LL |   (  ) => { pong ! (  ) ; }
[00:59:43] -    |   |         |
[00:59:43] -    |   |         in this macro invocation
[00:59:43] -    |   |         in this macro invocation
[00:59:43] + LL |   (  )   =>    {     pong      !       (         )          ;           }
[00:59:43] +    |   |                  |
[00:59:43] +    |   |                  in this macro invocation
[00:59:43] 31    |   in this expansion of `ping!`
[00:59:43] 32 
[00:59:43] 32 
[00:59:43] 33 error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `error`
[00:59:43] 44    | 
[00:59:43] 44    | 
[00:59:43] 45   ::: <::ping::deep macros>:1:1
[00:59:43] 46    |
[00:59:43] - LL |   (  ) => { foo ! (  ) ; }
[00:59:43] -    |   |         |
[00:59:43] -    |   |         in this macro invocation (#2)
[00:59:43] -    |   |         in this macro invocation (#2)
[00:59:43] + LL |   (  )   =>    {     foo      !       (         )          ;           }
[00:59:43] +    |   |                  |
[00:59:43] +    |   |                  in this macro invocation (#2)
[00:59:43] +    |   |                  in this macro invocation (#2)
[00:59:43] 51    |   in this expansion of `deep!` (#1)
[00:59:43] 52    | 
[00:59:43] 53   ::: <::ping::foo macros>:1:1
[00:59:43] 54    |
[00:59:43] 54    |
[00:59:43] - LL |   (  ) => { bar ! (  ) ; }
[00:59:43] -    |   |         |
[00:59:43] -    |   |         in this macro invocation (#3)
[00:59:43] -    |   |         in this macro invocation (#3)
[00:59:43] + LL |   (  )   =>    {     bar      !       (         )          ;           }
[00:59:43] +    |   |                  |
[00:59:43] +    |   |                  in this macro invocation (#3)
[00:59:43] 59    |   in this expansion of `foo!` (#2)
[00:59:43] 60    | 
[00:59:43] 60    | 
[00:59:43] 61   ::: <::ping::bar macros>:1:1
[00:59:43] 62    |
[00:59:43] 62    |
[00:59:43] - LL |   (  ) => { ping ! (  ) ; }
[00:59:43] -    |   |         |
[00:59:43] -    |   |         in this macro invocation (#4)
[00:59:43] -    |   |         in this macro invocation (#4)
[00:59:43] + LL |   (  )   =>    {     ping      !       (         )          ;           }
[00:59:43] +    |   |                  |
[00:59:43] +    |   |                  in this macro invocation (#4)
[00:59:43] 67    |   in this expansion of `bar!` (#3)
[00:59:43] 68    | 
[00:59:43] 68    | 
[00:59:43] 69   ::: <::ping::ping macros>:1:1
[00:59:43] 70    |
[00:59:43] 70    |
[00:59:43] - LL |   (  ) => { pong ! (  ) ; }
[00:59:43] -    |   |         |
[00:59:43] -    |   |         in this macro invocation (#5)
[00:59:43] -    |   |         in this macro invocation (#5)
[00:59:43] + LL |   (  )   =>    {     pong      !       (         )          ;           }
[00:59:43] +    |   |                  |
[00:59:43] +    |   |                  in this macro invocation (#5)
[00:59:43] 75    |   in this expansion of `ping!` (#4)
[00:59:43] 76 
[00:59:43] 76 
[00:59:43] 77 error: aborting due to 3 previous errors
[00:59:43] 
[00:59:43] 
[00:59:43] The actual stderr differed from the expected stderr.
[00:59:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macro_backtrace/main/main.stderr
[00:59:43] To update references, rerun the tests and pass the `--bless` flag
[00:59:43] To only update this specific test, also pass `--test-args macro_backtrace/main.rs`
[00:59:43] error: 1 errors occurred comparing output.
[00:59:43] status: exit code: 1
[00:59:43] status: exit code: 1
[00:59:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macro_backtrace/main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macro_backtrace/main" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "external-macro-backtrace" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macro_backtrace/main/auxiliary" "-A" "unused"
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] stderr:
[00:59:43] stderr:
[00:59:43] ------------------------------------------
[00:59:43] error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `error`
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL | / macro_rules! pong {
[00:59:43] LL | |     () => { syntax error };
[00:59:43]    | |                    ^^^^^ expected one of 8 possible tokens here
[00:59:43] LL | | }
[00:59:43]    | |_- in this expansion of `pong!`
[00:59:43] ...
[00:59:43] LL |       pong!();
[00:59:43] 
[00:59:43] 
[00:59:43] error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `error`
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL | / macro_rules! pong {
[00:59:43] LL | |     () => { syntax error };
[00:59:43]    | |                    ^^^^^ expected one of 8 possible tokens here
[00:59:43] LL | | }
[00:59:43]    | |_- in this expansion of `pong!`
[00:59:43] LL |       ping!();
[00:59:43]    |       -------- in this macro invocation
[00:59:43]    | 
[00:59:43]    | 
[00:59:43]   ::: <::ping::ping macros>:1:1
[00:59:43]    |
[00:59:43] LL |   (  )   =>    {     pong      !       (         )          ;           }
[00:59:43]    |   |                  |
[00:59:43]    |   |                  in this macro invocation
[00:59:43]    |   in this expansion of `ping!`
[00:59:43] 
[00:59:43] 
[00:59:43] error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `error`
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL | / macro_rules! pong {
[00:59:43] LL | |     () => { syntax error };
[00:59:43]    | |                    ^^^^^ expected one of 8 possible tokens here
[00:59:43] LL | | }
[00:59:43]    | |_- in this expansion of `pong!` (#5)
[00:59:43] LL |       deep!();
[00:59:43]    |       -------- in this macro invocation (#1)
[00:59:43]    | 
[00:59:43]    | 
[00:59:43]   ::: <::ping::deep macros>:1:1
[00:59:43]    |
[00:59:43] LL |   (  )   =>    {     foo      !       (         )          ;           }
[00:59:43]    |   |                  |
[00:59:43]    |   |                  in this macro invocation (#2)
[00:59:43]    |   |                  in this macro invocation (#2)
[00:59:43]    |   in this expansion of `deep!` (#1)
[00:59:43]    | 
[00:59:43]   ::: <::ping::foo macros>:1:1
[00:59:43]    |
[00:59:43] LL |   (  )   =>    {     bar      !       (         )          ;           }
[00:59:43]    |   |                  |
[00:59:43]    |   |                  in this macro invocation (#3)
[00:59:43]    |   in this expansion of `foo!` (#2)
[00:59:43]    | 
[00:59:43]    | 
[00:59:43]   ::: <::ping::bar macros>:1:1
[00:59:43]    |
[00:59:43] LL |   (  )   =>    {     ping      !       (         )          ;           }
[00:59:43]    |   |                  |
[00:59:43]    |   |                  in this macro invocation (#4)
[00:59:43]    |   in this expansion of `bar!` (#3)
[00:59:43]    | 
[00:59:43]    | 
[00:59:43]   ::: <::ping::ping macros>:1:1
[00:59:43]    |
[00:59:43] LL |   (  )   =>    {     pong      !       (         )          ;           }
[00:59:43]    |   |                  |
[00:59:43]    |   |                  in this macro invocation (#5)
[00:59:43]    |   in this expansion of `ping!` (#4)
[00:59:43] 
---
[00:59:43] 
[00:59:43] ---- [ui] ui/macros/nonterminal-matching.rs stdout ----
[00:59:43] diff of stderr:
[00:59:43] 
[00:59:43] - error: no rules expected the token `enum E { }`
[00:59:43] + error: no rules expected the token `enum E {  }`
[00:59:43] 3    |
[00:59:43] 3    |
[00:59:43] 4 LL |     macro n(a $nt_item b) {
[00:59:43] 
[00:59:43] The actual stderr differed from the expected stderr.
[00:59:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/nonterminal-matching/nonterminal-matching.stderr
[00:59:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/nonterminal-matching/nonterminal-matching.stderr
[00:59:43] To update references, rerun the tests and pass the `--bless` flag
[00:59:43] To only update this specific test, also pass `--test-args macros/nonterminal-matching.rs`
[00:59:43] error: 1 errors occurred comparing output.
[00:59:43] status: exit code: 1
[00:59:43] status: exit code: 1
[00:59:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/nonterminal-matching.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/nonterminal-matching" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/nonterminal-matching/auxiliary" "-A" "unused"
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] stderr:
[00:59:43] stderr:
[00:59:43] ------------------------------------------
[00:59:43] error: no rules expected the token `enum E {  }`
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |     macro n(a $nt_item b) {
[00:59:43]    |     --------------------- when calling this macro
[00:59:43] ...
[00:59:43] LL |     n!(a $nt_item b); //~ ERROR no rules expected the token `enum E { }`
[00:59:43]    |          ^^^^^^^^ no rules expected this token in macro call
[00:59:43] ...
[00:59:43] LL | complex_nonterminal!(enum E {});
[00:59:43]    | -------------------------------- in this macro invocation
[00:59:43] error: aborting due to previous error
[00:59:43] 
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] 
[00:59:43] ---- [ui] ui/macros/trace_faulty_macros.rs stdout ----
[00:59:43] diff of stderr:
[00:59:43] 
[00:59:43] 17    |     ^^^^^^^^^^^^^^^^^^^
[00:59:43] 18    |
[00:59:43] 19    = note: expanding `my_faulty_macro! {  }`
[00:59:43] -    = note: to `my_faulty_macro ! ( bcd ) ;`
[00:59:43] +    = note: to `my_faulty_macro !  (   bcd    )     ;`
[00:59:43] 21    = note: expanding `my_faulty_macro! { bcd }`
[00:59:43] 22 
[00:59:43] 23 error: recursion limit reached while expanding the macro `my_recursive_macro`
[00:59:43] 38    |     ^^^^^^^^^^^^^^^^^^^^^^
[00:59:43] 39    |
[00:59:43] 39    |
[00:59:43] 40    = note: expanding `my_recursive_macro! {  }`
[00:59:43] -    = note: to `my_recursive_macro ! (  ) ;`
[00:59:43] +    = note: to `my_recursive_macro !  (    )     ;`
[00:59:43] 42    = note: expanding `my_recursive_macro! {  }`
[00:59:43] -    = note: to `my_recursive_macro ! (  ) ;`
[00:59:43] +    = note: to `my_recursive_macro !  (    )     ;`
[00:59:43] 44    = note: expanding `my_recursive_macro! {  }`
[00:59:43] -    = note: to `my_recursive_macro ! (  ) ;`
[00:59:43] +    = note: to `my_recursive_macro !  (    )     ;`
[00:59:43] 46    = note: expanding `my_recursive_macro! {  }`
[00:59:43] -    = note: to `my_recursive_macro ! (  ) ;`
[00:59:43] +    = note: to `my_recursive_macro !  (    )     ;`
[00:59:43] 48    = note: expanding `my_recursive_macro! {  }`
[00:59:43] -    = note: to `my_recursive_macro ! (  ) ;`
[00:59:43] +    = note: to `my_recursive_macro !  (    )     ;`
[00:59:43] 51 error: aborting due to 2 previous errors
[00:59:43] 52 
[00:59:43] 
[00:59:43] 
[00:59:43] 
[00:59:43] The actual stderr differed from the expected stderr.
[00:59:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/trace_faulty_macros/trace_faulty_macros.stderr
[00:59:43] To update references, rerun the tests and pass the `--bless` flag
[00:59:43] To only update this specific test, also pass `--test-args macros/trace_faulty_macros.rs`
[00:59:43] error: 1 errors occurred comparing output.
[00:59:43] status: exit code: 1
[00:59:43] status: exit code: 1
[00:59:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/trace_faulty_macros.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/trace_faulty_macros" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "trace-macros" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/trace_faulty_macros/auxiliary" "-A" "unused"
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] stderr:
[00:59:43] stderr:
[00:59:43] ------------------------------------------
[00:59:43] error: no rules expected the token `bcd`
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL | macro_rules! my_faulty_macro {
[00:59:43]    | ---------------------------- when calling this macro
[00:59:43] LL |     () => {
[00:59:43] LL |         my_faulty_macro!(bcd); //~ ERROR no rules
[00:59:43]    |                          ^^^ no rules expected this token in macro call
[00:59:43] ...
[00:59:43] LL |     my_faulty_macro!();
[00:59:43] 
[00:59:43] note: trace_macro
[00:59:43]   --> /checkout/src/test/ui/macros/trace_faulty_macros.rs:33:5
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |     my_faulty_macro!();
[00:59:43]    |
[00:59:43]    |
[00:59:43]    = note: expanding `my_faulty_macro! {  }`
[00:59:43]    = note: to `my_faulty_macro !  (   bcd    )     ;`
[00:59:43]    = note: expanding `my_faulty_macro! { bcd }`
[00:59:43] 
[00:59:43] error: recursion limit reached while expanding the macro `my_recursive_macro`
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |         my_recursive_macro!(); //~ ERROR recursion limit
[00:59:43] ...
[00:59:43] ...
[00:59:43] LL |     my_recursive_macro!();
[00:59:43]    |
[00:59:43]    |
[00:59:43]    = help: consider adding a `#![recursion_limit="8"]` attribute to your crate
[00:59:43] note: trace_macro
[00:59:43]   --> /checkout/src/test/ui/macros/trace_faulty_macros.rs:34:5
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |     my_recursive_macro!();
[00:59:43]    |
[00:59:43]    |
[00:59:43]    = note: expanding `my_recursive_macro! {  }`
[00:59:43]    = note: to `my_recursive_macro !  (    )     ;`
[00:59:43]    = note: expanding `my_recursive_macro! {  }`
[00:59:43]    = note: to `my_recursive_macro !  (    )     ;`
[00:59:43]    = note: expanding `my_recursive_macro! {  }`
[00:59:43]    = note: to `my_recursive_macro !  (    )     ;`
[00:59:43]    = note: expanding `my_recursive_macro! {  }`
[00:59:43]    = note: to `my_recursive_macro !  (    )     ;`
[00:59:43]    = note: expanding `my_recursive_macro! {  }`
[00:59:43]    = note: to `my_recursive_macro !  (    )     ;`
[00:59:43] error: aborting due to 2 previous errors
[00:59:43] 
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] 
[00:59:43] ---- [ui] ui/macros/trace-macro.rs stdout ----
[00:59:43] diff of stderr:
[00:59:43] 
[00:59:43] 5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:43] 6    |
[00:59:43] 7    = note: expanding `println! { "Hello, World!" }`
[00:59:43] -    = note: to `{ $crate :: io :: _print ( format_args_nl ! ( "Hello, World!" ) ) ; }`
[00:59:43] +    = note: to `{ $crate  ::   io    ::     _print      (       format_args_nl        !         (          "Hello, World!"           )            )             ;              }`
[00:59:43] 10 
[00:59:43] 
[00:59:43] 
[00:59:43] The actual stderr differed from the expected stderr.
[00:59:43] The actual stderr differed from the expected stderr.
[00:59:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/trace-macro/trace-macro.stderr
[00:59:43] To update references, rerun the tests and pass the `--bless` flag
[00:59:43] To only update this specific test, also pass `--test-args macros/trace-macro.rs`
[00:59:43] error: 1 errors occurred comparing output.
[00:59:43] status: exit code: 0
[00:59:43] status: exit code: 0
[00:59:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/trace-macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/trace-macro" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "trace-macros" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/trace-macro/auxiliary" "-A" "unused"
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] stderr:
[00:59:43] stderr:
[00:59:43] ------------------------------------------
[00:59:43] note: trace_macro
[00:59:43]   --> /checkout/src/test/ui/macros/trace-macro.rs:5:5
[00:59:43]    |
[00:59:43] LL |     println!("Hello, World!");
[00:59:43]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:43]    |
[00:59:43]    = note: expanding `println! { "Hello, World!" }`
[00:59:43]    = note: to `{ $crate  ::   io    ::     _print      (       format_args_nl        !         (          "Hello, World!"           )            )             ;              }`
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] 
[00:59:43] 
[00:59:43] ---- [ui] ui/malformed/malformed-interpolated.rs stdout ----
[00:59:43] diff of stderr:
[00:59:43] 
[00:59:43] 15 LL | check!(-0); // ERROR, see above
[00:59:43] 16    | ----------- in this macro invocation
[00:59:43] - error: unexpected token: `0 + 0`
[00:59:43] - error: unexpected token: `0 + 0`
[00:59:43] + error: unexpected token: `0 +  0`
[00:59:43] 20    |
[00:59:43] 20    |
[00:59:43] 21 LL |         #[rustc_dummy = $expr]
[00:59:43] 
[00:59:43] The actual stderr differed from the expected stderr.
[00:59:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/malformed/malformed-interpolated/malformed-interpolated.stderr
[00:59:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/malformed/malformed-interpolated/malformed-interpolated.stderr
[00:59:43] To update references, rerun the tests and pass the `--bless` flag
[00:59:43] To only update this specific test, also pass `--test-args malformed/malformed-interpolated.rs`
[00:59:43] error: 1 errors occurred comparing output.
[00:59:43] status: exit code: 1
[00:59:43] status: exit code: 1
[00:59:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/malformed/malformed-interpolated.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/malformed/malformed-interpolated" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/malformed/malformed-interpolated/auxiliary" "-A" "unused"
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] stderr:
[00:59:43] stderr:
[00:59:43] ------------------------------------------
[00:59:43] error: suffixed literals are not allowed in attributes
[00:59:43]   --> /checkout/src/test/ui/malformed/malformed-interpolated.rs:13:8
[00:59:43]    |
[00:59:43] LL | check!(0u8); //~ ERROR suffixed literals are not allowed in attributes
[00:59:43]    |
[00:59:43]    |
[00:59:43]    = help: instead of using a suffixed literal (1u8, 1.0f32, etc.), use an unsuffixed version (1, 1.0, etc.).
[00:59:43] error: unexpected token: `-0`
[00:59:43]   --> /checkout/src/test/ui/malformed/malformed-interpolated.rs:5:25
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |         #[rustc_dummy = $expr] //~ ERROR unexpected token: `-0`
[00:59:43] ...
[00:59:43] ...
[00:59:43] LL | check!(-0); // ERROR, see above
[00:59:43]    | ----------- in this macro invocation
[00:59:43] 
[00:59:43] error: unexpected token: `0 +  0`
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |         #[rustc_dummy = $expr] //~ ERROR unexpected token: `-0`
[00:59:43] ...
[00:59:43] ...
[00:59:43] LL | check!(0 + 0); // ERROR, see above
[00:59:43]    | -------------- in this macro invocation
[00:59:43] error: aborting due to 3 previous errors
[00:59:43] 
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] 
[00:59:43] ---- [ui] ui/missing/missing-block-hint.rs stdout ----
[00:59:43] diff of stderr:
[00:59:43] 
[00:59:43] 15    |             ^^^-
[00:59:43] 16    |             |
[00:59:43] 17    |             expected `{`
[00:59:43] -    |             help: try placing this code inside a block: `{ bar; }`
[00:59:43] +    |             help: try placing this code inside a block: `{ bar ;  }`
[00:59:43] 20 error: aborting due to 2 previous errors
[00:59:43] 21 
[00:59:43] 
[00:59:43] 
[00:59:43] 
[00:59:43] The actual stderr differed from the expected stderr.
[00:59:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing/missing-block-hint/missing-block-hint.stderr
[00:59:43] To update references, rerun the tests and pass the `--bless` flag
[00:59:43] To only update this specific test, also pass `--test-args missing/missing-block-hint.rs`
[00:59:43] error: 1 errors occurred comparing output.
[00:59:43] status: exit code: 1
[00:59:43] status: exit code: 1
[00:59:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/missing/missing-block-hint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing/missing-block-hint" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing/missing-block-hint/auxiliary" "-A" "unused"
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] stderr:
[00:59:43] stderr:
[00:59:43] ------------------------------------------
[00:59:43] error: expected `{`, found `=>`
[00:59:43]   --> /checkout/src/test/ui/missing/missing-block-hint.rs:3:18
[00:59:43]    |
[00:59:43] LL |         if (foo) => {} //~ ERROR expected `{`, found `=>`
[00:59:43]    |         --       ^^ expected `{`
[00:59:43]    |         |
[00:59:43]    |         this `if` statement has a condition, but no block
[00:59:43] error: expected `{`, found `bar`
[00:59:43]   --> /checkout/src/test/ui/missing/missing-block-hint.rs:7:13
[00:59:43]    |
[00:59:43] LL |         if (foo)
[00:59:43] LL |         if (foo)
[00:59:43]    |         -- this `if` statement has a condition, but no block
[00:59:43] LL |             bar; //~ ERROR expected `{`, found `bar`
[00:59:43]    |             ^^^-
[00:59:43]    |             expected `{`
[00:59:43]    |             expected `{`
[00:59:43]    |             help: try placing this code inside a block: `{ bar ;  }`
[00:59:43] error: aborting due to 2 previous errors
[00:59:43] 
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] 
[00:59:43] ---- [ui] ui/parser/trait-object-bad-parens.rs stdout ----
[00:59:43] diff of stderr:
[00:59:43] 
[00:59:43] 4 LL |     let _: Box<((Auto)) + Auto>;
[00:59:43] 6 
[00:59:43] 6 
[00:59:43] - error[E0178]: expected a path on the left-hand side of `+`, not `(Auto + Auto)`
[00:59:43] + error[E0178]: expected a path on the left-hand side of `+`, not `(Auto + Auto )`
[00:59:43] 9    |
[00:59:43] 9    |
[00:59:43] 10 LL |     let _: Box<(Auto + Auto) + Auto>;
[00:59:43] 
[00:59:43] The actual stderr differed from the expected stderr.
[00:59:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/trait-object-bad-parens/trait-object-bad-parens.stderr
[00:59:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/trait-object-bad-parens/trait-object-bad-parens.stderr
[00:59:43] To update references, rerun the tests and pass the `--bless` flag
[00:59:43] To only update this specific test, also pass `--test-args parser/trait-object-bad-parens.rs`
[00:59:43] error: 1 errors occurred comparing output.
[00:59:43] status: exit code: 1
[00:59:43] status: exit code: 1
[00:59:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/trait-object-bad-parens.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/trait-object-bad-parens" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/trait-object-bad-parens/auxiliary" "-A" "unused"
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] stderr:
[00:59:43] stderr:
[00:59:43] ------------------------------------------
[00:59:43] error[E0178]: expected a path on the left-hand side of `+`, not `((Auto))`
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |     let _: Box<((Auto)) + Auto>;
[00:59:43] 
[00:59:43] 
[00:59:43] error[E0178]: expected a path on the left-hand side of `+`, not `(Auto + Auto )`
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |     let _: Box<(Auto + Auto) + Auto>;
[00:59:43] 
[00:59:43] 
[00:59:43] error[E0178]: expected a path on the left-hand side of `+`, not `(Auto)`
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |     let _: Box<(Auto +) + Auto>;
[00:59:43] 
[00:59:43] 
[00:59:43] error[E0178]: expected a path on the left-hand side of `+`, not `(dyn Auto)`
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |     let _: Box<(dyn Auto) + Auto>;
[00:59:43] 
[00:59:43] error: aborting due to 4 previous errors
[00:59:43] 
[00:59:43] For more information about this error, try `rustc --explain E0178`.
[00:59:43] For more information about this error, try `rustc --explain E0178`.
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] 
[00:59:43] ---- [ui] ui/pattern/const-pat-ice.rs stdout ----
[00:59:43] diff of stderr:
[00:59:43] 
[00:59:43] - thread 'rustc' panicked at 'assertion failed: rows.iter().all(|r| r.len() == v.len())', src/librustc_mir/hair/pattern/_match.rs:LL:CC
[00:59:43] + thread 'rustc' panicked at 'assertion failed: rows.iter().all(|r| r . len ( )  ==   v   .   len   (   )   )', src/librustc_mir/hair/pattern/_match.rs:LL:CC
[00:59:43] 3 
[00:59:43] 4 error: internal compiler error: unexpected panic
[00:59:43] 
[00:59:43] 
[00:59:43] 
[00:59:43] The actual stderr differed from the expected stderr.
[00:59:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/const-pat-ice/const-pat-ice.stderr
[00:59:43] To update references, rerun the tests and pass the `--bless` flag
[00:59:43] To only update this specific test, also pass `--test-args pattern/const-pat-ice.rs`
[00:59:43] error: 1 errors occurred comparing output.
[00:59:43] status: exit code: 101
[00:59:43] status: exit code: 101
[00:59:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/const-pat-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/const-pat-ice" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/const-pat-ice/auxiliary" "-A" "unused"
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] stderr:
[00:59:43] stderr:
[00:59:43] ------------------------------------------
[00:59:43] thread 'rustc' panicked at 'assertion failed: rows.iter().all(|r| r . len ( )  ==   v   .   len   (   )   )', src/librustc_mir/hair/pattern/_match.rs:1088:5
[00:59:43] 
[00:59:43] error: internal compiler error: unexpected panic
[00:59:43] 
[00:59:43] note: the compiler unexpectedly panicked. this is a bug.
[00:59:43] note: the compiler unexpectedly panicked. this is a bug.
[00:59:43] 
[00:59:43] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:59:43] 
[00:59:43] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[00:59:43] 
[00:59:43] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] 
[00:59:43] 
[00:59:43] ---- [ui] ui/proc-macro/attr-stmt-expr.rs stdout ----
[00:59:43] diff of stderr:
[00:59:43] 
[00:59:43] 16    = note: for more information, see https://github.com/rust-lang/rust/issues/15701
[00:59:43] 17    = help: add #![feature(stmt_expr_attributes)] to the crate attributes to enable
[00:59:43] 18 
[00:59:43] - error: aborting due to 2 previous errors
[00:59:43] + error: custom attribute panicked
[00:59:43] +    |
[00:59:43] + LL |     #[expect_print_expr]
[00:59:43] +    |     ^^^^^^^^^^^^^^^^^^^^
[00:59:43] +    |
[00:59:43] +    |
[00:59:43] +    = help: message: assertion failed: `(left == right)`
[00:59:43] +              left: `"println!(\"{}\" ,  string  )"`,
[00:59:43] +             right: `"println!(\"{}\" , string)"`
[00:59:43] + error: aborting due to 3 previous errors
[00:59:43] 20 
[00:59:43] 21 For more information about this error, try `rustc --explain E0658`.
[00:59:43] 22 
[00:59:43] 22 
[00:59:43] 
[00:59:43] 
[00:59:43] The actual stderr differed from the expected stderr.
[00:59:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attr-stmt-expr/attr-stmt-expr.stderr
[00:59:43] To update references, rerun the tests and pass the `--bless` flag
[00:59:43] To only update this specific test, also pass `--test-args proc-macro/attr-stmt-expr.rs`
[00:59:43] error: 1 errors occurred comparing output.
[00:59:43] status: exit code: 1
[00:59:43] status: exit code: 1
[00:59:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/attr-stmt-expr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attr-stmt-expr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attr-stmt-expr/auxiliary" "-A" "unused"
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] stderr:
---
[00:59:43]    |
[00:59:43] LL |     #[expect_print_expr]
[00:59:43]    |     ^^^^^^^^^^^^^^^^^^^^
[00:59:43]    |
[00:59:43]    = help: message: assertion failed: `(left == right)`
[00:59:43]              left: `"println!(\"{}\" ,  string  )"`,
[00:59:43]             right: `"println!(\"{}\" , string)"`
[00:59:43] error: aborting due to 3 previous errors
[00:59:43] 
[00:59:43] For more information about this error, try `rustc --explain E0658`.
[00:59:43] 
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] 
[00:59:43] ---- [ui] ui/proc-macro/attribute-spans-preserved.rs stdout ----
[00:59:43] diff of stdout:
[00:59:43] 
[00:59:43] - fn main (  ) { let y : u32 = "z" ; { let x : u32 = "y" ; } }
[00:59:43] + fn main  (    )     {      let       y        :         u32          =           "z"            ;             {              let               x                :                 u32                  =                   "y"                    ;                     }                      }
[00:59:43] 
[00:59:43] 
[00:59:43] The actual stdout differed from the expected stdout.
[00:59:43] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attribute-spans-preserved/attribute-spans-preserved.stdout
[00:59:43] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attribute-spans-preserved/attribute-spans-preserved.stdout
[00:59:43] To update references, rerun the tests and pass the `--bless` flag
[00:59:43] To only update this specific test, also pass `--test-args proc-macro/attribute-spans-preserved.rs`
[00:59:43] error: 1 errors occurred comparing output.
[00:59:43] status: exit code: 1
[00:59:43] status: exit code: 1
[00:59:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/attribute-spans-preserved.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attribute-spans-preserved" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attribute-spans-preserved/auxiliary" "-A" "unused"
[00:59:43] ------------------------------------------
[00:59:43] ------------------------------------------
[00:59:43] fn main  (    )     {      let       y        :         u32          =           "z"            ;             {              let               x                :                 u32                  =                   "y"                    ;                     }                      }
[00:59:43] ------------------------------------------
[00:59:43] stderr:
[00:59:43] ------------------------------------------
[00:59:43] error[E0308]: mismatched types
[00:59:43] error[E0308]: mismatched types
[00:59:43]   --> /checkout/src/test/ui/proc-macro/attribute-spans-preserved.rs:7:23
[00:59:43]    |
[00:59:43] LL | #[ foo ( let y: u32 = "z"; ) ] //~ ERROR: mismatched types
[00:59:43]    |
[00:59:43]    = note: expected type `u32`
[00:59:43]               found type `&'static str`
[00:59:43] 
[00:59:43] 
[00:59:43] error[E0308]: mismatched types
[00:59:43]   --> /checkout/src/test/ui/proc-macro/attribute-spans-preserved.rs:8:23
[00:59:43]    |
[00:59:43] LL | #[ bar { let x: u32 = "y"; } ] //~ ERROR: mismatched types
[00:59:43]    |
[00:59:43]    = note: expected type `u32`
[00:59:43]               found type `&'static str`
[00:59:43] 
---
[00:59:43] 
[00:59:43] ---- [ui] ui/proc-macro/dollar-crate-issue-57089.rs stdout ----
[00:59:43] diff of stdout:
[00:59:43] 
[00:59:43] - PRINT-BANG INPUT (DISPLAY): struct M ( $crate :: S ) ;
[00:59:43] + PRINT-BANG INPUT (DISPLAY): struct M  (   $crate    ::     S      )       ;
[00:59:43] 2 PRINT-BANG INPUT (DEBUG): TokenStream [
[00:59:43] 4         ident: "struct",
[00:59:43] 
[00:59:43] 39     },
[00:59:43] 40 ]
[00:59:43] 40 ]
[00:59:43] 41 PRINT-ATTR INPUT (DISPLAY): struct A(crate::S);
[00:59:43] - PRINT-ATTR RE-COLLECTED (DISPLAY): struct A ( $crate :: S ) ;
[00:59:43] + PRINT-ATTR RE-COLLECTED (DISPLAY): struct A  (   $crate    ::     S      )       ;
[00:59:43] 43 PRINT-ATTR INPUT (DEBUG): TokenStream [
[00:59:43] 45         ident: "struct",
[00:59:43] 
[00:59:43] 
[00:59:43] The actual stdout differed from the expected stdout.
[00:59:43] The actual stdout differed from the expected stdout.
[00:59:43] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate-issue-57089/dollar-crate-issue-57089.stdout
[00:59:43] To update references, rerun the tests and pass the `--bless` flag
[00:59:43] To only update this specific test, also pass `--test-args proc-macro/dollar-crate-issue-57089.rs`
[00:59:43] error: 1 errors occurred comparing output.
[00:59:43] status: exit code: 0
[00:59:43] status: exit code: 0
[00:59:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/dollar-crate-issue-57089.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate-issue-57089" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate-issue-57089/auxiliary" "-A" "unused"
[00:59:43] ------------------------------------------
[00:59:43] ------------------------------------------
[00:59:43] PRINT-BANG INPUT (DISPLAY): struct M  (   $crate    ::     S      )       ;
[00:59:43] PRINT-BANG INPUT (DEBUG): TokenStream [
[00:59:43]         ident: "struct",
[00:59:43]         ident: "struct",
[00:59:43]         span: #2 bytes(401..407),
[00:59:43]     Ident {
[00:59:43]         ident: "M",
[00:59:43]         ident: "M",
[00:59:43]         span: #2 bytes(408..409),
[00:59:43]     Group {
[00:59:43]         delimiter: Parenthesis,
[00:59:43]         delimiter: Parenthesis,
[00:59:43]         stream: TokenStream [
[00:59:43]             Ident {
[00:59:43]                 ident: "$crate",
[00:59:43]                 span: #2 bytes(410..416),
[00:59:43]             Punct {
[00:59:43]             Punct {
[00:59:43]                 ch: ':',
[00:59:43]                 spacing: Joint,
[00:59:43]                 span: #2 bytes(416..418),
[00:59:43]             Punct {
[00:59:43]             Punct {
[00:59:43]                 ch: ':',
[00:59:43]                 spacing: Alone,
[00:59:43]                 span: #2 bytes(416..418),
[00:59:43]             Ident {
[00:59:43]                 ident: "S",
[00:59:43]                 ident: "S",
[00:59:43]                 span: #2 bytes(418..419),
[00:59:43]         ],
[00:59:43]         ],
[00:59:43]         span: #2 bytes(409..420),
[00:59:43]     Punct {
[00:59:43]     Punct {
[00:59:43]         ch: ';',
[00:59:43]         spacing: Alone,
[00:59:43]         span: #2 bytes(420..421),
[00:59:43] ]
[00:59:43] ]
[00:59:43] PRINT-ATTR INPUT (DISPLAY): struct A(crate::S);
[00:59:43] PRINT-ATTR RE-COLLECTED (DISPLAY): struct A  (   $crate    ::     S      )       ;
[00:59:43] PRINT-ATTR INPUT (DEBUG): TokenStream [
[00:59:43]         ident: "struct",
[00:59:43]         ident: "struct",
[00:59:43]         span: #2 bytes(463..469),
[00:59:43]     Ident {
[00:59:43]         ident: "A",
[00:59:43]         ident: "A",
[00:59:43]         span: #2 bytes(470..471),
[00:59:43]     Group {
[00:59:43]         delimiter: Parenthesis,
[00:59:43]         delimiter: Parenthesis,
[00:59:43]         stream: TokenStream [
[00:59:43]             Ident {
[00:59:43]                 ident: "$crate",
[00:59:43]                 span: #2 bytes(472..478),
[00:59:43]             Punct {
[00:59:43]             Punct {
[00:59:43]                 ch: ':',
[00:59:43]                 spacing: Joint,
[00:59:43]                 span: #2 bytes(478..480),
[00:59:43]             Punct {
[00:59:43]             Punct {
[00:59:43]                 ch: ':',
[00:59:43]                 spacing: Alone,
[00:59:43]                 span: #2 bytes(478..480),
[00:59:43]             Ident {
[00:59:43]                 ident: "S",
[00:59:43]                 ident: "S",
[00:59:43]                 span: #2 bytes(480..481),
[00:59:43]         ],
[00:59:43]         ],
[00:59:43]         span: #2 bytes(471..482),
[00:59:43]     Punct {
[00:59:43]     Punct {
[00:59:43]         ch: ';',
[00:59:43]         spacing: Alone,
[00:59:43]         span: #2 bytes(482..483),
[00:59:43] ]
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] stderr:
---
[00:59:43] 
[00:59:43] ---- [ui] ui/proc-macro/dollar-crate.rs stdout ----
[00:59:43] diff of stdout:
[00:59:43] 
[00:59:43] - PRINT-BANG INPUT (DISPLAY): struct M ( $crate :: S ) ;
[00:59:43] + PRINT-BANG INPUT (DISPLAY): struct M  (   $crate    ::     S      )       ;
[00:59:43] 2 PRINT-BANG INPUT (DEBUG): TokenStream [
[00:59:43] 4         ident: "struct",
[00:59:43] 
[00:59:43] 39     },
[00:59:43] 40 ]
[00:59:43] 40 ]
[00:59:43] 41 PRINT-ATTR INPUT (DISPLAY): struct A(crate::S);
[00:59:43] - PRINT-ATTR RE-COLLECTED (DISPLAY): struct A ( $crate :: S ) ;
[00:59:43] + PRINT-ATTR RE-COLLECTED (DISPLAY): struct A  (   $crate    ::     S      )       ;
[00:59:43] 43 PRINT-ATTR INPUT (DEBUG): TokenStream [
[00:59:43] 45         ident: "struct",
[00:59:43] 
[00:59:43] 80     },
[00:59:43] 81 ]
[00:59:43] 81 ]
[00:59:43] 82 PRINT-DERIVE INPUT (DISPLAY): struct D(crate::S);
[00:59:43] - PRINT-DERIVE RE-COLLECTED (DISPLAY): struct D ( $crate :: S ) ;
[00:59:43] + PRINT-DERIVE RE-COLLECTED (DISPLAY): struct D  (   $crate    ::     S      )       ;
[00:59:43] 84 PRINT-DERIVE INPUT (DEBUG): TokenStream [
[00:59:43] 86         ident: "struct",
[00:59:43] 
[00:59:43] 
[00:59:43] 120         span: #2 bytes(LO..HI),
[00:59:43] 122 ]
[00:59:43] 122 ]
[00:59:43] - PRINT-BANG INPUT (DISPLAY): struct M ( $crate :: S ) ;
[00:59:43] + PRINT-BANG INPUT (DISPLAY): struct M  (   $crate    ::     S      )       ;
[00:59:43] 124 PRINT-BANG INPUT (DEBUG): TokenStream [
[00:59:43] 126         ident: "struct",
[00:59:43] 
[00:59:43] 161     },
[00:59:43] 162 ]
[00:59:43] 162 ]
[00:59:43] 163 PRINT-ATTR INPUT (DISPLAY): struct A(::dollar_crate_external::S);
[00:59:43] - PRINT-ATTR RE-COLLECTED (DISPLAY): struct A ( $crate :: S ) ;
[00:59:43] + PRINT-ATTR RE-COLLECTED (DISPLAY): struct A  (   $crate    ::     S      )       ;
[00:59:43] 165 PRINT-ATTR INPUT (DEBUG): TokenStream [
[00:59:43] 167         ident: "struct",
[00:59:43] 
[00:59:43] 202     },
[00:59:43] 203 ]
[00:59:43] 203 ]
[00:59:43] 204 PRINT-DERIVE INPUT (DISPLAY): struct D(::dollar_crate_external::S);
[00:59:43] - PRINT-DERIVE RE-COLLECTED (DISPLAY): struct D ( $crate :: S ) ;
[00:59:43] + PRINT-DERIVE RE-COLLECTED (DISPLAY): struct D  (   $crate    ::     S      )       ;
[00:59:43] 206 PRINT-DERIVE INPUT (DEBUG): TokenStream [
[00:59:43] 208         ident: "struct",
[00:59:43] 
[00:59:43] 
[00:59:43] The actual stdout differed from the expected stdout.
[00:59:43] The actual stdout differed from the expected stdout.
[00:59:43] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate/dollar-crate.stdout
[00:59:43] To update references, rerun the tests and pass the `--bless` flag
[00:59:43] To only update this specific test, also pass `--test-args proc-macro/dollar-crate.rs`
[00:59:43] error: 1 errors occurred comparing output.
[00:59:43] status: exit code: 1
[00:59:43] status: exit code: 1
[00:59:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/dollar-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate/auxiliary" "-A" "unused"
[00:59:43] ------------------------------------------
[00:59:43] ------------------------------------------
[00:59:43] PRINT-BANG INPUT (DISPLAY): struct M  (   $crate    ::     S      )       ;
[00:59:43] PRINT-BANG INPUT (DEBUG): TokenStream [
[00:59:43]         ident: "struct",
[00:59:43]         ident: "struct",
[00:59:43]         span: #2 bytes(491..497),
[00:59:43]     Ident {
[00:59:43]         ident: "M",
[00:59:43]         ident: "M",
[00:59:43]         span: #2 bytes(498..499),
[00:59:43]     Group {
[00:59:43]         delimiter: Parenthesis,
[00:59:43]         delimiter: Parenthesis,
[00:59:43]         stream: TokenStream [
[00:59:43]             Ident {
[00:59:43]                 ident: "$crate",
[00:59:43]                 span: #2 bytes(500..506),
[00:59:43]             Punct {
[00:59:43]             Punct {
[00:59:43]                 ch: ':',
[00:59:43]                 spacing: Joint,
[00:59:43]                 span: #2 bytes(506..508),
[00:59:43]             Punct {
[00:59:43]             Punct {
[00:59:43]                 ch: ':',
[00:59:43]                 spacing: Alone,
[00:59:43]                 span: #2 bytes(506..508),
[00:59:43]             Ident {
[00:59:43]                 ident: "S",
[00:59:43]                 ident: "S",
[00:59:43]                 span: #2 bytes(508..509),
[00:59:43]         ],
[00:59:43]         ],
[00:59:43]         span: #2 bytes(499..510),
[00:59:43]     Punct {
[00:59:43]     Punct {
[00:59:43]         ch: ';',
[00:59:43]         spacing: Alone,
[00:59:43]         span: #2 bytes(510..511),
[00:59:43] ]
[00:59:43] ]
[00:59:43] PRINT-ATTR INPUT (DISPLAY): struct A(crate::S);
[00:59:43] PRINT-ATTR RE-COLLECTED (DISPLAY): struct A  (   $crate    ::     S      )       ;
[00:59:43] PRINT-ATTR INPUT (DEBUG): TokenStream [
[00:59:43]         ident: "struct",
[00:59:43]         ident: "struct",
[00:59:43]         span: #2 bytes(565..571),
[00:59:43]     Ident {
[00:59:43]         ident: "A",
[00:59:43]         ident: "A",
[00:59:43]         span: #2 bytes(572..573),
[00:59:43]     Group {
[00:59:43]         delimiter: Parenthesis,
[00:59:43]         delimiter: Parenthesis,
[00:59:43]         stream: TokenStream [
[00:59:43]             Ident {
[00:59:43]                 ident: "$crate",
[00:59:43]                 span: #2 bytes(574..580),
[00:59:43]             Punct {
[00:59:43]             Punct {
[00:59:43]                 ch: ':',
[00:59:43]                 spacing: Joint,
[00:59:43]                 span: #2 bytes(580..582),
[00:59:43]             Punct {
[00:59:43]             Punct {
[00:59:43]                 ch: ':',
[00:59:43]                 spacing: Alone,
[00:59:43]                 span: #2 bytes(580..582),
[00:59:43]             Ident {
[00:59:43]                 ident: "S",
[00:59:43]                 ident: "S",
[00:59:43]                 span: #2 bytes(582..583),
[00:59:43]         ],
[00:59:43]         ],
[00:59:43]         span: #2 bytes(573..584),
[00:59:43]     Punct {
[00:59:43]     Punct {
[00:59:43]         ch: ';',
[00:59:43]         spacing: Alone,
[00:59:43]         span: #2 bytes(584..585),
[00:59:43] ]
[00:59:43] ]
[00:59:43] PRINT-DERIVE INPUT (DISPLAY): struct D(crate::S);
[00:59:43] PRINT-DERIVE RE-COLLECTED (DISPLAY): struct D  (   $crate    ::     S      )       ;
[00:59:43] PRINT-DERIVE INPUT (DEBUG): TokenStream [
[00:59:43]         ident: "struct",
[00:59:43]         ident: "struct",
[00:59:43]         span: #2 bytes(628..634),
[00:59:43]     Ident {
[00:59:43]         ident: "D",
[00:59:43]         ident: "D",
[00:59:43]         span: #2 bytes(635..636),
[00:59:43]     Group {
[00:59:43]         delimiter: Parenthesis,
[00:59:43]         delimiter: Parenthesis,
[00:59:43]         stream: TokenStream [
[00:59:43]             Ident {
[00:59:43]                 ident: "$crate",
[00:59:43]                 span: #2 bytes(637..643),
[00:59:43]             Punct {
[00:59:43]             Punct {
[00:59:43]                 ch: ':',
[00:59:43]                 spacing: Joint,
[00:59:43]                 span: #2 bytes(643..645),
[00:59:43]             Punct {
[00:59:43]             Punct {
[00:59:43]                 ch: ':',
[00:59:43]                 spacing: Alone,
[00:59:43]                 span: #2 bytes(643..645),
[00:59:43]             Ident {
[00:59:43]                 ident: "S",
[00:59:43]                 ident: "S",
[00:59:43]                 span: #2 bytes(645..646),
[00:59:43]         ],
[00:59:43]         ],
[00:59:43]         span: #2 bytes(636..647),
[00:59:43]     Punct {
[00:59:43]     Punct {
[00:59:43]         ch: ';',
[00:59:43]         spacing: Alone,
[00:59:43]         span: #2 bytes(647..648),
[00:59:43] ]
[00:59:43] ]
[00:59:43] PRINT-BANG INPUT (DISPLAY): struct M  (   $crate    ::     S      )       ;
[00:59:43] PRINT-BANG INPUT (DEBUG): TokenStream [
[00:59:43]         ident: "struct",
[00:59:43]         ident: "struct",
[00:59:43]         span: #10 bytes(8242703..8242709),
[00:59:43]     Ident {
[00:59:43]         ident: "M",
[00:59:43]         ident: "M",
[00:59:43]         span: #10 bytes(8242713..8242714),
[00:59:43]     Group {
[00:59:43]         delimiter: Parenthesis,
[00:59:43]         delimiter: Parenthesis,
[00:59:43]         stream: TokenStream [
[00:59:43]             Ident {
[00:59:43]                 ident: "$crate",
[00:59:43]                 span: #10 bytes(8242726..8242739),
[00:59:43]             Punct {
[00:59:43]             Punct {
[00:59:43]                 ch: ':',
[00:59:43]                 spacing: Joint,
[00:59:43]                 span: #10 bytes(8242747..8242749),
[00:59:43]             Punct {
[00:59:43]             Punct {
[00:59:43]                 ch: ':',
[00:59:43]                 spacing: Alone,
[00:59:43]                 span: #10 bytes(8242747..8242749),
[00:59:43]             Ident {
[00:59:43]                 ident: "S",
[00:59:43]                 ident: "S",
[00:59:43]                 span: #10 bytes(8242758..8242759),
[00:59:43]         ],
[00:59:43]         ],
[00:59:43]         span: #10 bytes(8242719..8242770),
[00:59:43]     Punct {
[00:59:43]     Punct {
[00:59:43]         ch: ';',
[00:59:43]         spacing: Alone,
[00:59:43]         span: #10 bytes(8242781..8242782),
[00:59:43] ]
[00:59:43] ]
[00:59:43] PRINT-ATTR INPUT (DISPLAY): struct A(::dollar_crate_external::S);
[00:59:43] PRINT-ATTR RE-COLLECTED (DISPLAY): struct A  (   $crate    ::     S      )       ;
[00:59:43] PRINT-ATTR INPUT (DEBUG): TokenStream [
[00:59:43]         ident: "struct",
[00:59:43]         ident: "struct",
[00:59:43]         span: #10 bytes(8242883..8242889),
[00:59:43]     Ident {
[00:59:43]         ident: "A",
[00:59:43]         ident: "A",
[00:59:43]         span: #10 bytes(8242907..8242908),
[00:59:43]     Group {
[00:59:43]         delimiter: Parenthesis,
[00:59:43]         delimiter: Parenthesis,
[00:59:43]         stream: TokenStream [
[00:59:43]             Ident {
[00:59:43]                 ident: "$crate",
[00:59:43]                 span: #10 bytes(8242929..8242936),
[00:59:43]             Punct {
[00:59:43]             Punct {
[00:59:43]                 ch: ':',
[00:59:43]                 spacing: Joint,
[00:59:43]                 span: #10 bytes(8242938..8242940),
[00:59:43]             Punct {
[00:59:43]             Punct {
[00:59:43]                 ch: ':',
[00:59:43]                 spacing: Alone,
[00:59:43]                 span: #10 bytes(8242938..8242940),
[00:59:43]             Ident {
[00:59:43]                 ident: "S",
[00:59:43]                 ident: "S",
[00:59:43]                 span: #10 bytes(8242943..8242944),
[00:59:43]         ],
[00:59:43]         ],
[00:59:43]         span: #10 bytes(8242927..8242949),
[00:59:43]     Punct {
[00:59:43]     Punct {
[00:59:43]         ch: ';',
[00:59:43]         spacing: Alone,
[00:59:43]         span: #10 bytes(8242954..8242955),
[00:59:43] ]
[00:59:43] ]
[00:59:43] PRINT-DERIVE INPUT (DISPLAY): struct D(::dollar_crate_external::S);
[00:59:43] PRINT-DERIVE RE-COLLECTED (DISPLAY): struct D  (   $crate    ::     S      )       ;
[00:59:43] PRINT-DERIVE INPUT (DEBUG): TokenStream [
[00:59:43]         ident: "struct",
[00:59:43]         ident: "struct",
[00:59:43]         span: #10 bytes(8243047..8243053),
[00:59:43]     Ident {
[00:59:43]         ident: "D",
[00:59:43]         ident: "D",
[00:59:43]         span: #10 bytes(8243067..8243068),
[00:59:43]     Group {
[00:59:43]         delimiter: Parenthesis,
[00:59:43]         delimiter: Parenthesis,
[00:59:43]         stream: TokenStream [
[00:59:43]             Ident {
[00:59:43]                 ident: "$crate",
[00:59:43]                 span: #10 bytes(8243100..8243123),
[00:59:43]             Punct {
[00:59:43]             Punct {
[00:59:43]                 ch: ':',
[00:59:43]                 spacing: Joint,
[00:59:43]                 span: #10 bytes(8243141..8243143),
[00:59:43]             Punct {
[00:59:43]             Punct {
[00:59:43]                 ch: ':',
[00:59:43]                 spacing: Alone,
[00:59:43]                 span: #10 bytes(8243141..8243143),
[00:59:43]             Ident {
[00:59:43]                 ident: "S",
[00:59:43]                 ident: "S",
[00:59:43]                 span: #10 bytes(8243162..8243163),
[00:59:43]         ],
[00:59:43]         ],
[00:59:43]         span: #10 bytes(8243083..8243184),
[00:59:43]     Punct {
[00:59:43]     Punct {
[00:59:43]         ch: ';',
[00:59:43]         spacing: Alone,
[00:59:43]         span: #10 bytes(8243205..8243206),
[00:59:43] ]
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] stderr:
[00:59:43] stderr:
[00:59:43] ------------------------------------------
[00:59:43] error[E0428]: the name `D` is defined multiple times
[00:59:43]   --> /checkout/src/test/ui/proc-macro/dollar-crate.rs:26:13
[00:59:43]    |
[00:59:43] LL |             struct D($crate::S); //~ ERROR the name `D` is defined multiple times
[00:59:43]    |             |
[00:59:43]    |             |
[00:59:43]    |             `D` redefined here
[00:59:43]    |             previous definition of the type `D` here
[00:59:43] LL |     local!();
[00:59:43]    |     --------- in this macro invocation
[00:59:43]    |
[00:59:43]    |
[00:59:43]    = note: `D` must be defined only once in the type namespace of this module
[00:59:43] error[E0428]: the name `D` is defined multiple times
[00:59:43]   --> /checkout/src/test/ui/proc-macro/dollar-crate.rs:36:5
[00:59:43] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[00:59:43] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:59:43] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:59:43]    |
[00:59:43] LL |     dollar_crate_external::external!(); //~ ERROR the name `D` is defined multiple times
[00:59:43]    |     |
[00:59:43]    |     |
[00:59:43]    |     `D` redefined here
[00:59:43]    |     previous definition of the type `D` here
[00:59:43]    |
[00:59:43]    = note: `D` must be defined only once in the type namespace of this module
[00:59:43] 
[00:59:43] error: aborting due to 2 previous errors
[00:59:43] 
[00:59:43] For more information about this error, try `rustc --explain E0428`.
[00:59:43] For more information about this error, try `rustc --explain E0428`.
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] 
[00:59:43] ---- [ui] ui/qualified/qualified-path-params.rs stdout ----
[00:59:43] diff of stderr:
[00:59:43] 
[00:59:43] - error[E0533]: expected unit struct/variant or constant, found method `<<S as Tr>::A>::f<u8>`
[00:59:43] + error[E0533]: expected unit struct/variant or constant, found method `<<S as  Tr  >  ::  A  >  ::  f  <  u8  >`
[00:59:43] 3    |
[00:59:43] 3    |
[00:59:43] 4 LL |         <S as Tr>::A::f::<u8> => {}
[00:59:43] 
[00:59:43] The actual stderr differed from the expected stderr.
[00:59:43] The actual stderr differed from the expected stderr.
[00:59:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/qualified/qualified-path-params/qualified-path-params.stderr
[00:59:43] To update references, rerun the tests and pass the `--bless` flag
[00:59:43] To only update this specific test, also pass `--test-args qualified/qualified-path-params.rs`
[00:59:43] error: 1 errors occurred comparing output.
[00:59:43] status: exit code: 1
[00:59:43] status: exit code: 1
[00:59:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/qualified/qualified-path-params.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/qualified/qualified-path-params" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/qualified/qualified-path-params/auxiliary" "-A" "unused"
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] stderr:
[00:59:43] stderr:
[00:59:43] ------------------------------------------
[00:59:43] error[E0533]: expected unit struct/variant or constant, found method `<<S as  Tr  >  ::  A  >  ::  f  <  u8  >`
[00:59:43]   --> /checkout/src/test/ui/qualified/qualified-path-params.rs:20:9
[00:59:43]    |
[00:59:43] LL |         <S as Tr>::A::f::<u8> => {}
[00:59:43] 
[00:59:43] 
[00:59:43] error[E0029]: only char and numeric types are allowed in range patterns
[00:59:43]   --> /checkout/src/test/ui/qualified/qualified-path-params.rs:22:15
[00:59:43]    |
[00:59:43] LL |         0 ..= <S as Tr>::A::f::<u8> => {} //~ ERROR only char and numeric types are allowed in range
[00:59:43]    |               ^^^^^^^^^^^^^^^^^^^^^ ranges require char or numeric types
[00:59:43]    |
[00:59:43]    = note: start type: {integer}
[00:59:43]    = note: end type: fn() {S::f::<u8>}
[00:59:43] error: aborting due to 2 previous errors
[00:59:43] 
[00:59:43] For more information about this error, try `rustc --explain E0029`.
[00:59:43] 
---
[00:59:43] diff of stderr:
[00:59:43] 
[00:59:43] 2   --> $DIR/range-inclusive-pattern-precedence.rs:17:10
[00:59:43] 3    |
[00:59:43] 4 LL |         &10..=15 => {}
[00:59:43] -    |          ^^^^^^^ help: add parentheses to clarify the precedence: `(10 ..=15)`
[00:59:43] +    |          ^^^^^^^ help: add parentheses to clarify the precedence: `(10 ..= 15)`
[00:59:43] 6 
[00:59:43] 7 error: the range pattern here has ambiguous interpretation
[00:59:43] 
[00:59:43] 9    |
[00:59:43] 9    |
[00:59:43] 10 LL |         box 10..=15 => {}
[00:59:43] -    |             ^^^^^^^ help: add parentheses to clarify the precedence: `(10 ..=15)`
[00:59:43] +    |             ^^^^^^^ help: add parentheses to clarify the precedence: `(10 ..= 15)`
[00:59:43] 13 warning: `...` range patterns are deprecated
[00:59:43] 14   --> $DIR/range-inclusive-pattern-precedence.rs:14:9
[00:59:43] 
[00:59:43] 
[00:59:43] 
[00:59:43] The actual stderr differed from the expected stderr.
[00:59:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/range-inclusive-pattern-precedence/range-inclusive-pattern-precedence.stderr
[00:59:43] To update references, rerun the tests and pass the `--bless` flag
[00:59:43] To only update this specific test, also pass `--test-args range/range-inclusive-pattern-precedence.rs`
[00:59:43] error: 1 errors occurred comparing output.
[00:59:43] status: exit code: 1
[00:59:43] status: exit code: 1
[00:59:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/range/range-inclusive-pattern-precedence.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/range-inclusive-pattern-precedence" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/range-inclusive-pattern-precedence/auxiliary" "-A" "unused"
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] stderr:
[00:59:43] stderr:
[00:59:43] ------------------------------------------
[00:59:43] error: the range pattern here has ambiguous interpretation
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |         &10..=15 => {}
[00:59:43]    |          ^^^^^^^ help: add parentheses to clarify the precedence: `(10 ..= 15)`
[00:59:43] 
[00:59:43] error: the range pattern here has ambiguous interpretation
[00:59:43]    |
[00:59:43]    |
[00:59:43] LL |         box 10..=15 => {}
[00:59:43]    |             ^^^^^^^ help: add parentheses to clarify the precedence: `(10 ..= 15)`
[00:59:43] warning: `...` range patterns are deprecated
[00:59:43]   --> /checkout/src/test/ui/range/range-inclusive-pattern-precedence.rs:14:9
[00:59:43]    |
[00:59:43] LL |         &0...9 => {}
[00:59:43] LL |         &0...9 => {}
[00:59:43]    |         ^^^^^^ help: use `..=` for an inclusive range: `&(0..=9)`
[00:59:43] note: lint level defined here
[00:59:43]   --> /checkout/src/test/ui/range/range-inclusive-pattern-precedence.rs:9:9
[00:59:43]    |
[00:59:43] LL | #![warn(ellipsis_inclusive_range_patterns)]
---
[00:59:43] 
[00:59:43] ---- [ui] ui/rfc-2497-if-let-chains/ast-pretty-check.rs stdout ----
[00:59:43] diff of stdout:
[00:59:43] 
[00:59:43] 7 // compile-pass
[00:59:43] 8 // compile-flags: -Z unpretty=expanded
[00:59:43] 9 
[00:59:43] - fn main() { if let 0 = 1 { } }
[00:59:43] + fn main() { if   let  0  =   1    {     }      }
[00:59:43] 
[00:59:43] 
[00:59:43] The actual stdout differed from the expected stdout.
[00:59:43] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/ast-pretty-check/ast-pretty-check.stdout
[00:59:43] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/ast-pretty-check/ast-pretty-check.stdout
[00:59:43] To update references, rerun the tests and pass the `--bless` flag
[00:59:43] To only update this specific test, also pass `--test-args rfc-2497-if-let-chains/ast-pretty-check.rs`
[00:59:43] error: 1 errors occurred comparing output.
[00:59:43] status: exit code: 0
[00:59:43] status: exit code: 0
[00:59:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2497-if-let-chains/ast-pretty-check.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/ast-pretty-check" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unpretty=expanded" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/ast-pretty-check/auxiliary" "-A" "unused"
[00:59:43] ------------------------------------------
[00:59:43] #![feature(prelude_import)]
[00:59:43] #![feature(prelude_import)]
[00:59:43] #![no_std]
[00:59:43] #[prelude_import]
[00:59:43] use ::std::prelude::v1::*;
[00:59:43] #[macro_use]
[00:59:43] extern crate std;
[00:59:43] // compile-pass
[00:59:43] // compile-flags: -Z unpretty=expanded
[00:59:43] 
[00:59:43] fn main() { if   let  0  =   1    {     }      }
[00:59:43] ------------------------------------------
[00:59:43] stderr:
[00:59:43] ------------------------------------------
[00:59:43] 
---
[00:59:43] ------------------------------------------
[00:59:43] stderr:
[00:59:43] ------------------------------------------
[00:59:43] thread 'main' panicked at 'assertion failed: `(left == right)`
[00:59:43]   left: `[":22] Unit = Unit", ":23] a = Unit", ":29] Point{x: 42 ,  y  :   24   ,   } = Point {", "    x: 42,", "    y: 24,", "}", ":30] b = Point {", "    x: 42,", "    y: 24,", "}", ":38]", ":42] &a = NoCopy(", "    1337,", ")", ":42] dbg!(& a ) = NoCopy(", "    1337,", ")", ":47] f(&42) = 42", "before", ":52] { foo  +  =   1   ;    eprintln    !    (    \"before\"    )    ;     7331      } = 7331", ":60] (\"Yeah\",) = (", "    \"Yeah\",", ")", ":63] 1 = 1", ":63] 2 = 2", ":67] 1u8 = 1", ":67] 2u32 = 2", ":67] \"Yeah\" = \"Yeah\""]`,
[00:59:43]  right: `[":22] Unit = Unit", ":23] a = Unit", ":29] Point{x: 42, y: 24,} = Point {", "    x: 42,", "    y: 24,", "}", ":30] b = Point {", "    x: 42,", "    y: 24,", "}", ":38]", ":42] &a = NoCopy(", "    1337,", ")", ":42] dbg!(& a) = NoCopy(", "    1337,", ")", ":47] f(&42) = 42", "before", ":52] { foo += 1; eprintln!(\"before\"); 7331 } = 7331", ":60] (\"Yeah\",) = (", "    \"Yeah\",", ")", ":63] 1 = 1", ":63] 2 = 2", ":67] 1u8 = 1", ":67] 2u32 = 2", ":67] \"Yeah\" = \"Yeah\""]`', /checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:72:5
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] 
[00:59:43] 
[00:59:43] ---- [ui] ui/tuple/tuple-float-index.rs stdout ----
[00:59:43] diff of stderr:
[00:59:43] 
[00:59:43] 5    |     ------------^^^
[00:59:43] 6    |     |           |
[00:59:43] 7    |     |           unexpected token
[00:59:43] -    |     help: try parenthesizing the first index: `((1, (2, 3)).1).1`
[00:59:43] +    |     help: try parenthesizing the first index: `((1, ( 2 ,  3  )  )  .  1  )  .  1`
[00:59:43] 10 error: aborting due to previous error
[00:59:43] 11 
[00:59:43] 
[00:59:43] 
[00:59:43] 
[00:59:43] The actual stderr differed from the expected stderr.
[00:59:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tuple/tuple-float-index/tuple-float-index.stderr
[00:59:43] diff of fixed:
[00:59:43] 
[00:59:43] 1 // run-rustfix
[00:59:43] 2 
[00:59:43] 3 fn main () {
[00:59:43] -     ((1, (2, 3)).1).1; //~ ERROR unexpected token: `1.1`
[00:59:43] +     ((1, ( 2 ,  3  )  )  .  1  )  .  1; //~ ERROR unexpected token: `1.1`
[00:59:43] 6 
[00:59:43] 
[00:59:43] 
[00:59:43] The actual fixed differed from the expected fixed.
[00:59:43] The actual fixed differed from the expected fixed.
[00:59:43] Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tuple/tuple-float-index/tuple-float-index.fixed
[00:59:43] To update references, rerun the tests and pass the `--bless` flag
[00:59:43] To only update this specific test, also pass `--test-args tuple/tuple-float-index.rs`
[00:59:43] error: 2 errors occurred comparing output.
[00:59:43] status: exit code: 1
[00:59:43] status: exit code: 1
[00:59:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/tuple/tuple-float-index.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tuple/tuple-float-index" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tuple/tuple-float-index/auxiliary" "-A" "unused"
[00:59:43] ------------------------------------------
[00:59:43] 
[00:59:43] ------------------------------------------
[00:59:43] stderr:
[00:59:43] stderr:
[00:59:43] ------------------------------------------
[00:59:43] error: unexpected token: `1.1`
[00:59:43]   --> /checkout/src/test/ui/tuple/tuple-float-index.rs:4:17
[00:59:43]    |
[00:59:43] LL |     (1, (2, 3)).1.1; //~ ERROR unexpected token: `1.1`
[00:59:43]    |     |           |
[00:59:43]    |     |           unexpected token
[00:59:43]    |     |           unexpected token
[00:59:43]    |     help: try parenthesizing the first index: `((1, ( 2 ,  3  )  )  .  1  )  .  1`
[00:59:43] error: aborting due to previous error
[00:59:43] 
[00:59:43] 
[00:59:43] ------------------------------------------
---
[00:59:43] test result: FAILED. 5658 passed; 27 failed; 21 ignored; 0 measured; 0 filtered out
[00:59:43] 
[00:59:43] 
[00:59:43] 
[00:59:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:59:43] 
[00:59:43] 
[00:59:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:59:43] Build completed unsuccessfully in 0:54:50
---
travis_time:end:1cd8c464:start=1561505300114007132,finish=1561505300121383227,duration=7376095
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03b4b4d9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0add38a4
$ dmesg | grep -i kill

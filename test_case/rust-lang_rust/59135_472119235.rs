plain
travis_time:end:04cf50e0:start=1552409933166691199,finish=1552410011092996395,duration=77926305196
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:24] 
[01:07:24] running 5450 tests
[01:07:27] .................................................................................................... 100/5450
[01:07:29] ................................F................................................................... 200/5450
[01:07:35] .................................................................................................... 400/5450
[01:07:39] .................................................................................................... 500/5450
[01:07:42] .......................................i............................................................ 600/5450
[01:07:46] .................................................................................................... 700/5450
---
[01:10:35] ---- [ui] ui/async-fn-multiple-lifetimes.rs stdout ----
[01:10:35] diff of stderr:
[01:10:35] 
[01:10:35] 8    |
[01:10:35] 9    = help: `async fn` can only accept borrowed values with identical lifetimes
[01:10:35] 10 
[01:10:35] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[01:10:35] +    |
[01:10:35] +    |
[01:10:35] + LL |     _: impl for<'a> Add<&'a u8>,
[01:10:35] +    |
[01:10:35] +    = help: add #![feature(impl_trait_in_bindings)] to the crate attributes to enable
[01:10:35] + 
[01:10:35] + 
[01:10:35] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[01:10:35] +    |
[01:10:35] +    |
[01:10:35] + LL |     _: impl for<'b> Add<&'b u8>,
[01:10:35] +    |
[01:10:35] +    = help: add #![featureDIR/async-fn-multiple-lifetimes.rs:16:52
[01:10:35] +    |
[01:10:35] +    |
[01:10:35] + LL | async fn multiple_elided_lifetimes(_: &u8, _: &u8) {}
[01:10:35] +    |                                               ---  ^ lifetime `'static` required
[01:10:35] +    |                                               |
[01:10:35] +    |                                               help: add explicit lifetime `'static` to the type of `__arg1`: `&'static u8`
[01:10:35] + 
[01:10:35] + error[E0621]: explicit lifetime required in the type of `__arg0`
[01:10:35] +    |
[01:10:35] +    |
[01:10:35] + LL | async fn multiple_elided_lifetimes(_: &u8, _: &u8) {}
[01:10:35] +    |                                       ---          ^ lifetime `'static` required
[01:10:35] +    |                                       |
[01:10:35] +    |                                       help: add explicit lifetime `'static` to the type of `__arg0`: `&'static u8`
[01:10:35] + error: aborting due to 8 previous errors
[01:10:35] + 
[01:10:35] + Some errors occurred: E0106, E0562, E0621, E0623, E0707, E0709.
[01:10:35] 32 For more information about an error, try `rustc --explain E0106`.
[01:10:35] 32 For more information about an error, try `rustc --explain E0106`.
[01:10:35] 33 
[01:10:35] 
[01:10:35] 
[01:10:35] The actual stderr differed from the expected stderr.
[01:10:35] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-fn-multiple-lifetimes/async-fn-multiple-lifetimes.stderr
[01:10:35] To update references, rerun the tests and pass the `--bless` flag
[01:10:35] To only update this specific test, also pass `--test-args async-fn-multiple-lifetimes.rs`
[01:10:35] error: 1 errors occurred comparing output.
[01:10:35] status: exit code: 1
[01:10:35] status: exit code: 1
[01:10:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-fn-multiple-lifetimes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-fn-multiple-lifetimes/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-fn-multiple-lifetimes/auxiliary" "-A" "unused"
[01:10:35] ------------------------------------------
[01:10:35] 
[01:10:35] ------------------------------------------
[01:10:35] stderr:
[01:10:35] stderr:
[01:10:35] ------------------------------------------
[01:10:35] {"message":"multiple different lifetimes used in arguments of `async fn`","code":{"code":"E0709","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/async-fn-multiple-lifetimes.rs","byte_start":162,"byte_end":164,"line_start":7,"line_end":7,"column_start":47,"column_end":49,"is_primary":true,"text":[{"text":"async fn multiple_named_lifetimes<'a, 'b>(_: &'a u8, _: &'b u8) {}","highlight_start":47,"highlight_end":49}],"label":"first lifetime here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/async-fn-multiple-lifetimes.rs","byte_start":173,"byte_end":175,"line_start":7,"line_end":7,"column_start":58,"column_end":60,"is_primary":true,"text":[{"text":"async fn multiple_named_lifetimes<'a, 'b>(_: &'a u8, _: &'b u8) {}","highlight_start":58,"highlight_end":60}],"label":"different lifetime here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`async fn` can only accept borrowed values with identical lifetimes","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0709]: multiple different lifetimes used in arguments of `async fn`\n  --> /checkout/src/test/ui/async-fn-multiple-lifetimes.rs:7:47\n   |\nLL | async fn multiple_named_lifetimes<'a, 'b>(_: &'a u8, _: &'b u8) {}\n   |                                               ^^         ^^ different lifetime here\n   |                                               |\n   |                                               first lifetime here\n   |\n   = help: `async fn` can only accept borrowed values with identical lifetimes\n\n"}
[01:10:35] {"message":"`impl Trait` not allowed outside of function and inherent method return types","code":{"code":"E0562","explanation":"\nAbstract return types (written `impl Trait` for some trait `Trait`) are only\nallowed as function and inherent impl return types.\n\nErroneous code example:\n\n
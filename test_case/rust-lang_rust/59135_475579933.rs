plain
travis_time:end:2dc63ba0:start=1553248319249541091,finish=1553248321556995489,duration=2307454398
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
[01:13:15] 
[01:13:15] running 5479 tests
[01:13:19] .................................................................................................... 100/5479
[01:13:22] ..................................F................................................................. 200/5479
[01:13:29] .................................................................................................... 400/5479
[01:13:33] .................................................................................................... 500/5479
[01:13:37] ..........................................i......................................................... 600/5479
[01:13:42] .................................................................................................... 700/5479
---
[01:17:04] ---- [ui] ui/async-fn-multiple-lifetimes.rs stdout ----
[01:17:04] diff of stderr:
[01:17:04] 
[01:17:04] 26    |
[01:17:04] 27    = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `_` or `_`
[01:17:04] - error: aborting due to 3 previous errors
[01:17:04] + error[E0623]: lifetime mismatch
[01:17:04] +   --> $DIR/async-fn-multiple-lifetimes.rs:7:65
[01:17:04] +    |
[01:17:04] +    |
[01:17:04] + LL | async fn multiple_named_lifetimes<'a, 'b>(_: &'a u8, _: &'b u8) {}
[01:17:04] +    |                                                         ------  ^
[01:17:04] +    |                                                         |       ...but data is returned here
[01:17:04] +    |                                                         this parameter and the return type are declared with different lifetimes...
[01:17:04] 30 
[01:17:04] - Some errors occurred: E0106, E0707, E0709.
[01:17:04] - Some errors occurred: E0106, E0707, E0709.
[01:17:04] + error[E0309]: the parameter type `impl for<'b> Add<&'b u8>` may not live long enough
[01:17:04] +    |
[01:17:04] + LL | ) {}
[01:17:04] +    |   ^
[01:17:04] +    |
[01:17:04] +    |
[01:17:04] + note: ...so that the type `impl std::future::Future` will meet its required lifetime bounds
[01:17:04] +    |
[01:17:04] + LL | ) {}
[01:17:04] +    |   ^
[01:17:04] +    |   ^
[01:17:04] + help: consider adding an explicit lifetime bound  `'c` to `impl for<'b> Add<&'b u8>`...
[01:17:04] +    |
[01:17:04] + LL |     _: impl for<'b> Add<&'b u8> + 'c,
[01:17:04] + 
[01:17:04] + 
[01:17:04] + error[E0309]: the parameter type `impl for<'a> Add<&'a u8>` may not live long enough
[01:17:04] +    |
[01:17:04] + LL | ) {}
[01:17:04] +    |   ^
[01:17:04] +    |
[01:17:04] +    |
[01:17:04] + note: ...so that the type `impl std::future::Future` will meet its required lifetime bounds
[01:17:04] +    |
[01:17:04] + LL | ) {}
[01:17:04] +    |   ^
[01:17:04] +    |   ^
[01:17:04] + help: consider adding an explicit lifetime bound  `'c` to `impl for<'a> Add<&'a u8>`...
[01:17:04] +    |
[01:17:04] + LL |     _: impl for<'a> Add<&'a u8> + 'c,
[01:17:04] + 
[01:17:04] + error[E0621]: explicit lifetime required in parameter type
[01:17:04] +   --> $DIR/async-fn-multiple-lifetimes.rs:16:52
[01:17:04] +    |
[01:17:04] +    |
[01:17:04] + LL | async fn multiple_elided_lifetimes(_: &u8, _: &u8) {}
[01:17:04] +    |                                               ---  ^ lifetime `'static` required
[01:17:04] +    |                                               help: add explicit lifetime `'static` to type: `&'static u8`
[01:17:04] + 
[01:17:04] + error[E0621]: explicit lifetime required in parameter type
[01:17:04] +   --> $DIR/async-fn-multiple-lifetimes.rs:16:52
[01:17:04] +   --> $DIR/async-fn-multiple-lifetimes.rs:16:52
[01:17:04] +    |
[01:17:04] + LL | async fn multiple_elided_lifetimes(_: &u8, _: &u8) {}
[01:17:04] +    |                                       ---          ^ lifetime `'static` required
[01:17:04] +    |                                       help: add explicit lifetime `'static` to type: `&'static u8`
[01:17:04] + 
[01:17:04] + error: aborting due to 8 previous errors
[01:17:04] + 
[01:17:04] + 
[01:17:04] + Some errors occurred: E0106, E0309, E0621, E0623, E0707, E0709.
[01:17:04] 32 For more information about an error, try `rustc --explain E0106`.
[01:17:04] 33 
[01:17:04] 
[01:17:04] 
[01:17:04] The actual stderr differed from the expected stderr.
[01:17:04] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-fn-multiple-lifetimes/async-fn-multiple-lifetimes.stderr
[01:17:04] To update references, rerun the tests and pass the `--bless` flag
[01:17:04] To only update this specific test, also pass `--test-args async-fn-multiple-lifetimes.rs`
[01:17:04] error: 1 errors occurred comparing output.
[01:17:04] status: exit code: 1
[01:17:04] status: exit code: 1
[01:17:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-fn-multiple-lifetimes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-fn-multiple-lifetimes/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-fn-multiple-lifetimes/auxiliary" "-A" "unused"
[01:17:04] ------------------------------------------
[01:17:04] 
[01:17:04] ------------------------------------------
[01:17:04] stderr:
[01:17:04] stderr:
[01:17:04] ------------------------------------------
[01:17:04] {"message":"multiple different lifetimes used in arguments of `async fn`","code":{"code":"E0709","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/async-fn-multiple-lifetimes.rs","byte_start":162,"byte_end":164,"line_start":7,"line_end":7,"column_start":47,"column_end":49,"is_primary":true,"text":[{"text":"async fn multiple_named_lifetimes<'a, 'b>(_: &'a u8, _: &'b u8) {}","highlight_start":47,"highlight_end":49}],"label":"first lifetime here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/async-fn-multiple-lifetimes.rs","byte_start":173,"byte_end":175,"line_start":7,"line_end":7,"column_start":58,"column_end":60,"is_primary":true,"text":[{"text":"async fn multiple_named_lifetimes<'a, 'b>(_: &'a u8, _: &'b u8) {}","highlight_start":58,"highlight_end":60}],"label":"different lifetime here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`async fn` can only accept borrowed values with identical lifetimes","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0709]: multiple different lifetimes used in arguments of `async fn`\n  --> /checkout/src/test/ui/async-fn-multiple-lifetimes.rs:7:47\n   |\nLL | async fn multiple_named_lifetimes<'a, 'b>(_: &'a u8, _: &'b u8) {}\n   |                                               ^^         ^^ different lifetime here\n   |                                               |\n   |                                               first lifetime here\n   |\n   = help: `async fn` can only accept borrowed values with identical lifetimes\n\n"}
[01:17:04] {"message":"multiple elided lifetimes used in arguments of `async fn`","code":{"code":"E0707","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/async-fn-multiple-lifetimes.rs","byte_start":438,"byte_end":439,"line_start":16,"line_end":16,"column_start":39,"column_end":40,"is_primary":true,"text":[{"text":"async fn multiple_elided_lifetimes(_: &u8, _: &u8) {}","highlight_start":39,"highlight_end":40}],"label":"first lifetime here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/async-fn-multiple-lifetimes.rs","byte_start":446,"byte_end":447,"line_start":16,"line_end":16,"column_start":47,"column_end":48,"is_primary":true,"text":[{"text":"async fn multiple_elided_lifetimes(_: &u8, _: &u8) {}","highlight_start":47,"highlight_end":48}],"label":"different lifetime here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider giving these arguments named lifetimes","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0707]: multiple elided lifetimes used in arguments of `async fn`\n  --> /checkout/src/test/ui/async-fn-multiple-lifetimes.rs:16:39\n   |\nLL | async fn multiple_elided_lifetimes(_: &u8, _: &u8) {}\n   |                                       ^       ^ different lifetime here\n   |                                       |\n   |                                       first lifetime here\n   |\n   = help: consider giving these arguments named lifetimes\n\n"}
[01:17:04] {"message":"missing lifetime specifier","code":{"code":"E0106","explanation":"\nThis error indicates that a lifetime is missing from a type. If it is an error\ninside a function signature, the problem may be with failing to adhere to the\nlifetime elision rules (see below).\n\nHere are some simple examples of where you'll run into this error:\n\n
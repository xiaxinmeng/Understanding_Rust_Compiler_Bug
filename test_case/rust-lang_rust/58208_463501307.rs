plain
travis_time:end:21ce513a:start=1550120216135209234,finish=1550120218453186266,duration=2317977032
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:59:57] .................................................................................................... 1400/5386
[01:00:00] .................................................................................................... 1500/5386
[01:00:03] .................................................................................................... 1600/5386
[01:00:06] ...i..........................................................................i..................... 1700/5386
[01:00:09] ......F............................................................................................. 1800/5386
[01:00:17] .................................................................................................... 2000/5386
[01:00:20] ...................................i................................................................ 2100/5386
[01:00:24] .................................................................................................... 2200/5386
[01:00:28] .................................................................................................... 2300/5386
---
[01:01:57] ..................................i................................................................. 4600/5386
[01:02:03] .................................................................................................... 4700/5386
[01:02:07] .................................................................................................... 4800/5386
[01:02:10] .................................................................................................... 4900/5386
[01:02:14] ..............................F..................................................................... 5000/5386
[01:02:21] .................................................................................................... 5200/5386
[01:02:24] .................................................................................................... 5300/5386
[01:02:26] .........................i............................................................
[01:02:26] failures:
[01:02:26] failures:
[01:02:26] 
[01:02:26] ---- [ui] ui/hygiene/no_implicit_prelude.rs stdout ----
[01:02:26] diff of stderr:
[01:02:26] 
[01:02:26] 7 LL |         Vec::new(); //~ ERROR failed to resolve
[01:02:26] 8    |         ^^^ use of undeclared type or module `Vec`
[01:02:26] 9 
[01:02:26] + error: cannot find macro `print!` in this scope
[01:02:26] +   --> $DIR/no_implicit_prelude.rs:16:9
[01:02:26] +    |
[01:02:26] + LL |         println!(); // OK on 2015 edition (at least for now)
[01:02:26] +    |
[01:02:26] +    |
[01:02:26] +    = help: have you added the `#[macro_use]` on the module/import?
[01:02:26] + 
[01:02:26] + 
[01:02:26] 10 error[E0599]: no method named `clone` found for type `()` in the current scope
[01:02:26] 11   --> $DIR/no_implicit_prelude.rs:12:12
[01:02:26] 
[01:02:26] 20    = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:02:26] 21            `use std::clone::Clone;`
[01:02:26] 22 
---
[01:02:26] 26 For more information about an error, try `rustc --explain E0433`.
[01:02:26] 
[01:02:26] 
[01:02:26] The actual stderr differed from the expected stderr.
[01:02:26] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/no_implicit_prelude/no_implicit_prelude.stderr
[01:02:26] To update references, rerun the tests and pass the `--bless` flag
[01:02:26] To only update this specific test, also pass `--test-args hygiene/no_implicit_prelude.rs`
[01:02:26] error: 1 errors occurred comparing output.
[01:02:26] status: exit code: 1
[01:02:26] status: exit code: 1
[01:02:26] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/no_implicit_prelude.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/no_implicit_prelude/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/no_implicit_prelude/auxiliary" "-A" "unused"
[01:02:26] ------------------------------------------
[01:02:26] 
[01:02:26] ------------------------------------------
[01:02:26] stderr:
[01:02:26] stderr:
[01:02:26] ------------------------------------------
[01:02:26] {"message":"failed to resolve: use of undeclared type or module `Vec`","code":{"code":"E0433","explanation":"\nAn undeclared type or module was used.\n\nErroneous code example:\n\n
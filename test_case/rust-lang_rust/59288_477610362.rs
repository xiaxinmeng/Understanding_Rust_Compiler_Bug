plain
travis_time:end:0249c0e8:start=1553777598003770167,finish=1553777600592045260,duration=2588275093
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:12:11] .................................................................................................... 1400/5501
[01:12:14] .................................................................................................... 1500/5501
[01:12:16] .................................................................................................... 1600/5501
[01:12:20] ............................................i....................................................... 1700/5501
[01:12:24] ................................................................FF.................................. 1800/5501
[01:12:31] .................................................................................................... 2000/5501
[01:12:35] ..............................................................................i..................... 2100/5501
[01:12:39] .................................................................................................... 2200/5501
[01:12:43] .................................................................................................... 2300/5501
[01:12:43] .................................................................................................... 2300/5501
[01:12:47] .................................................................................................... 2400/5501
[01:12:51] .................................................................................................... 2500/5501
[01:12:55] .................................................................................................... 2600/5501
[01:12:59] .................................................................................................... 2700/5501
[01:13:04] ............F....................................................................................... 2800/5501
[01:13:07] .............................................................................F...................... 2900/5501
[01:13:15] .................................................................................................... 3100/5501
[01:13:18] .................................................................................................... 3200/5501
[01:13:22] .................................................................................................... 3300/5501
[01:13:26] .........i.......................................................................................... 3400/5501
---
[01:14:46] 
[01:14:46] 1 error[E0317]: if may be missing an else clause
[01:14:46] 2   --> $DIR/if-without-else-as-fn-expr.rs:2:5
[01:14:46] 3    |
[01:14:46] - LL |   fn foo(bar: usize) -> usize {
[01:14:46] -    |                         ----- expected `usize` because of this return type
[01:14:46] 6 LL | /     if bar % 5 == 0 {
[01:14:46] 7 LL | |         return 3;
[01:14:46] 8 LL | |     }
[01:14:46] -    | |_____^ expected usize, found ()
[01:14:46] +    | |_____^ expected (), found usize
[01:14:46] 10    |
[01:14:46] -    = note: expected type `usize`
[01:14:46] -    = note: expected type `usize`
[01:14:46] -               found type `()`
[01:14:46] -    = note: `if` expressions without `else` evaluate to `()`
[01:14:46] +    = note: expected type `()`
[01:14:46] +               found type `usize`
[01:14:46] 15 
[01:14:46] 16 error[E0317]: if may be missing an else clause
[01:14:46] 16 error[E0317]: if may be missing an else clause
[01:14:46] 17   --> $DIR/if-without-else-as-fn-expr.rs:9:20
[01:14:46] 
[01:14:46] 18    |
[01:14:46] 19 LL |       let x: usize = if bar % 5 == 0 {
[01:14:46] -    |  _________-__________^
[01:14:46] -    | |         expected because of this assignment
[01:14:46] +    |  ____________________^
[01:14:46] 23 LL | |         return 3;
[01:14:46] 24 LL | |     };
[01:14:46] 24 LL | |     };
[01:14:46] -    | |_____^ expected usize, found ()
[01:14:46] +    | |_____^ expected (), found usize
[01:14:46] 26    |
[01:14:46] -    = note: expected type `usize`
[01:14:46] -               found type `()`
[01:14:46] -    = note: `if` expressions without `else` evaluate to `()`
[01:14:46] +    = note: expected type `()`
[01:14:46] +               found type `usize`
[01:14:46] 31 
[01:14:46] 32 error[E0317]: if may be missing an else clause
[01:14:46] 32 error[E0317]: if may be missing an else clause
[01:14:46] 33   --> $DIR/if-without-else-as-fn-expr.rs:17:5
[01:14:46] 
[01:14:46] 34    |
[01:14:46] - LL |   fn foo3(bar: usize) -> usize {
[01:14:46] -    |                          ----- expected `usize` because of this return type
[01:14:46] 37 LL | /     if bar % 5 == 0 {
[01:14:46] 38 LL | |         3
[01:14:46] 39 LL | |     }
[01:14:46] -    | |_____^ expected usize, found ()
[01:14:46] +    | |_____^ expected (), found usize
[01:14:46] 41    |
[01:14:46] -    = note: expected type `usize`
[01:14:46] -    = note: expected type `usize`
[01:14:46] -               found type `()`
[01:14:46] -    = note: `if` expressions without `else` evaluate to `()`
[01:14:46] +    = note: expected type `()`
[01:14:46] +               found type `usize`
[01:14:46] 46 
[01:14:46] 47 error: aborting due to 3 previous errors
[01:14:46] 47 error: aborting due to 3 previous errors
[01:14:46] 48 
[01:14:46] 
[01:14:46] 
[01:14:46] The actual stderr differed from the expected stderr.
[01:14:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/if-without-else-as-fn-expr/if-without-else-as-fn-expr.stderr
[01:14:46] To update references, rerun the tests and pass the `--bless` flag
[01:14:46] To only update this specific test, also pass `--test-args if/if-without-else-as-fn-expr.rs`
[01:14:46] error: 1 errors occurred comparing output.
[01:14:46] status: exit code: 1
[01:14:46] status: exit code: 1
[01:14:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/if/if-without-else-as-fn-expr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/if-without-else-as-fn-expr/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/if-without-else-as-fn-expr/auxiliary" "-A" "unused"
[01:14:46] ------------------------------------------
[01:14:46] 
[01:14:46] ------------------------------------------
[01:14:46] stderr:
[01:14:46] stderr:
[01:14:46] ------------------------------------------
[01:14:46] {"message":"if may be missing an else clause","code":{"code":"E0317","explanation":"\nThis error occurs when an `if` expression without an `else` block is used in a\ncontext where a type other than `()` is expected, for example a `let`\nexpression:\n\n
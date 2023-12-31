plain
travis_time:end:1b4595a0:start=1544150510421805175,finish=1544150513191922006,duration=2770116831
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:46:17] 
[00:46:17] running 5127 tests
[00:46:20] ...............................................................F.................................... 100/5127
[00:46:23] .................................................................................................F.. 200/5127
[00:46:28] .................................................................................................... 400/5127
[00:46:32] .................................................................................................... 500/5127
[00:46:35] ..............................i..................................................................... 600/5127
[00:46:38] .................................................................................................... 700/5127
[00:46:38] .................................................................................................... 700/5127
[00:46:44] .................................................................................................... 800/5127
[00:46:48] .i...............i..........................................................................F....... 900/5127
[00:46:51] ........................iiiii..............F........................................................ 1000/5127
[00:46:54] ..................F................................................................................. 1100/5127
[00:46:57] .................................................................................................... 1200/5127
[00:46:59] ......................................................F............................................. 1300/5127
[00:47:04] .................................................................................................... 1500/5127
[00:47:07] ............................i....................................................................i.. 1600/5127
[00:47:10] .................................................................................................... 1700/5127
[00:47:13] ................................................................................................F... 1800/5127
[00:47:13] ................................................................................................F... 1800/5127
[00:47:17] .................................................................................................... 1900/5127
[00:47:20] ......................................i............................................................. 2000/5127
[00:47:23] .................................................................................................... 2100/5127
[00:47:28] ..........................................................F.F...........FF.......................... 2200/5127
[00:47:31] ........................................................................................F.........F. 2300/5127
[00:47:35] ....F.....................................F......................................................... 2400/5127
[00:47:39] .................................................................................................... 2500/5127
[00:47:42] ..F..............F..F..................................................F............................ 2600/5127
[00:47:50] .................................................................................................... 2800/5127
[00:47:50] .................................................................................................... 2800/5127
[00:47:53] .................................F..................................................F............... 2900/5127
[00:48:00] ..................................................................................i................. 3100/5127
[00:48:03] .................................................................................................... 3200/5127
[00:48:06] .............................................ii..i..ii.............................................. 3300/5127
[00:48:10] .................................................................................................... 3400/5127
---
[00:48:21] ..i................................................................................................. 3900/5127
[00:48:26] .................................................................................................... 4000/5127
[00:48:31] .................................................................................................... 4100/5127
[00:48:34] .................................................................................................... 4200/5127
[00:48:38] .................................................................................................Fi. 4300/5127
[00:48:43] .................................................................................................... 4400/5127
[00:48:47] ......F............................................................................................. 4500/5127
[00:48:54] ...................................................................................i................ 4700/5127
[00:48:54] ...................................................................................i................ 4700/5127
[00:48:57] ...............................F.................................................................... 4800/5127
[00:49:01] ..................................................F................................................. 4900/5127
[00:49:03] ..............................................................................................F..... 5000/5127
[00:49:07] ...........................
[00:49:07] failures:
[00:49:07] 
[00:49:07] ---- [ui] ui/associated-const/associated-const-no-item.rs stdout ----
[00:49:07] ---- [ui] ui/associated-const/associated-const-no-item.rs stdout ----
[00:49:07] diff of stderr:
[00:49:07] 
[00:49:07] 1 error[E0599]: no associated item named `ID` found for type `i32` in the current scope
[00:49:07] -   --> $DIR/associated-const-no-item.rs:16:23
[00:49:07] +   --> $DIR/associated-const-no-item.rs:16:16
[00:49:07] 3    |
[00:49:07] 4 LL | const X: i32 = <i32>::ID;
[00:49:07] -    |                |
[00:49:07] -    |                associated item not found in `i32`
[00:49:07] +    |                ^^^^^^^^^ associated item not found in `i32`
[00:49:07] 8    |
[00:49:07] 8    |
[00:49:07] 9    = help: items from traits can only be used if the trait is implemented and in scope
[00:49:07] 10    = note: the following trait defines an item `ID`, perhaps you need to implement it:
[00:49:07] 
[00:49:07] 
[00:49:07] The actual stderr differed from the expected stderr.
[00:49:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/associated-const-no-item/associated-const-no-item.stderr
[00:49:07] To update references, rerun the tests and pass the `--bless` flag
[00:49:07] To only update this specific test, also pass `--test-args associated-const/associated-const-no-item.rs`
[00:49:07] error: 1 errors occurred comparing output.
[00:49:07] status: exit code: 1
[00:49:07] status: exit code: 1
[00:49:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-const/associated-const-no-item.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/associated-const-no-item/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/associated-const-no-item/auxiliary" "-A" "unused"
[00:49:07] ------------------------------------------
[00:49:07] 
[00:49:07] ------------------------------------------
[00:49:07] stderr:
[00:49:07] stderr:
[00:49:07] ------------------------------------------
[00:49:07] {"message":"no associated item named `ID` found for type `i32` in the current scope","code":{"code":"E0599","explanation":"\nThis error occurs when a method is used on a type which doesn't implement it:\n\nErroneous code example:\n\n
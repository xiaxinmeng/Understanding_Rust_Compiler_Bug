plain
travis_time:end:0012bfde:start=1556762880318433727,finish=1556762881299066202,duration=980632475
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:10:21] ...........................................................i...............i........................ 900/5489
[01:10:24] ............................................................................................iiiii... 1000/5489
[01:10:28] .................................................................................................... 1100/5489
[01:10:31] .................................................................................................... 1200/5489
[01:10:33] .....F.............................................................................................. 1300/5489
[01:10:39] .................................................................................................... 1500/5489
[01:10:42] .................................................................................................... 1600/5489
[01:10:45] ......i............................................................................................. 1700/5489
[01:10:49] .................................................................................................... 1800/5489
---
[01:12:43] .................................................................................................... 4700/5489
[01:12:48] .................................................................................................... 4800/5489
[01:12:51] .................................................................................................... 4900/5489
[01:12:55] .................................................................................................... 5000/5489
[01:12:59] ........................................F.F..F...................................................... 5100/5489
[01:13:03] ......F............................................................................................. 5200/5489
[01:13:09] .................................................................................................... 5400/5489
[01:13:12] ...........................i.............................................................
[01:13:12] failures:
[01:13:12] 
[01:13:12] 
[01:13:12] ---- [ui] ui/error-codes/E0225.rs stdout ----
[01:13:12] diff of stderr:
[01:13:12] 
[01:13:12] 1 error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12] -   --> $DIR/E0225.rs:6:32
[01:13:12] +   --> $DIR/E0225.rs:6:36
[01:13:12] 3    |
[01:13:12] - LL |     let _: Box<std::io::Read + std::io::Write>;
[01:13:12] -    |                -------------   ^^^^^^^^^^^^^^ additional non-auto trait
[01:13:12] -    |                first non-auto trait
[01:13:12] -    |                first non-auto trait
[01:13:12] + LL |     let _: Box<dyn std::io::Read + std::io::Write>;
[01:13:12] +    |                    -------------   ^^^^^^^^^^^^^^ additional non-auto trait
[01:13:12] +    |                    first non-auto trait
[01:13:12] 8 
[01:13:12] 8 
[01:13:12] 9 error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12] -   --> $DIR/E0225.rs:8:16
[01:13:12] +   --> $DIR/E0225.rs:8:20
[01:13:12] 11    |
[01:13:12] 12 LL | trait Foo = std::io::Read + std::io::Write;
[01:13:12] 13    |             -------------   -------------- additional non-auto trait
[01:13:12] 14    |             |
[01:13:12] 15    |             first non-auto trait
[01:13:12] 16 ...
[01:13:12] 16 ...
[01:13:12] - LL |     let _: Box<Foo>;
[01:13:12] -    |                ^^^
[01:13:12] + LL |     let _: Box<dyn Foo>;
[01:13:12] 19 
[01:13:12] 20 error: aborting due to 2 previous errors
[01:13:12] 21 
[01:13:12] 
[01:13:12] 
[01:13:12] 
[01:13:12] The actual stderr differed from the expected stderr.
[01:13:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0225/E0225.stderr
[01:13:12] To update references, rerun the tests and pass the `--bless` flag
[01:13:12] To only update this specific test, also pass `--test-args error-codes/E0225.rs`
[01:13:12] error: 1 errors occurred comparing output.
[01:13:12] status: exit code: 1
[01:13:12] status: exit code: 1
[01:13:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0225.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0225/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0225/auxiliary" "-A" "unused"
[01:13:12] ------------------------------------------
[01:13:12] 
[01:13:12] ------------------------------------------
[01:13:12] stderr:
[01:13:12] stderr:
[01:13:12] ------------------------------------------
[01:13:12] error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12]    |
[01:13:12]    |
[01:13:12] LL |     let _: Box<dyn std::io::Read + std::io::Write>;
[01:13:12]    |                    -------------   ^^^^^^^^^^^^^^ additional non-auto trait
[01:13:12]    |                    first non-auto trait
[01:13:12] 
[01:13:12] 
[01:13:12] error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12]    |
[01:13:12]    |
[01:13:12] LL | trait Foo = std::io::Read + std::io::Write;
[01:13:12]    |             -------------   -------------- additional non-auto trait
[01:13:12]    |             first non-auto trait
[01:13:12] ...
[01:13:12] LL |     let _: Box<dyn Foo>;
[01:13:12]    |                    ^^^
---
[01:13:12] 
[01:13:12] ---- [ui] ui/traits/trait-alias/trait-alias-no-duplicates.rs stdout ----
[01:13:12] diff of stderr:
[01:13:12] 
[01:13:12] 1 error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12] -   --> $DIR/trait-alias-no-duplicates.rs:16:22
[01:13:12] -    |
[01:13:12] - LL | trait _0 = Obj;
[01:13:12] -    |            |
[01:13:12] -    |            additional non-auto trait
[01:13:12] -    |            first non-auto trait
[01:13:12] - ...
[01:13:12] - ...
[01:13:12] - LL | type _T00 = dyn _0 + _0;
[01:13:12] - 
[01:13:12] - 
[01:13:12] - error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12] -   --> $DIR/trait-alias-no-duplicates.rs:19:22
[01:13:12] -    |
[01:13:12] - LL | trait _0 = Obj;
[01:13:12] -    |            |
[01:13:12] -    |            additional non-auto trait
[01:13:12] -    |            first non-auto trait
[01:13:12] - ...
[01:13:12] - ...
[01:13:12] - LL | type _T01 = dyn _1 + _0;
[01:13:12] - 
[01:13:12] - 
[01:13:12] - error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12] -   --> $DIR/trait-alias-no-duplicates.rs:22:22
[01:13:12] -    |
[01:13:12] - LL | trait _0 = Obj;
[01:13:12] -    |            |
[01:13:12] -    |            additional non-auto trait
[01:13:12] -    |            first non-auto trait
[01:13:12] -    |            first non-auto trait
[01:13:12] - LL | trait _1 = _0;
[01:13:12] -    |            -- referenced here
[01:13:12] - ...
[01:13:12] - LL | type _T02 = dyn _1 + _1;
[01:13:12] - 
[01:13:12] - 
[01:13:12] - error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12] 40   --> $DIR/trait-alias-no-duplicates.rs:25:23
[01:13:12] 41    |
[01:13:12] 42 LL | trait _0 = Obj;
[01:13:12] 59    |                      ^^^ additional non-auto trait
[01:13:12] 60 
[01:13:12] 60 
[01:13:12] 61 error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12] -   --> $DIR/trait-alias-no-duplicates.rs:37:17
[01:13:12] +   --> $DIR/trait-alias-no-duplicates.rs:37:22
[01:13:12] 63    |
[01:13:12] 64 LL | trait _0 = Obj;
[01:13:12] -    |            |
[01:13:12] -    |            additional non-auto trait
[01:13:12] -    |            first non-auto trait
[01:13:12] -    |            first non-auto trait
[01:13:12] - LL | trait _1 = _0;
[01:13:12] -    |            -- referenced here
[01:13:12] +    |            --- first non-auto trait
[01:13:12] 71 ...
[01:13:12] - LL | trait _2 = _0 + _1;
[01:13:12] -    |                 -- referenced here
[01:13:12] + LL | trait _3 = Obj;
[01:13:12] +    |            --- additional non-auto trait
[01:13:12] 74 ...
[01:13:12] 75 LL | type _T10 = dyn _2 + _3;
[01:13:12] +    |                      ^^
[01:13:12] 77 
[01:13:12] 77 
[01:13:12] 78 error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12] 79   --> $DIR/trait-alias-no-duplicates.rs:40:22
[01:13:12] 104    |                 first non-auto trait
[01:13:12] 105 
[01:13:12] 105 
[01:13:12] 106 error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12] -   --> $DIR/trait-alias-no-duplicates.rs:46:17
[01:13:12] +   --> $DIR/trait-alias-no-duplicates.rs:46:22
[01:13:12] 108    |
[01:13:12] 109 LL | trait _0 = Obj;
[01:13:12] -    |            |
[01:13:12] -    |            additional non-auto trait
[01:13:12] -    |            first non-auto trait
[01:13:12] -    |            first non-auto trait
[01:13:12] - LL | trait _1 = _0;
[01:13:12] -    |            -- referenced here
[01:13:12] +    |            --- first non-auto trait
[01:13:12] 116 ...
[01:13:12] - LL | trait _2 = _0 + _1;
[01:13:12] -    |                 -- referenced here
[01:13:12] - ...
[01:13:12] 120 LL | type _T13 = dyn _2 + Obj;
[01:13:12] +    |                      ^^^ additional non-auto trait
[01:13:12] 122 
[01:13:12] 122 
[01:13:12] 123 error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12] 124   --> $DIR/trait-alias-no-duplicates.rs:49:22
[01:13:12] 175    |                      ^^
[01:13:12] 176 
[01:13:12] 176 
[01:13:12] 177 error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12] -   --> $DIR/trait-alias-no-duplicates.rs:65:22
[01:13:12] -    |
[01:13:12] - LL | trait _5 = Obj + Send;
[01:13:12] -    |            |
[01:13:12] -    |            additional non-auto trait
[01:13:12] -    |            first non-auto trait
[01:13:12] - LL | 
[01:13:12] - LL | 
[01:13:12] - LL | type _T20 = dyn _5 + _5;
[01:13:12] - 
[01:13:12] - 
[01:13:12] - error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12] 190   --> $DIR/trait-alias-no-duplicates.rs:68:23
[01:13:12] 191    |
[01:13:12] 192 LL | trait _5 = Obj + Send;
[01:13:12] 216    |                                    ^^^ additional non-auto trait
[01:13:12] 217 
[01:13:12] 217 
[01:13:12] 218 error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12] -   --> $DIR/trait-alias-no-duplicates.rs:81:17
[01:13:12] -    |
[01:13:12] - LL | trait _5 = Obj + Send;
[01:13:12] -    |            |
[01:13:12] -    |            additional non-auto trait
[01:13:12] -    |            first non-auto trait
[01:13:12] - ...
[01:13:12] - ...
[01:13:12] - LL | trait _6 = _5 + _5; // ==> Obj + Send + Obj + Send
[01:13:12] -    |                 -- referenced here
[01:13:12] - LL | 
[01:13:12] - LL | type _T30 = dyn _6;
[01:13:12] - 
[01:13:12] - 
[01:13:12] - error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12] -   --> $DIR/trait-alias-no-duplicates.rs:84:17
[01:13:12] -    |
[01:13:12] - LL | trait _5 = Obj + Send;
[01:13:12] -    |            |
[01:13:12] -    |            additional non-auto trait
[01:13:12] -    |            first non-auto trait
[01:13:12] - ...
[01:13:12] - ...
[01:13:12] - LL | trait _6 = _5 + _5; // ==> Obj + Send + Obj + Send
[01:13:12] -    |                 -- referenced here
[01:13:12] - ...
[01:13:12] - LL | type _T31 = dyn _6 + Send;
[01:13:12] - 
[01:13:12] - 
[01:13:12] - error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12] -   --> $DIR/trait-alias-no-duplicates.rs:87:24
[01:13:12] -    |
[01:13:12] - LL | trait _5 = Obj + Send;
[01:13:12] -    |            |
[01:13:12] -    |            additional non-auto trait
[01:13:12] -    |            first non-auto trait
[01:13:12] - ...
[01:13:12] - ...
[01:13:12] - LL | trait _6 = _5 + _5; // ==> Obj + Send + Obj + Send
[01:13:12] -    |                 -- referenced here
[01:13:12] - ...
[01:13:12] - LL | type _T32 = dyn Send + _6;
[01:13:12] - 
[01:13:12] - 
[01:13:12] - error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12] 264   --> $DIR/trait-alias-no-duplicates.rs:95:22
[01:13:12] 265    |
[01:13:12] 266 LL | trait _5 = Obj + Send;
[01:13:12] 
[01:13:12] 353 LL | type _T60 = _11 + _12;
[01:13:12] 355 
[01:13:12] - error: aborting due to 27 previous errors
[01:13:12] + error: aborting due to 20 previous errors
[01:13:12] 357 
[01:13:12] 357 
[01:13:12] 358 For more information about this error, try `rustc --explain E0225`.
[01:13:12] 359 
[01:13:12] 
[01:13:12] 
[01:13:12] The actual stderr differed from the expected stderr.
[01:13:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias/trait-alias-no-duplicates/trait-alias-no-duplicates.stderr
[01:13:12] To update references, rerun the tests and pass the `--bless` flag
[01:13:12] To only update this specific test, also pass `--test-args traits/trait-alias/trait-alias-no-duplicates.rs`
[01:13:12] error: 1 errors occurred comparing output.
[01:13:12] status: exit code: 1
[01:13:12] status: exit code: 1
[01:13:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-alias/trait-alias-no-duplicates.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias/trait-alias-no-duplicates/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias/trait-alias-no-duplicates/auxiliary" "-A" "unused"
[01:13:12] ------------------------------------------
[01:13:12] 
[01:13:12] ------------------------------------------
[01:13:12] stderr:
[01:13:12] stderr:
[01:13:12] ------------------------------------------
[01:13:12] error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12]   --> /checkout/src/test/ui/traits/trait-alias/trait-alias-no-duplicates.rs:25:23
[01:13:12]    |
[01:13:12] LL | trait _0 = Obj;
[01:13:12]    |            --- additional non-auto trait
[01:13:12] LL | trait _1 = _0;
[01:13:12]    |            -- referenced here
[01:13:12] ...
[01:13:12] LL | type _T03 = dyn Obj + _1;
[01:13:12]    |                 |
[01:13:12]    |                 first non-auto trait
[01:13:12] 
[01:13:12] 
[01:13:12] error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12]   --> /checkout/src/test/ui/traits/trait-alias/trait-alias-no-duplicates.rs:28:22
[01:13:12]    |
[01:13:12] LL | trait _0 = Obj;
[01:13:12]    |            --- first non-auto trait
[01:13:12] ...
[01:13:12] LL | type _T04 = dyn _1 + Obj;
[01:13:12]    |                      ^^^ additional non-auto trait
[01:13:12] 
[01:13:12] error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12]   --> /checkout/src/test/ui/traits/trait-alias/trait-alias-no-duplicates.rs:37:22
[01:13:12]    |
[01:13:12] LL | trait _0 = Obj;
[01:13:12]    |            --- first non-auto trait
[01:13:12] ...
[01:13:12] LL | trait _3 = Obj;
[01:13:12]    |            --- additional non-auto trait
[01:13:12] ...
[01:13:12] LL | type _T10 = dyn _2 + _3;
[01:13:12] 
[01:13:12] 
[01:13:12] error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12]   --> /checkout/src/test/ui/traits/trait-alias/trait-alias-no-duplicates.rs:40:22
[01:13:12]    |
[01:13:12] LL | trait _0 = Obj;
[01:13:12]    |            --- additional non-auto trait
[01:13:12] ...
[01:13:12] LL | trait _2 = _0 + _1;
[01:13:12]    |            -- referenced here
[01:13:12] LL | trait _3 = Obj;
[01:13:12]    |            --- first non-auto trait
[01:13:12] ...
[01:13:12] LL | type _T11 = dyn _3 + _2;
[01:13:12] 
[01:13:12] 
[01:13:12] error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12]   --> /checkout/src/test/ui/traits/trait-alias/trait-alias-no-duplicates.rs:43:23
[01:13:12]    |
[01:13:12] LL | trait _0 = Obj;
[01:13:12]    |            --- additional non-auto trait
[01:13:12] ...
[01:13:12] LL | trait _2 = _0 + _1;
[01:13:12]    |            -- referenced here
[01:13:12] ...
[01:13:12] LL | type _T12 = dyn Obj + _2;
[01:13:12]    |                 |
[01:13:12]    |                 first non-auto trait
[01:13:12] 
[01:13:12] 
[01:13:12] error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12]   --> /checkout/src/test/ui/traits/trait-alias/trait-alias-no-duplicates.rs:46:22
[01:13:12]    |
[01:13:12] LL | trait _0 = Obj;
[01:13:12]    |            --- first non-auto trait
[01:13:12] ...
[01:13:12] LL | type _T13 = dyn _2 + Obj;
[01:13:12]    |                      ^^^ additional non-auto trait
[01:13:12] 
[01:13:12] error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12]   --> /checkout/src/test/ui/traits/trait-alias/trait-alias-no-duplicates.rs:49:22
[01:13:12]    |
[01:13:12] LL | trait _0 = Obj;
[01:13:12]    |            --- first non-auto trait
[01:13:12] ...
[01:13:12] LL | trait _3 = Obj;
[01:13:12]    |            --- additional non-auto trait
[01:13:12] ...
[01:13:12] LL | type _T14 = dyn _1 + _3;
[01:13:12] 
[01:13:12] 
[01:13:12] error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12]   --> /checkout/src/test/ui/traits/trait-alias/trait-alias-no-duplicates.rs:52:22
[01:13:12]    |
[01:13:12] LL | trait _0 = Obj;
[01:13:12]    |            --- additional non-auto trait
[01:13:12] LL | trait _1 = _0;
[01:13:12]    |            -- referenced here
[01:13:12] ...
[01:13:12] LL | trait _3 = Obj;
[01:13:12]    |            --- first non-auto trait
[01:13:12] ...
[01:13:12] LL | type _T15 = dyn _3 + _1;
[01:13:12] 
[01:13:12] 
[01:13:12] error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12]   --> /checkout/src/test/ui/traits/trait-alias/trait-alias-no-duplicates.rs:55:22
[01:13:12]    |
[01:13:12] LL | trait _0 = Obj;
[01:13:12]    |            --- first non-auto trait
[01:13:12] ...
[01:13:12] LL | trait _3 = Obj;
[01:13:12]    |            --- additional non-auto trait
[01:13:12] LL | trait _4 = _3;
[01:13:12]    |            -- referenced here
[01:13:12] ...
[01:13:12] LL | type _T16 = dyn _1 + _4;
[01:13:12] 
[01:13:12] 
[01:13:12] error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12]   --> /checkout/src/test/ui/traits/trait-alias/trait-alias-no-duplicates.rs:58:22
[01:13:12]    |
[01:13:12] LL | trait _0 = Obj;
[01:13:12]    |            --- additional non-auto trait
[01:13:12] LL | trait _1 = _0;
[01:13:12]    |            -- referenced here
[01:13:12] ...
[01:13:12] LL | trait _3 = Obj;
[01:13:12]    |            --- first non-auto trait
[01:13:12] ...
[01:13:12] LL | type _T17 = dyn _4 + _1;
[01:13:12] 
[01:13:12] 
[01:13:12] error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12]   --> /checkout/src/test/ui/traits/trait-alias/trait-alias-no-duplicates.rs:68:23
[01:13:12]    |
[01:13:12] LL | trait _5 = Obj + Send;
[01:13:12]    |            --- additional non-auto trait
[01:13:12] ...
[01:13:12] LL | type _T21 = dyn Obj + _5;
[01:13:12]    |                 |
[01:13:12]    |                 first non-auto trait
[01:13:12] 
[01:13:12] 
[01:13:12] error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12]   --> /checkout/src/test/ui/traits/trait-alias/trait-alias-no-duplicates.rs:71:22
[01:13:12]    |
[01:13:12] LL | trait _5 = Obj + Send;
[01:13:12]    |            --- first non-auto trait
[01:13:12] ...
[01:13:12] LL | type _T22 = dyn _5 + Obj;
[01:13:12]    |                      ^^^ additional non-auto trait
[01:13:12] 
[01:13:12] error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12]   --> /checkout/src/test/ui/traits/trait-alias/trait-alias-no-duplicates.rs:74:36
[01:13:12]    |
[01:13:12] LL | trait _5 = Obj + Send;
[01:13:12]    |            --- first non-auto trait
[01:13:12] ...
[01:13:12] LL | type _T23 = dyn _5 + Send + Sync + Obj;
[01:13:12]    |                                    ^^^ additional non-auto trait
[01:13:12] 
[01:13:12] error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12]   --> /checkout/src/test/ui/traits/trait-alias/trait-alias-no-duplicates.rs:95:22
[01:13:12]    |
[01:13:12] LL | trait _5 = Obj + Send;
[01:13:12]    |            --- first non-auto trait
[01:13:12] ...
[01:13:12] LL | type _T40 = dyn _8 + Obj;
[01:13:12]    |                      ^^^ additional non-auto trait
[01:13:12] 
[01:13:12] error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12]   --> /checkout/src/test/ui/traits/trait-alias/trait-alias-no-duplicates.rs:98:23
[01:13:12]    |
[01:13:12] LL | trait _5 = Obj + Send;
[01:13:12]    |            --- additional non-auto trait
[01:13:12] ...
[01:13:12] LL | trait _7 = _5 + Sync;
[01:13:12]    |            -- referenced here
[01:13:12] LL | trait _8 = Unpin + _7;
[01:13:12]    |                    -- referenced here
[01:13:12] ...
[01:13:12] LL | type _T41 = dyn Obj + _8;
[01:13:12]    |                 |
[01:13:12]    |                 first non-auto trait
[01:13:12] 
[01:13:12] 
[01:13:12] error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12]   --> /checkout/src/test/ui/traits/trait-alias/trait-alias-no-duplicates.rs:101:22
[01:13:12]    |
[01:13:12] LL | trait _3 = Obj;
[01:13:12]    |            --- additional non-auto trait
[01:13:12] LL | trait _4 = _3;
[01:13:12]    |            -- referenced here
[01:13:12] ...
[01:13:12] LL | trait _5 = Obj + Send;
[01:13:12]    |            --- first non-auto trait
[01:13:12] ...
[01:13:12] LL | type _T42 = dyn _8 + _4;
[01:13:12] 
[01:13:12] 
[01:13:12] error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12]   --> /checkout/src/test/ui/traits/trait-alias/trait-alias-no-duplicates.rs:104:22
[01:13:12]    |
[01:13:12] LL | trait _3 = Obj;
[01:13:12]    |            --- first non-auto trait
[01:13:12] ...
[01:13:12] LL | trait _5 = Obj + Send;
[01:13:12]    |            --- additional non-auto trait
[01:13:12] ...
[01:13:12] LL | trait _7 = _5 + Sync;
[01:13:12]    |            -- referenced here
[01:13:12] LL | trait _8 = Unpin + _7;
[01:13:12]    |                    -- referenced here
[01:13:12] ...
[01:13:12] LL | type _T43 = dyn _4 + _8;
[01:13:12] 
[01:13:12] 
[01:13:12] error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12]   --> /checkout/src/test/ui/traits/trait-alias/trait-alias-no-duplicates.rs:107:36
[01:13:12]    |
[01:13:12] LL | trait _3 = Obj;
[01:13:12]    |            --- first non-auto trait
[01:13:12] ...
[01:13:12] LL | trait _5 = Obj + Send;
[01:13:12]    |            --- additional non-auto trait
[01:13:12] ...
[01:13:12] LL | trait _7 = _5 + Sync;
[01:13:12]    |            -- referenced here
[01:13:12] LL | trait _8 = Unpin + _7;
[01:13:12]    |                    -- referenced here
[01:13:12] ...
[01:13:12] LL | type _T44 = dyn _4 + Send + Sync + _8;
[01:13:12] 
[01:13:12] 
[01:13:12] error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12]   --> /checkout/src/test/ui/traits/trait-alias/trait-alias-no-duplicates.rs:117:18
[01:13:12]    |
[01:13:12] LL | trait _9 = for<'a> ObjL<'a>;
[01:13:12]    |            ---------------- first non-auto trait
[01:13:12] LL | trait _10 = for<'b> ObjL<'b>;
[01:13:12]    |             ---------------- additional non-auto trait
[01:13:12] LL | type _T50 = _9 + _10;
[01:13:12] 
[01:13:12] 
[01:13:12] error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12]   --> /checkout/src/test/ui/traits/trait-alias/trait-alias-no-duplicates.rs:123:19
[01:13:12]    |
[01:13:12] LL | trait _11 = ObjT<for<'a> fn(&'a u8)>;
[01:13:12]    |             ------------------------ first non-auto trait
[01:13:12] LL | trait _12 = ObjT<for<'b> fn(&'b u8)>;
[01:13:12]    |             ------------------------ additional non-auto trait
[01:13:12] LL | type _T60 = _11 + _12;
[01:13:12] 
[01:13:12] error: aborting due to 20 previous errors
[01:13:12] 
[01:13:12] For more information about this error, try `rustc --explain E0225`.
[01:13:12] For more information about this error, try `rustc --explain E0225`.
[01:13:12] 
[01:13:12] ------------------------------------------
[01:13:12] 
[01:13:12] 
[01:13:12] ---- [ui] ui/traits/trait-alias/trait-alias-no-extra-traits.rs stdout ----
[01:13:12] 
[01:13:12] 41    |                      ^^^^ additional non-auto trait
[01:13:12] 42 
[01:13:12] 42 
[01:13:12] 43 error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12] -   --> $DIR/trait-alias-no-extra-traits.rs:34:22
[01:13:12] -    |
[01:13:12] - LL | trait _2 = ObjB;
[01:13:12] -    |            |
[01:13:12] -    |            additional non-auto trait
[01:13:12] -    |            first non-auto trait
[01:13:12] -    |            first non-auto trait
[01:13:12] - LL | trait _3 = _2;
[01:13:12] -    |            -- referenced here
[01:13:12] - ...
[01:13:12] - LL | type _T10 = dyn _2 + _3;
[01:13:12] - 
[01:13:12] - 
[01:13:12] - error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12] -   --> $DIR/trait-alias-no-extra-traits.rs:37:22
[01:13:12] -    |
[01:13:12] - LL | trait _2 = ObjB;
[01:13:12] -    |            |
[01:13:12] -    |            additional non-auto trait
[01:13:12] -    |            first non-auto trait
[01:13:12] - ...
[01:13:12] - ...
[01:13:12] - LL | type _T11 = dyn _3 + _2;
[01:13:12] - 
[01:13:12] - 
[01:13:12] - error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12] -   --> $DIR/trait-alias-no-extra-traits.rs:40:22
[01:13:12] -    |
[01:13:12] - LL | trait _2 = ObjB;
[01:13:12] -    |            |
[01:13:12] -    |            additional non-auto trait
[01:13:12] -    |            first non-auto trait
[01:13:12] -    |            first non-auto trait
[01:13:12] - LL | trait _3 = _2;
[01:13:12] -    |            -- referenced here
[01:13:12] - LL | trait _4 = _3;
[01:13:12] -    |            -- referenced here
[01:13:12] - ...
[01:13:12] - LL | type _T12 = dyn _2 + _4;
[01:13:12] - 
[01:13:12] - 
[01:13:12] - error[E0225]: only auto traits can be used as additional traits in a trait object
[01:13:12] -   --> $DIR/trait-alias-no-extra-traits.rs:43:22
[01:13:12] -    |
[01:13:12] - LL | trait _2 = ObjB;
[01:13:12] -    |            |
---
[01:13:12] 1 error[E0224]: at least one non-builtin trait is required for an object type
[01:13:12] -   --> $DIR/wf-trait-object-maybe-bound.rs:4:11
[01:13:12] +   --> $DIR/wf-trait-object-only-maybe-bound.rs:3:11
[01:13:12] 3    |
[01:13:12] 4 LL | type _0 = dyn ?Sized;
[01:13:12] 
[01:13:12] 
[01:13:12] The actual stderr differed from the expected stderr.
[01:13:12] The actual stderr differed from the expected stderr.
[01:13:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/wf-trait-object-only-maybe-bound/wf-trait-object-only-maybe-bound.stderr
[01:13:12] To update references, rerun the tests and pass the `--bless` flag
[01:13:12] To only update this specific test, also pass `--test-args traits/wf-trait-object-only-maybe-bound.rs`
[01:13:12] error: 1 errors occurred comparing output.
[01:13:12] status: exit code: 1
[01:13:12] status: exit code: 1
[01:13:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/wf-trait-object-only-maybe-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/wf-trait-object-only-maybe-bound/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/wf-trait-object-only-maybe-bound/auxiliary" "-A" "unused"
[01:13:12] ------------------------------------------
[01:13:12] 
[01:13:12] ------------------------------------------
[01:13:12] stderr:
[01:13:12] stderr:
[01:13:12] ------------------------------------------
[01:13:12] error[E0224]: at least one non-builtin trait is required for an object type
[01:13:12]   --> /checkout/src/test/ui/traits/wf-trait-object-only-maybe-bound.rs:3:11
[01:13:12]    |
[01:13:12] LL | type _0 = dyn ?Sized;
[01:13:12] 
[01:13:12] error: aborting due to previous error
[01:13:12] 
[01:13:12] 
[01:13:12] 
[01:13:12] ------------------------------------------
[01:13:12] 
[01:13:12] 
[01:13:12] 
[01:13:12] failures:
[01:13:12]     [ui] ui/error-codes/E0225.rs
[01:13:12]     [ui] ui/traits/trait-alias/trait-alias-no-duplicates.rs
[01:13:12]     [ui] ui/traits/trait-alias/trait-alias-no-extra-traits.rs
[01:13:12]     [ui] ui/traits/trait-alias/trait-alias-only-maybe-bound.rs
[01:13:12]     [ui] ui/traits/wf-trait-object-only-maybe-bound.rs
[01:13:12] test result: FAILED. 5463 passed; 5 failed; 21 ignored; 0 measured; 0 filtered out
[01:13:12] 
[01:13:12] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:519:22
[01:13:12] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:13:12] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:13:12] 
[01:13:12] 
[01:13:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:12] 
[01:13:12] 
[01:13:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:12] Build completed unsuccessfully in 0:04:25
[01:13:12] Build completed unsuccessfully in 0:04:25
[01:13:12] Makefile:48: recipe for target 'check' failed
[01:13:12] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:073b3fa6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu May  2 03:21:24 UTC 2019
---
travis_time:end:00388cb6:start=1556767285955664725,finish=1556767285960485568,duration=4820843
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0d36a078
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:22496a5c
travis_time:start:22496a5c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0575a891
$ dmesg | grep -i kill

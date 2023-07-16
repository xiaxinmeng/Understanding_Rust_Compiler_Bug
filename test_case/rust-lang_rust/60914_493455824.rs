plain
travis_time:end:01d9a8a2:start=1558095920146174210,finish=1558095920880119194,duration=733944984
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:06:51] 
[01:06:51] running 5533 tests
[01:06:54] .................................................................................................... 100/5533
[01:06:58] .................................................................................................... 200/5533
[01:07:01] ...........F...........................................F................................FF.......... 300/5533
[01:07:04] ..........F......................................................................................... 400/5533
[01:07:07] ...................................................................................................i 500/5533
[01:07:14] .................................................................................................... 700/5533
[01:07:19] .................................................................................................... 800/5533
[01:07:19] .................................................................................................... 800/5533
[01:07:24] .....................F....................F..F.................................i...............i.... 900/5533
[01:07:32] ............iiiii................................................................................... 1100/5533
[01:07:32] ............iiiii................................................................................... 1100/5533
[01:07:35] .......F............................................................................................ 1200/5533
[01:07:40] .................................................................................................... 1400/5533
[01:07:43] .................................................................................................... 1500/5533
[01:07:43] .................................................................................................... 1500/5533
[01:07:45] .............................................................F...................................... 1600/5533
[01:07:51] .................................................................................................... 1800/5533
[01:07:55] .................................................................................................... 1900/5533
[01:07:58] .................................................................................................... 2000/5533
[01:07:58] .................................................................................................... 2000/5533
[01:08:01] ..................................................F.......................i......................... 2100/5533
[01:08:08] .................................................................................................... 2300/5533
[01:08:12] .................................................................................................... 2400/5533
[01:08:16] .................................................................................................... 2500/5533
[01:08:20] .................................................................................................... 2600/5533
[01:08:20] .................................................................................................... 2600/5533
[01:08:24] ...............................................................................FF................... 2700/5533
[01:08:28] ......................................................................F............................. 2800/5533
[01:08:31] .................................................F.................................................. 2900/5533
[01:08:38] .................................................................................................... 3100/5533
[01:08:41] .................................................................................................... 3200/5533
[01:08:45] .................................................................................................... 3300/5533
[01:08:48] .........i.......................................................................................... 3400/5533
---
[01:09:05] .................................................................................................... 3900/5533
[01:09:07] .............i...................................................................................... 4000/5533
[01:09:09] ............................................................................i....................... 4100/5533
[01:09:11] .................................................................................................... 4200/5533
[01:09:17] .....F.............................................................................................. 4300/5533
[01:09:24] .................................................................................................... 4400/5533
[01:09:27] .......F............................................................................................ 4500/5533
[01:09:34] .................................................................................................... 4700/5533
[01:09:40] .................................................................................................... 4800/5533
[01:09:43] .................................................................................................... 4900/5533
[01:09:45] .................................................................................................... 5000/5533
[01:09:45] .................................................................................................... 5000/5533
[01:09:49] .................................................................................F.................. 5100/5533
[01:09:56] .................................................................................................... 5300/5533
[01:09:58] .................................................................................................... 5400/5533
[01:10:01] .......................................................................i............................ 5500/5533
[01:10:02] .................................
[01:10:02] .................................
[01:10:02] failures:
[01:10:02] 
[01:10:02] ---- [ui] ui/borrowck/borrowck-anon-fields-variant.rs stdout ----
[01:10:02] diff of stderr:
[01:10:02] 
[01:10:02] - warning[E0503]: cannot use `y` because it was mutably borrowed
[01:10:02] + error[E0503]: cannot use `y` because it was mutably borrowed
[01:10:02] 3    |
[01:10:02] 3    |
[01:10:02] 4 LL |       Foo::Y(ref mut a, _) => a,
[01:10:02] 9 ...
[01:10:02] 9 ...
[01:10:02] 10 LL |     *a += 1;
[01:10:02] -    |
[01:10:02] -    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:10:02] -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:10:02] 15 
[01:10:02] 15 
[01:10:02] 16 error[E0503]: cannot use `y` because it was mutably borrowed
[01:10:02] 17   --> $DIR/borrowck-anon-fields-variant.rs:37:7
[01:10:02] 
[01:10:02] 37 LL |     *a += 1;
[01:10:02] 38    |     ------- first borrow later used here
[01:10:02] - error: aborting due to 2 previous errors
[01:10:02] + error: aborting due to 3 previous errors
[01:10:02] 41 
[01:10:02] 42 Some errors have detailed explanations: E0499, E0503.
[01:10:02] 42 Some errors have detailed explanations: E0499, E0503.
[01:10:02] 43 For more information about an error, try `rustc --explain E0499`.
[01:10:02] 
[01:10:02] 
[01:10:02] The actual stderr differed from the expected stderr.
[01:10:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-anon-fields-variant/borrowck-anon-fields-variant.stderr
[01:10:02] To update references, rerun the tests and pass the `--bless` flag
[01:10:02] To only update this specific test, also pass `--test-args borrowck/borrowck-anon-fields-variant.rs`
[01:10:02] error: 1 errors occurred comparing output.
[01:10:02] status: exit code: 1
[01:10:02] status: exit code: 1
[01:10:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-anon-fields-variant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-anon-fields-variant" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-anon-fields-variant/auxiliary" "-A" "unused"
[01:10:02] ------------------------------------------
[01:10:02] 
[01:10:02] ------------------------------------------
[01:10:02] stderr:
[01:10:02] stderr:
[01:10:02] ------------------------------------------
[01:10:02] error[E0503]: cannot use `y` because it was mutably borrowed
[01:10:02]   --> /checkout/src/test/ui/borrowck/borrowck-anon-fields-variant.rs:17:7
[01:10:02]    |
[01:10:02] LL |       Foo::Y(ref mut a, _) => a,
[01:10:02]    |              --------- borrow of `y.0` occurs here
[01:10:02] ...
[01:10:02] LL |       Foo::Y(_, ref mut b) => b,
[01:10:02]    |       ^^^^^^^^^^^^^^^^^^^^ use of borrowed `y.0`
[01:10:02] ...
[01:10:02] LL |     *a += 1;
[01:10:02] 
[01:10:02] error[E0503]: cannot use `y` because it was mutably borrowed
[01:10:02]   --> /checkout/src/test/ui/borrowck/borrowck-anon-fields-variant.rs:37:7
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL |       Foo::Y(ref mut a, _) => a,
[01:10:02]    |              --------- borrow of `y.0` occurs here
[01:10:02] ...
[01:10:02] LL |       Foo::Y(ref mut b, _) => b, //~ ERROR cannot use `y`
[01:10:02]    |       ^^^^^^^^^^^^^^^^^^^^ use of borrowed `y.0`
[01:10:02] ...
[01:10:02] LL |     *a += 1;
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0499]: cannot borrow `y.0` as mutable more than once at a time
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL |       Foo::Y(ref mut a, _) => a,
[01:10:02]    |              --------- first mutable borrow occurs here
[01:10:02] ...
[01:10:02] LL |       Foo::Y(ref mut b, _) => b, //~ ERROR cannot use `y`
[01:10:02]    |              ^^^^^^^^^ second mutable borrow occurs here
[01:10:02] ...
[01:10:02] LL |     *a += 1;
[01:10:02]    |     ------- first borrow later used here
[01:10:02] error: aborting due to 3 previous errors
[01:10:02] 
[01:10:02] Some errors have detailed explanations: E0499, E0503.
[01:10:02] For more information about an error, try `rustc --explain E0499`.
---
[01:10:02] 
[01:10:02] 328 LL |         drop(x);
[01:10:02] 329    |              - mutable borrow later used here
[01:10:02] 330 
[01:10:02] - warning[E0502]: cannot borrow `*block.current` as immutable because it is also borrowed as mutable
[01:10:02] + error[E0502]: cannot borrow `*block.current` as immutable because it is also borrowed as mutable
[01:10:02] 333    |
[01:10:02] 334 LL |             let x = &mut block;
[01:10:02] 
[01:10:02] 338 ...
[01:10:02] 338 ...
[01:10:02] 339 LL |             drop(x);
[01:10:02] 340    |                  - mutable borrow later used here
[01:10:02] -    |
[01:10:02] -    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:10:02] -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:10:02] 344 
[01:10:02] - warning[E0502]: cannot borrow `*block.current` as immutable because it is also borrowed as mutable
[01:10:02] + error[E0502]: cannot borrow `*block.current` as immutable because it is also borrowed as mutable
[01:10:02] 347    |
[01:10:02] 348 LL |             let x = &mut block;
[01:10:02] 
[01:10:02] 352 ...
---
[01:10:02] 359 error[E0382]: use of moved value: `x`
[01:10:02] 360   --> $DIR/borrowck-describe-lvalue.rs:282:22
[01:10:02] 
[01:10:02] 366    |
[01:10:02] 367    = note: move occurs because `x` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
[01:10:02] - error: aborting due to 30 previous errors
[01:10:02] + error: aborting due to 32 previous errors
[01:10:02] 370 
[01:10:02] 371 Some errors have detailed explanations: E0382, E0499, E0502, E0503.
[01:10:02] 371 Some errors have detailed explanations: E0382, E0499, E0502, E0503.
[01:10:02] 372 For more information about an error, try `rustc --explain E0382`.
[01:10:02] 
[01:10:02] 
[01:10:02] The actual stderr differed from the expected stderr.
[01:10:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue/borrowck-describe-lvalue.stderr
[01:10:02] To update references, rerun the tests and pass the `--bless` flag
[01:10:02] To only update this specific test, also pass `--test-args borrowck/borrowck-describe-lvalue.rs`
[01:10:02] error: 1 errors occurred comparing output.
[01:10:02] status: exit code: 1
[01:10:02] status: exit code: 1
[01:10:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue/auxiliary" "-A" "unused"
[01:10:02] ------------------------------------------
[01:10:02] 
[01:10:02] ------------------------------------------
[01:10:02] stderr:
[01:10:02] stderr:
[01:10:02] ------------------------------------------
[01:10:02] error[E0499]: cannot borrow `x` as mutable more than once at a time
[01:10:02]    |
[01:10:02] LL |             let y = &mut x;
[01:10:02]    |                     ------ first mutable borrow occurs here
[01:10:02]    |                     ------ first mutable borrow occurs here
[01:10:02] LL |             &mut x; //~ ERROR cannot borrow `x` as mutable more than once at a time
[01:10:02]    |             ^^^^^^ second mutable borrow occurs here
[01:10:02] LL |             *y = 1;
[01:10:02]    |             ------ first borrow later used here
[01:10:02] 
[01:10:02] error[E0499]: cannot borrow `x` as mutable more than once at a time
[01:10:02]    |
[01:10:02] LL |                    let y = &mut x;
[01:10:02]    |                            ------ first mutable borrow occurs here
[01:10:02]    |                            ------ first mutable borrow occurs here
[01:10:02] LL |                    &mut x; //~ ERROR cannot borrow `x` as mutable more than once at a time
[01:10:02]    |                    ^^^^^^ second mutable borrow occurs here
[01:10:02] LL |                    *y = 1;
[01:10:02]    |                    ------ first borrow later used here
[01:10:02] 
[01:10:02] error: captured variable cannot escape `FnMut` closure body
[01:10:02]    |
[01:10:02] LL |              || {
[01:10:02] LL |              || {
[01:10:02]    |               - inferred to be a `FnMut` closure
[01:10:02] LL | /                || { //~ ERROR captured variable cannot escape `FnMut` closure body
[01:10:02] LL | |                    let y = &mut x;
[01:10:02] LL | |                    &mut x; //~ ERROR cannot borrow `x` as mutable more than once at a time
[01:10:02] LL | |                    *y = 1;
[01:10:02] LL | |                    drop(y);
[01:10:02]    | |_________________^ returns a closure that contains a reference to a captured variable, which then escapes the closure body
[01:10:02]    |
[01:10:02]    |
[01:10:02]    = note: `FnMut` closures only have access to their captured variables while they are executing...
[01:10:02]    = note: ...therefore, they cannot allow references to captured variables to escape
[01:10:02] 
[01:10:02] error[E0503]: cannot use `f.x` because it was mutably borrowed
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL |         let x = f.x();
[01:10:02]    |                 - borrow of `f` occurs here
[01:10:02] LL |         f.x; //~ ERROR cannot use `f.x` because it was mutably borrowed
[01:10:02]    |         ^^^ use of borrowed `f`
[01:10:02] LL |         drop(x);
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0503]: cannot use `g.0` because it was mutably borrowed
[01:10:02]    |
[01:10:02] LL |         let x = g.x();
[01:10:02] LL |         let x = g.x();
[01:10:02]    |                 - borrow of `g` occurs here
[01:10:02] LL |         g.0; //~ ERROR cannot use `g.0` because it was mutably borrowed
[01:10:02]    |         ^^^ use of borrowed `g`
[01:10:02] LL |         drop(x);
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0503]: cannot use `h.0` because it was mutably borrowed
[01:10:02]    |
[01:10:02] LL |         let x = &mut h.0;
[01:10:02] LL |         let x = &mut h.0;
[01:10:02]    |                 -------- borrow of `h.0` occurs here
[01:10:02] LL |         h.0; //~ ERROR cannot use `h.0` because it was mutably borrowed
[01:10:02]    |         ^^^ use of borrowed `h.0`
[01:10:02] LL |         drop(x);
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0503]: cannot use `e.0` because it was mutably borrowed
[01:10:02]    |
[01:10:02] LL |         let x = e.x();
[01:10:02]    |                 - borrow of `e` occurs here
[01:10:02] LL |         match e {
[01:10:02] LL |         match e {
[01:10:02] LL |             Baz::X(value) => value //~ ERROR cannot use `e.0` because it was mutably borrowed
[01:10:02]    |                    ^^^^^ use of borrowed `e`
[01:10:02] LL |         drop(x);
[01:10:02]    |              - borrow later used here
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0503]: cannot use `u.a` because it was mutably borrowed
[01:10:02]    |
[01:10:02] LL |         let x = &mut u.a;
[01:10:02] LL |         let x = &mut u.a;
[01:10:02]    |                 -------- borrow of `u.a` occurs here
[01:10:02] LL |         u.a; //~ ERROR cannot use `u.a` because it was mutably borrowed
[01:10:02]    |         ^^^ use of borrowed `u.a`
[01:10:02] LL |         drop(x);
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0503]: cannot use `f.x` because it was mutably borrowed
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL |         let x = f.x();
[01:10:02]    |                 - borrow of `*f` occurs here
[01:10:02] LL |         f.x; //~ ERROR cannot use `f.x` because it was mutably borrowed
[01:10:02]    |         ^^^ use of borrowed `*f`
[01:10:02] LL |         drop(x);
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0503]: cannot use `g.0` because it was mutably borrowed
[01:10:02]    |
[01:10:02] LL |         let x = g.x();
[01:10:02] LL |         let x = g.x();
[01:10:02]    |                 - borrow of `*g` occurs here
[01:10:02] LL |         g.0; //~ ERROR cannot use `g.0` because it was mutably borrowed
[01:10:02]    |         ^^^ use of borrowed `*g`
[01:10:02] LL |         drop(x);
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0503]: cannot use `h.0` because it was mutably borrowed
[01:10:02]    |
[01:10:02] LL |         let x = &mut h.0;
[01:10:02] LL |         let x = &mut h.0;
[01:10:02]    |                 -------- borrow of `h.0` occurs here
[01:10:02] LL |         h.0; //~ ERROR cannot use `h.0` because it was mutably borrowed
[01:10:02]    |         ^^^ use of borrowed `h.0`
[01:10:02] LL |         drop(x);
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0503]: cannot use `e.0` because it was mutably borrowed
[01:10:02]    |
[01:10:02] LL |         let x = e.x();
[01:10:02] LL |         let x = e.x();
[01:10:02]    |                 - borrow of `*e` occurs here
[01:10:02] LL |         match *e {
[01:10:02] LL |             Baz::X(value) => value
[01:10:02]    |                    ^^^^^ use of borrowed `*e`
[01:10:02] LL |         drop(x);
[01:10:02]    |              - borrow later used here
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0503]: cannot use `u.a` because it was mutably borrowed
[01:10:02]    |
[01:10:02] LL |         let x = &mut u.a;
[01:10:02] LL |         let x = &mut u.a;
[01:10:02]    |                 -------- borrow of `u.a` occurs here
[01:10:02] LL |         u.a; //~ ERROR cannot use `u.a` because it was mutably borrowed
[01:10:02]    |         ^^^ use of borrowed `u.a`
[01:10:02] LL |         drop(x);
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0503]: cannot use `v[..]` because it was mutably borrowed
[01:10:02]    |
[01:10:02] LL |         let x = &mut v;
[01:10:02]    |                 ------ borrow of `v` occurs here
[01:10:02] LL |         match v {
[01:10:02] LL |         match v {
[01:10:02] LL |             &[x, _, .., _, _] => println!("{}", x),
[01:10:02]    |               ^ use of borrowed `v`
[01:10:02] LL |         drop(x);
[01:10:02]    |              - borrow later used here
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0503]: cannot use `v[..]` because it was mutably borrowed
[01:10:02]    |
[01:10:02] LL |         let x = &mut v;
[01:10:02]    |                 ------ borrow of `v` occurs here
[01:10:02] ...
[01:10:02] ...
[01:10:02] LL |             &[_, x, .., _, _] => println!("{}", x),
[01:10:02]    |                  ^ use of borrowed `v`
[01:10:02] LL |         drop(x);
[01:10:02]    |              - borrow later used here
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0503]: cannot use `v[..]` because it was mutably borrowed
[01:10:02]    |
[01:10:02] LL |         let x = &mut v;
[01:10:02]    |                 ------ borrow of `v` occurs here
[01:10:02] ...
[01:10:02] ...
[01:10:02] LL |             &[_, _, .., x, _] => println!("{}", x),
[01:10:02]    |                         ^ use of borrowed `v`
[01:10:02] LL |         drop(x);
[01:10:02]    |              - borrow later used here
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0503]: cannot use `v[..]` because it was mutably borrowed
[01:10:02]    |
[01:10:02] LL |         let x = &mut v;
[01:10:02]    |                 ------ borrow of `v` occurs here
[01:10:02] ...
[01:10:02] ...
[01:10:02] LL |             &[_, _, .., _, x] => println!("{}", x),
[01:10:02]    |                            ^ use of borrowed `v`
[01:10:02] LL |         drop(x);
[01:10:02]    |              - borrow later used here
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0503]: cannot use `v[..]` because it was mutably borrowed
[01:10:02]    |
[01:10:02] LL |         let x = &mut v;
[01:10:02]    |                 ------ borrow of `v` occurs here
[01:10:02] LL |         match v {
[01:10:02] LL |         match v {
[01:10:02] LL |             &[x..] => println!("{:?}", x),
[01:10:02]    |               ^ use of borrowed `v`
[01:10:02] LL |         drop(x);
[01:10:02]    |              - borrow later used here
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0503]: cannot use `v[..]` because it was mutably borrowed
[01:10:02]    |
[01:10:02] LL |         let x = &mut v;
[01:10:02]    |                 ------ borrow of `v` occurs here
[01:10:02] ...
[01:10:02] ...
[01:10:02] LL |             &[_, x..] => println!("{:?}", x),
[01:10:02]    |                  ^ use of borrowed `v`
[01:10:02] LL |         drop(x);
[01:10:02]    |              - borrow later used here
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0503]: cannot use `v[..]` because it was mutably borrowed
[01:10:02]    |
[01:10:02] LL |         let x = &mut v;
[01:10:02]    |                 ------ borrow of `v` occurs here
[01:10:02] ...
[01:10:02] ...
[01:10:02] LL |             &[x.., _] => println!("{:?}", x),
[01:10:02]    |               ^ use of borrowed `v`
[01:10:02] LL |         drop(x);
[01:10:02]    |              - borrow later used here
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0503]: cannot use `v[..]` because it was mutably borrowed
[01:10:02]    |
[01:10:02] LL |         let x = &mut v;
[01:10:02]    |                 ------ borrow of `v` occurs here
[01:10:02] ...
[01:10:02] ...
[01:10:02] LL |             &[_, x.., _] => println!("{:?}", x),
[01:10:02]    |                  ^ use of borrowed `v`
[01:10:02] LL |         drop(x);
[01:10:02]    |              - borrow later used here
[01:10:02] 
[01:10:02] error[E0503]: cannot use `e` because it was mutably borrowed
[01:10:02] error[E0503]: cannot use `e` because it was mutably borrowed
[01:10:02]   --> /checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs:171:13
[01:10:02]    |
[01:10:02] LL |         let x = &mut e;
[01:10:02]    |                 ------ borrow of `e` occurs here
[01:10:02] LL |         match e {
[01:10:02] LL |             E::A(ref ax) =>
[01:10:02]    |             ^^^^^^^^^^^^ use of borrowed `e`
[01:10:02] LL |         drop(x);
[01:10:02]    |              - borrow later used here
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0502]: cannot borrow `e.0` as immutable because it is also borrowed as mutable
[01:10:02]    |
[01:10:02] LL |         let x = &mut e;
[01:10:02]    |                 ------ mutable borrow occurs here
[01:10:02] LL |         match e {
[01:10:02] LL |         match e {
[01:10:02] LL |             E::A(ref ax) =>
[01:10:02] ...
[01:10:02] LL |         drop(x);
[01:10:02]    |              - mutable borrow later used here
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0502]: cannot borrow `e.x` as immutable because it is also borrowed as mutable
[01:10:02]    |
[01:10:02] LL |         let x = &mut e;
[01:10:02]    |                 ------ mutable borrow occurs here
[01:10:02] ...
[01:10:02] ...
[01:10:02] LL |             E::B { x: ref bx } =>
[01:10:02] ...
[01:10:02] LL |         drop(x);
[01:10:02]    |              - mutable borrow later used here
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0502]: cannot borrow `s.y.0` as immutable because it is also borrowed as mutable
[01:10:02]    |
[01:10:02] LL |         let x = &mut s;
[01:10:02]    |                 ------ mutable borrow occurs here
[01:10:02] LL |         match s {
[01:10:02] LL |         match s {
[01:10:02] LL |             S  { y: (ref y0, _), .. } =>
[01:10:02] ...
[01:10:02] LL |         drop(x);
[01:10:02]    |              - mutable borrow later used here
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0502]: cannot borrow `s.x.y` as immutable because it is also borrowed as mutable
[01:10:02]    |
[01:10:02] LL |         let x = &mut s;
[01:10:02]    |                 ------ mutable borrow occurs here
[01:10:02] ...
[01:10:02] ...
[01:10:02] LL |             S  { x: F { y: ref x0, .. }, .. } =>
[01:10:02] ...
[01:10:02] LL |         drop(x);
[01:10:02]    |              - mutable borrow later used here
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0503]: cannot use `*v` because it was mutably borrowed
[01:10:02]    |
[01:10:02] LL |         let x = &mut v;
[01:10:02]    |                 ------ borrow of `v` occurs here
[01:10:02] LL |         v[0].y;
[01:10:02] LL |         v[0].y;
[01:10:02]    |         ^^^^ use of borrowed `v`
[01:10:02] ...
[01:10:02] LL |         drop(x);
[01:10:02]    |              - borrow later used here
[01:10:02] 
[01:10:02] error[E0503]: cannot use `v[_].y` because it was mutably borrowed
[01:10:02]    |
[01:10:02] LL |         let x = &mut v;
[01:10:02]    |                 ------ borrow of `v` occurs here
[01:10:02] LL |         v[0].y;
[01:10:02] LL |         v[0].y;
[01:10:02]    |         ^^^^^^ use of borrowed `v`
[01:10:02] ...
[01:10:02] LL |         drop(x);
[01:10:02]    |              - borrow later used here
[01:10:02] 
[01:10:02] error[E0502]: cannot borrow `v[..].x` as immutable because it is also borrowed as mutable
[01:10:02]    |
[01:10:02] LL |         let x = &mut v;
[01:10:02]    |                 ------ mutable borrow occurs here
[01:10:02] LL |         match v {
[01:10:02] LL |         match v {
[01:10:02] LL |             &[_, F {x: ref xf, ..}] => println!("{}", xf),
[01:10:02] ...
[01:10:02] LL |         drop(x);
[01:10:02]    |              - mutable borrow later used here
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0502]: cannot borrow `*block.current` as immutable because it is also borrowed as mutable
[01:10:02]    |
[01:10:02] LL |             let x = &mut block;
[01:10:02]    |                     ---------- mutable borrow occurs here
[01:10:02]    |                     ---------- mutable borrow occurs here
[01:10:02] LL |             let p: &'a u8 = &*block.current;
[01:10:02] ...
[01:10:02] LL |             drop(x);
[01:10:02]    |                  - mutable borrow later used here
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0502]: cannot borrow `*block.current` as immutable because it is also borrowed as mutable
[01:10:02]    |
[01:10:02] LL |             let x = &mut block;
[01:10:02]    |                     ---------- mutable borrow occurs here
[01:10:02]    |                     ---------- mutable borrow occurs here
[01:10:02] LL |             let p : *const u8 = &*(*block).current;
[01:10:02] ...
[01:10:02] LL |             drop(x);
[01:10:02]    |                  - mutable borrow later used here
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0382]: use of moved value: `x`
[01:10:02]   --> /checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs:282:22
[01:10:02]    |
[01:10:02] LL |                 drop(x);
[01:10:02]    |                      - value moved here
[01:10:02] LL |                 drop(x); //~ ERROR use of moved value: `x`
[01:10:02]    |                      ^ value used here after move
[01:10:02]    |
[01:10:02]    = note: move occurs because `x` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
[01:10:02] error: aborting due to 32 previous errors
[01:10:02] 
[01:10:02] Some errors have detailed explanations: E0382, E0499, E0502, E0503.
[01:10:02] For more information about an error, try `rustc --explain E0382`.
[01:10:02] For more information about an error, try `rustc --explain E0382`.
[01:10:02] 
[01:10:02] ------------------------------------------
[01:10:02] 
[01:10:02] 
[01:10:02] ---- [ui] ui/borrowck/borrowck-migrate-to-nll.rs#edition stdout ----
[01:10:02] 
[01:10:02] error in revision `edition`: test compilation failed although it shouldn't!
[01:10:02] status: exit code: 1
[01:10:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-migrate-to-nll.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "edition" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-migrate-to-nll.edition/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-migrate-to-nll.edition/auxiliary" "-A" "unused"
[01:10:02] ------------------------------------------
[01:10:02] 
[01:10:02] ------------------------------------------
[01:10:02] stderr:
[01:10:02] stderr:
[01:10:02] ------------------------------------------
[01:10:02] error[E0507]: cannot move out of borrowed content
[01:10:02]   --> /checkout/src/test/ui/borrowck/borrowck-migrate-to-nll.rs:25:17
[01:10:02]    |
[01:10:02] LL |                 (|| { let bar = foo; bar.take() })();
[01:10:02]    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot move out of borrowed content
[01:10:02] error: aborting due to previous error
[01:10:02] 
[01:10:02] For more information about this error, try `rustc --explain E0507`.
[01:10:02] 
[01:10:02] 
[01:10:02] ------------------------------------------
[01:10:02] 
[01:10:02] 
[01:10:02] ---- [ui] ui/borrowck/borrowck-migrate-to-nll.rs#zflag stdout ----
[01:10:02] 
[01:10:02] error in revision `zflag`: test compilation failed although it shouldn't!
[01:10:02] status: exit code: 1
[01:10:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-migrate-to-nll.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "zflag" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-migrate-to-nll.zflag/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=migrate" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-migrate-to-nll.zflag/auxiliary" "-A" "unused"
[01:10:02] ------------------------------------------
[01:10:02] 
[01:10:02] ------------------------------------------
[01:10:02] stderr:
[01:10:02] stderr:
[01:10:02] ------------------------------------------
[01:10:02] error[E0507]: cannot move out of borrowed content
[01:10:02]   --> /checkout/src/test/ui/borrowck/borrowck-migrate-to-nll.rs:25:17
[01:10:02]    |
[01:10:02] LL |                 (|| { let bar = foo; bar.take() })();
[01:10:02]    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot move out of borrowed content
[01:10:02] error: aborting due to previous error
[01:10:02] 
[01:10:02] For more information about this error, try `rustc --explain E0507`.
[01:10:02] 
[01:10:02] 
[01:10:02] ------------------------------------------
[01:10:02] 
[01:10:02] 
[01:10:02] ---- [ui] ui/borrowck/borrowck-mutate-in-guard.rs stdout ----
[01:10:02] diff of stderr:
[01:10:02] 
[01:10:02] 18 LL |         Enum::A(_) if { let y = &mut x; *y = Enum::B(false); false } => 1,
[01:10:02] 19    |                                         ^^^^^^^^^^^^^^^^^^^ assignment in pattern guard
[01:10:02] 20 
[01:10:02] - warning[E0510]: cannot assign `x` in match guard
[01:10:02] + error[E0510]: cannot assign `x` in match guard
[01:10:02] 23    |
[01:10:02] 24 LL |     match x {
[01:10:02] 
[01:10:02] 
[01:10:02] 25    |           - value is immutable in match guard
[01:10:02] 26 LL |         Enum::A(_) if { x = Enum::B(false); false } => 1,
[01:10:02] -    |
[01:10:02] -    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:10:02] -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:10:02] 31 
[01:10:02] 31 
[01:10:02] - warning[E0510]: cannot mutably borrow `x` in match guard
[01:10:02] + error[E0510]: cannot mutably borrow `x` in match guard
[01:10:02] 34    |
[01:10:02] 35 LL |     match x {
[01:10:02] 
[01:10:02] 37 ...
[01:10:02] 37 ...
[01:10:02] 38 LL |         Enum::A(_) if { let y = &mut x; *y = Enum::B(false); false } => 1,
[01:10:02] -    |
[01:10:02] -    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:10:02] -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:10:02] 43 
---
[01:10:02] 47 For more information about an error, try `rustc --explain E0301`.
[01:10:02] 
[01:10:02] 
[01:10:02] The actual stderr differed from the expected stderr.
[01:10:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-mutate-in-guard/borrowck-mutate-in-guard.stderr
[01:10:02] To update references, rerun the tests and pass the `--bless` flag
[01:10:02] To only update this specific test, also pass `--test-args borrowck/borrowck-mutate-in-guard.rs`
[01:10:02] error: 1 errors occurred comparing output.
[01:10:02] status: exit code: 1
[01:10:02] status: exit code: 1
[01:10:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-mutate-in-guard.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-mutate-in-guard" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-mutate-in-guard/auxiliary" "-A" "unused"
[01:10:02] ------------------------------------------
[01:10:02] 
[01:10:02] ------------------------------------------
[01:10:02] stderr:
[01:10:02] stderr:
[01:10:02] ------------------------------------------
[01:10:02] error[E0302]: cannot assign in a pattern guard
[01:10:02]   --> /checkout/src/test/ui/borrowck/borrowck-mutate-in-guard.rs:10:25
[01:10:02]    |
[01:10:02] LL |         Enum::A(_) if { x = Enum::B(false); false } => 1,
[01:10:02]    |                         ^^^^^^^^^^^^^^^^^^ assignment in pattern guard
[01:10:02] error[E0301]: cannot mutably borrow in a pattern guard
[01:10:02]   --> /checkout/src/test/ui/borrowck/borrowck-mutate-in-guard.rs:15:38
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL |         Enum::A(_) if { let y = &mut x; *y = Enum::B(false); false } => 1,
[01:10:02]    |                                      ^ borrowed mutably in pattern guard
[01:10:02]    |
[01:10:02]    = help: add #![feature(bind_by_move_pattern_guards)] to the crate attributes to enable
[01:10:02] error[E0302]: cannot assign in a pattern guard
[01:10:02]   --> /checkout/src/test/ui/borrowck/borrowck-mutate-in-guard.rs:15:41
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL |         Enum::A(_) if { let y = &mut x; *y = Enum::B(false); false } => 1,
[01:10:02]    |                                         ^^^^^^^^^^^^^^^^^^^ assignment in pattern guard
[01:10:02] 
[01:10:02] error[E0510]: cannot assign `x` in match guard
[01:10:02]    |
[01:10:02] LL |     match x {
[01:10:02] LL |     match x {
[01:10:02]    |           - value is immutable in match guard
[01:10:02] LL |         Enum::A(_) if { x = Enum::B(false); false } => 1,
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0510]: cannot mutably borrow `x` in match guard
[01:10:02]    |
[01:10:02] LL |     match x {
[01:10:02] LL |     match x {
[01:10:02]    |           - value is immutable in match guard
[01:10:02] ...
[01:10:02] LL |         Enum::A(_) if { let y = &mut x; *y = Enum::B(false); false } => 1,
[01:10:02] 
[01:10:02] error: aborting due to 5 previous errors
[01:10:02] 
[01:10:02] Some errors have detailed explanations: E0301, E0302, E0510.
---
[01:10:02] 
[01:10:02] 13    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02] 14    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 15 
[01:10:02] - warning[E0381]: use of possibly uninitialized variable: `a`
[01:10:02] + error[E0381]: use of possibly uninitialized variable: `a`
[01:10:02] 18    |
[01:10:02] 19 LL |     a + b
[01:10:02] 
[01:10:02] 
[01:10:02] 20    |     ^ use of possibly uninitialized `a`
[01:10:02] -    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:10:02] -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:10:02] 24 
[01:10:02] 24 
[01:10:02] - warning[E0381]: use of possibly uninitialized variable: `b`
[01:10:02] + error[E0381]: use of possibly uninitialized variable: `b`
[01:10:02] 27    |
[01:10:02] 28 LL |     a + b
[01:10:02] 
[01:10:02] 
[01:10:02] 29    |         ^ use of possibly uninitialized `b`
[01:10:02] -    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:10:02] -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:10:02] 33 
[01:10:02] - error: aborting due to 2 previous errors
---
[01:10:02] 
[01:10:02] 
[01:10:02] The actual stderr differed from the expected stderr.
[01:10:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_let_refutable/const_let_refutable.stderr
[01:10:02] To update references, rerun the tests and pass the `--bless` flag
[01:10:02] To only update this specific test, also pass `--test-args consts/const_let_refutable.rs`
[01:10:02] error: 1 errors occurred comparing output.
[01:10:02] status: exit code: 1
[01:10:02] status: exit code: 1
[01:10:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_let_refutable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_let_refutable" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_let_refutable/auxiliary" "-A" "unused"
[01:10:02] ------------------------------------------
[01:10:02] 
[01:10:02] ------------------------------------------
[01:10:02] stderr:
[01:10:02] stderr:
[01:10:02] ------------------------------------------
[01:10:02] error[E0005]: refutable pattern in function argument: `&[]` not covered
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL | const fn slice([a, b]: &[i32]) -> i32 { //~ ERROR refutable pattern in function argument
[01:10:02]    |                ^^^^^^ pattern `&[]` not covered
[01:10:02] 
[01:10:02] error[E0723]: can only call other `const fn` within a `const fn`, but `const std::ops::Add::add` is not stable as `const fn`
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL |     a + b //~ ERROR can only call other `const fn` within a `const fn`
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0381]: use of possibly uninitialized variable: `a`
[01:10:02]   --> /checkout/src/test/ui/consts/const_let_refutable.rs:4:5
[01:10:02]    |
[01:10:02] LL |     a + b //~ ERROR can only call other `const fn` within a `const fn`
[01:10:02]    |     ^ use of possibly uninitialized `a`
[01:10:02] error[E0381]: use of possibly uninitialized variable: `b`
[01:10:02]   --> /checkout/src/test/ui/consts/const_let_refutable.rs:4:9
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL |     a + b //~ ERROR can only call other `const fn` within a `const fn`
[01:10:02]    |         ^ use of possibly uninitialized `b`
[01:10:02] error: aborting due to 4 previous errors
[01:10:02] 
[01:10:02] Some errors have detailed explanations: E0005, E0381, E0723.
[01:10:02] For more information about an error, try `rustc --explain E0005`.
---
[01:10:02] - warning[E0716]: temporary value dropped while borrowed
[01:10:02] + error[E0716]: temporary value dropped while borrowed
[01:10:02] 20   --> $DIR/min_const_fn_dyn.rs:12:67
[01:10:02] 21    |
[01:10:02] 22 LL | const fn no_inner_dyn_trait_ret() -> Hide { Hide(HasDyn { field: &0 }) }
[01:10:02] 24    |                                                                  ||
[01:10:02] 25    |                                                                  |creates a temporary which is freed while still in use
[01:10:02] 26    |                                                                  cast requires that borrow lasts for `'static`
[01:10:02] -    |
---
[01:10:02] 34 For more information about an error, try `rustc --explain E0716`.
[01:10:02] 
[01:10:02] 
[01:10:02] The actual stderr differed from the expected stderr.
[01:10:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_dyn/min_const_fn_dyn.stderr
[01:10:02] To update references, rerun the tests and pass the `--bless` flag
[01:10:02] To only update this specific test, also pass `--test-args consts/min_const_fn/min_const_fn_dyn.rs`
[01:10:02] error: 1 errors occurred comparing output.
[01:10:02] status: exit code: 1
[01:10:02] status: exit code: 1
[01:10:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/min_const_fn_dyn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_dyn" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_dyn/auxiliary" "-A" "unused"
[01:10:02] ------------------------------------------
[01:10:02] 
[01:10:02] ------------------------------------------
[01:10:02] stderr:
[01:10:02] stderr:
[01:10:02] ------------------------------------------
[01:10:02] error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn_dyn.rs:9:5
[01:10:02]    |
[01:10:02] LL |     x.0.field;
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn_dyn.rs:12:66
[01:10:02]    |
[01:10:02] LL | const fn no_inner_dyn_trait_ret() -> Hide { Hide(HasDyn { field: &0 }) }
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0716]: temporary value dropped while borrowed
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn_dyn.rs:12:67
[01:10:02]    |
[01:10:02] LL | const fn no_inner_dyn_trait_ret() -> Hide { Hide(HasDyn { field: &0 }) }
[01:10:02]    |                                                                  -^    - temporary value is freed at the end of this statement
[01:10:02]    |                                                                  |creates a temporary which is freed while still in use
[01:10:02]    |                                                                  cast requires that borrow lasts for `'static`
[01:10:02] 
[01:10:02] error: aborting due to 3 previous errors
---
[01:10:02] - warning[E0515]: cannot return reference to temporary value
[01:10:02] + error[E0515]: cannot return reference to temporary value
[01:10:02] 290   --> $DIR/min_const_fn.rs:136:63
[01:10:02] 291    |
[01:10:02] 292 LL | const fn no_dyn_trait_ret() -> &'static dyn std::fmt::Debug { &() }
[01:10:02] 294    |                                                               ||
[01:10:02] 295    |                                                               |temporary value created here
[01:10:02] 296    |                                                               returns a reference to data owned by the current function
[01:10:02] -    |
---
[01:10:02] 
[01:10:02] 
[01:10:02] The actual stderr differed from the expected stderr.
[01:10:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn/min_const_fn.stderr
[01:10:02] To update references, rerun the tests and pass the `--bless` flag
[01:10:02] To only update this specific test, also pass `--test-args consts/min_const_fn/min_const_fn.rs`
[01:10:02] error: 1 errors occurred comparing output.
[01:10:02] status: exit code: 1
[01:10:02] status: exit code: 1
[01:10:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn/auxiliary" "-A" "unused"
[01:10:02] ------------------------------------------
[01:10:02] 
[01:10:02] ------------------------------------------
[01:10:02] stderr:
[01:10:02] stderr:
[01:10:02] ------------------------------------------
[01:10:02] error[E0493]: destructors cannot be evaluated at compile-time
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:37:25
[01:10:02]    |
[01:10:02] LL |     const fn into_inner(self) -> T { self.0 } //~ destructors cannot be evaluated
[01:10:02]    |                         ^^^^ constant functions cannot evaluate destructors
[01:10:02] error[E0723]: mutable references in const fn are unstable
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:39:36
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL |     const fn get_mut(&mut self) -> &mut T { &mut self.0 }
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0493]: destructors cannot be evaluated at compile-time
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:44:28
[01:10:02]    |
[01:10:02] LL |     const fn into_inner_lt(self) -> T { self.0 } //~ destructors cannot be evaluated
[01:10:02]    |                            ^^^^ constant functions cannot evaluate destructors
[01:10:02] error[E0723]: mutable references in const fn are unstable
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:46:42
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL |     const fn get_mut_lt(&'a mut self) -> &mut T { &mut self.0 }
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0493]: destructors cannot be evaluated at compile-time
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:51:27
[01:10:02]    |
[01:10:02] LL |     const fn into_inner_s(self) -> T { self.0 } //~ ERROR destructors
[01:10:02]    |                           ^^^^ constant functions cannot evaluate destructors
[01:10:02] error[E0723]: mutable references in const fn are unstable
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:53:38
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL |     const fn get_mut_s(&mut self) -> &mut T { &mut self.0 }
[01:10:02]    |
[01:10:02] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:10:02] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] error[E0723]: mutable references in const fn are unstable
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:58:39
[01:10:02]    |
[01:10:02] LL |     const fn get_mut_sq(&mut self) -> &mut T { &mut self.0 }
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:76:16
[01:10:02]    |
[01:10:02] LL | const fn foo11<T: std::fmt::Display>(t: T) -> T { t }
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:78:18
[01:10:02]    |
[01:10:02] LL | const fn foo11_2<T: Send>(t: T) -> T { t }
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: only int, `bool` and `char` operations are stable in const fn
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL | const fn foo19(f: f32) -> f32 { f * 2.0 }
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: only int, `bool` and `char` operations are stable in const fn
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL | const fn foo19_2(f: f32) -> f32 { 2.0 - f }
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: only int and `bool` operations are stable in const fn
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:84:35
[01:10:02]    |
[01:10:02] LL | const fn foo19_3(f: f32) -> f32 { -f }
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: only int, `bool` and `char` operations are stable in const fn
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL | const fn foo19_4(f: f32, g: f32) -> f32 { f / g }
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: cannot access `static` items in const fn
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:90:27
[01:10:02]    |
[01:10:02] LL | const fn foo25() -> u32 { BAR } //~ ERROR cannot access `static` items in const fn
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: cannot access `static` items in const fn
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:91:36
[01:10:02]    |
[01:10:02] LL | const fn foo26() -> &'static u32 { &BAR } //~ ERROR cannot access `static` items
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: casting pointers to ints is unstable in const fn
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:92:42
[01:10:02]    |
[01:10:02] LL | const fn foo30(x: *const u32) -> usize { x as usize }
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: casting pointers to ints is unstable in const fn
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:94:63
[01:10:02]    |
[01:10:02] LL | const fn foo30_with_unsafe(x: *const u32) -> usize { unsafe { x as usize } }
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: casting pointers to ints is unstable in const fn
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:96:42
[01:10:02]    |
[01:10:02] LL | const fn foo30_2(x: *mut u32) -> usize { x as usize }
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: casting pointers to ints is unstable in const fn
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:98:63
[01:10:02]    |
[01:10:02] LL | const fn foo30_2_with_unsafe(x: *mut u32) -> usize { unsafe { x as usize } }
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: `if`, `match`, `&&` and `||` are not stable in const fn
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL | const fn foo30_4(b: bool) -> usize { if b { 1 } else { 42 } }
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: `if`, `match`, `&&` and `||` are not stable in const fn
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL | const fn foo30_5(b: bool) { while b { } } //~ ERROR not stable in const fn
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: `if`, `match`, `&&` and `||` are not stable in const fn
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL | const fn foo36(a: bool, b: bool) -> bool { a && b }
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: `if`, `match`, `&&` and `||` are not stable in const fn
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL | const fn foo37(a: bool, b: bool) -> bool { a || b }
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: mutable references in const fn are unstable
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:108:14
[01:10:02]    |
[01:10:02] LL | const fn inc(x: &mut i32) { *x += 1 }
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:113:6
[01:10:02]    |
[01:10:02] LL | impl<T: std::fmt::Debug> Foo<T> {
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:118:6
[01:10:02]    |
[01:10:02] LL | impl<T: std::fmt::Debug + Sized> Foo<T> {
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:123:6
[01:10:02]    |
[01:10:02] LL | impl<T: Sync + Sized> Foo<T> {
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: `impl Trait` in const fn is unstable
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL | const fn no_rpit2() -> AlanTuring<impl std::fmt::Debug> { AlanTuring(0) }
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:131:34
[01:10:02]    |
[01:10:02] LL | const fn no_apit2(_x: AlanTuring<impl std::fmt::Debug>) {}
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:133:22
[01:10:02]    |
[01:10:02] LL | const fn no_apit(_x: impl std::fmt::Debug) {} //~ ERROR trait bounds other than `Sized`
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: `impl Trait` in const fn is unstable
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL | const fn no_rpit() -> impl std::fmt::Debug {} //~ ERROR `impl Trait` in const fn is unstable
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:135:23
[01:10:02]    |
[01:10:02] LL | const fn no_dyn_trait(_x: &dyn std::fmt::Debug) {} //~ ERROR trait bounds other than `Sized`
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:136:32
[01:10:02]    |
[01:10:02] LL | const fn no_dyn_trait_ret() -> &'static dyn std::fmt::Debug { &() }
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0515]: cannot return reference to temporary value
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:136:63
[01:10:02]    |
[01:10:02] LL | const fn no_dyn_trait_ret() -> &'static dyn std::fmt::Debug { &() }
[01:10:02]    |                                                               ^--
[01:10:02]    |                                                               |temporary value created here
[01:10:02]    |                                                               returns a reference to data owned by the current function
[01:10:02] 
[01:10:02] error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
[01:10:02] error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:144:41
[01:10:02]    |
[01:10:02] LL | const fn really_no_traits_i_mean_it() { (&() as &std::fmt::Debug, ()).1 }
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: function pointers in const fn are unstable
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:147:21
[01:10:02]    |
[01:10:02] LL | const fn no_fn_ptrs(_x: fn()) {}
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0723]: function pointers in const fn are unstable
[01:10:02]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:149:27
[01:10:02]    |
[01:10:02] LL | const fn no_fn_ptrs2() -> fn() { fn foo() {} foo }
[01:10:02]    |
[01:10:02]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:02]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:02] 
---
[01:10:02] 
[01:10:02] ---- [ui] ui/empty/empty-never-array.rs stdout ----
[01:10:02] diff of stderr:
[01:10:02] 
[01:10:02] 11 LL |       let Helper::U(u) = Helper::T(t, []);
[01:10:02] 12    |           ^^^^^^^^^^^^ pattern `T(_, _)` not covered
[01:10:02] 13 
[01:10:02] - warning[E0381]: use of possibly uninitialized variable: `u`
[01:10:02] + error[E0381]: use of possibly uninitialized variable: `u`
[01:10:02] 16    |
[01:10:02] 17 LL |     u
[01:10:02] 
[01:10:02] 
[01:10:02] 18    |     ^ use of possibly uninitialized `u`
[01:10:02] -    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:10:02] -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:10:02] 22 
[01:10:02] - error: aborting due to previous error
[01:10:02] - error: aborting due to previous error
[01:10:02] + error: aborting due to 2 previous errors
[01:10:02] 24 
[01:10:02] 25 Some errors have detailed explanations: E0005, E0381.
[01:10:02] 26 For more information about an error, try `rustc --explain E0005`.
[01:10:02] 
[01:10:02] 
[01:10:02] The actual stderr differed from the expected stderr.
[01:10:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-never-array/empty-never-array.stderr
[01:10:02] To update references, rerun the tests and pass the `--bless` flag
[01:10:02] To only update this specific test, also pass `--test-args empty/empty-never-array.rs`
[01:10:02] error: 1 errors occurred comparing output.
[01:10:02] status: exit code: 1
[01:10:02] status: exit code: 1
[01:10:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/empty/empty-never-array.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-never-array" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-never-array/auxiliary" "-A" "unused"
[01:10:02] ------------------------------------------
[01:10:02] 
[01:10:02] ------------------------------------------
[01:10:02] stderr:
[01:10:02] stderr:
[01:10:02] ------------------------------------------
[01:10:02] error[E0005]: refutable pattern in local binding: `T(_, _)` not covered
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL | / enum Helper<T, U> {
[01:10:02] LL | |     T(T, [!; 0]),
[01:10:02] LL | |     #[allow(dead_code)]
[01:10:02] LL | |     U(U),
[01:10:02] LL | | }
[01:10:02]    | |_- `Helper<T, U>` defined here
[01:10:02] ...
[01:10:02] LL |       let Helper::U(u) = Helper::T(t, []);
[01:10:02]    |           ^^^^^^^^^^^^ pattern `T(_, _)` not covered
[01:10:02] error[E0381]: use of possibly uninitialized variable: `u`
[01:10:02]   --> /checkout/src/test/ui/empty/empty-never-array.rs:12:5
[01:10:02]    |
[01:10:02] LL |     u
[01:10:02] LL |     u
[01:10:02]    |     ^ use of possibly uninitialized `u`
[01:10:02] error: aborting due to 2 previous errors
[01:10:02] 
[01:10:02] Some errors have detailed explanations: E0005, E0381.
[01:10:02] For more information about an error, try `rustc --explain E0005`.
[01:10:02] For more information about an error, try `rustc --explain E0005`.
[01:10:02] 
[01:10:02] ------------------------------------------
[01:10:02] 
[01:10:02] 
[01:10:02] ---- [ui] ui/feature-gates/feature-gate-nll.rs stdout ----
[01:10:02] diff of stderr:
[01:10:02] 
[01:10:02] - warning[E0502]: cannot borrow `*x.1` as immutable because it is also borrowed as mutable
[01:10:02] + error[E0502]: cannot borrow `*x.1` as immutable because it is also borrowed as mutable
[01:10:02] 3    |
[01:10:02] 4 LL |     let m = &mut x;
[01:10:02] 
[01:10:02] 8 ...
---
[01:10:02] - 
[01:10:02] - error: compilation successful
[01:10:02] -   --> $DIR/feature-gate-nll.rs:10:1
[01:10:02] -    |
[01:10:02] - LL | / fn main() {
[01:10:02] - LL | |     let mut x = (33, &0);
[01:10:02] - LL | |
[01:10:02] - LL | |     let m = &mut x;
[01:10:02] - ...  |
[01:10:02] - LL | |     m;
[01:10:02] - LL | | }
[01:10:02] 26 
[01:10:02] 27 error: aborting due to previous error
[01:10:02] 28 
[01:10:02] 
[01:10:02] 
[01:10:02] 
[01:10:02] The actual stderr differed from the expected stderr.
[01:10:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-nll/feature-gate-nll.stderr
[01:10:02] To update references, rerun the tests and pass the `--bless` flag
[01:10:02] To only update this specific test, also pass `--test-args feature-gates/feature-gate-nll.rs`
[01:10:02] error: 1 errors occurred comparing output.
[01:10:02] status: exit code: 1
[01:10:02] status: exit code: 1
[01:10:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-nll.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-nll" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-nll/auxiliary" "-A" "unused"
[01:10:02] ------------------------------------------
[01:10:02] 
[01:10:02] ------------------------------------------
[01:10:02] stderr:
[01:10:02] stderr:
[01:10:02] ------------------------------------------
[01:10:02] error[E0502]: cannot borrow `*x.1` as immutable because it is also borrowed as mutable
[01:10:02]    |
[01:10:02] LL |     let m = &mut x;
[01:10:02]    |             ------ mutable borrow occurs here
[01:10:02]    |             ------ mutable borrow occurs here
[01:10:02] LL |     let p = &*x.1;
[01:10:02] ...
[01:10:02] LL |     m;
[01:10:02]    |     - mutable borrow later used here
[01:10:02] 
---
[01:10:02] 
[01:10:02] ---- [ui] ui/issues/issue-15381.rs stdout ----
[01:10:02] diff of stderr:
[01:10:02] 
[01:10:02] 4 LL |     for &[x,y,z] in values.chunks(3).filter(|&xs| xs.len() == 3) {
[01:10:02] 5    |         ^^^^^^^^ pattern `&[]` not covered
[01:10:02] - warning[E0381]: borrow of possibly uninitialized variable: `y`
[01:10:02] - warning[E0381]: borrow of possibly uninitialized variable: `y`
[01:10:02] + error[E0381]: borrow of possibly uninitialized variable: `y`
[01:10:02] 9    |
[01:10:02] 9    |
[01:10:02] 10 LL |         println!("y={}", y);
[01:10:02] 
[01:10:02] 11    |                          ^ use of possibly uninitialized `y`
[01:10:02] -    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:10:02] -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:10:02] 15 
[01:10:02] - error: aborting due to previous error
---
[01:10:02] 
[01:10:02] 
[01:10:02] The actual stderr differed from the expected stderr.
[01:10:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-15381/issue-15381.stderr
[01:10:02] To update references, rerun the tests and pass the `--bless` flag
[01:10:02] To only update this specific test, also pass `--test-args issues/issue-15381.rs`
[01:10:02] error: 1 errors occurred comparing output.
[01:10:02] status: exit code: 1
[01:10:02] status: exit code: 1
[01:10:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-15381.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-15381" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-15381/auxiliary" "-A" "unused"
[01:10:02] ------------------------------------------
[01:10:02] 
[01:10:02] ------------------------------------------
[01:10:02] stderr:
[01:10:02] stderr:
[01:10:02] ------------------------------------------
[01:10:02] error[E0005]: refutable pattern in `for` loop binding: `&[]` not covered
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL |     for &[x,y,z] in values.chunks(3).filter(|&xs| xs.len() == 3) {
[01:10:02]    |         ^^^^^^^^ pattern `&[]` not covered
[01:10:02] error[E0381]: borrow of possibly uninitialized variable: `y`
[01:10:02]   --> /checkout/src/test/ui/issues/issue-15381.rs:6:26
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL |         println!("y={}", y);
[01:10:02]    |                          ^ use of possibly uninitialized `y`
[01:10:02] error: aborting due to 2 previous errors
[01:10:02] 
[01:10:02] Some errors have detailed explanations: E0005, E0381.
[01:10:02] For more information about an error, try `rustc --explain E0005`.
[01:10:02] For more information about an error, try `rustc --explain E0005`.
[01:10:02] 
[01:10:02] ------------------------------------------
[01:10:02] 
[01:10:02] 
[01:10:02] ---- [ui] ui/issues/issue-40510-1.rs stdout ----
[01:10:02] 
[01:10:02] error: test compilation failed although it shouldn't!
[01:10:02] status: exit code: 1
[01:10:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-40510-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40510-1" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40510-1/auxiliary" "-A" "unused"
[01:10:02] ------------------------------------------
[01:10:02] 
[01:10:02] ------------------------------------------
[01:10:02] stderr:
[01:10:02] stderr:
[01:10:02] ------------------------------------------
[01:10:02] error: captured variable cannot escape `FnMut` closure body
[01:10:02]    |
[01:10:02] LL |     || {
[01:10:02] LL |     || {
[01:10:02]    |      - inferred to be a `FnMut` closure
[01:10:02] LL |         &mut x
[01:10:02]    |         ^^^^^^ returns a reference to a captured variable which escapes the closure body
[01:10:02]    |
[01:10:02]    = note: `FnMut` closures only have access to their captured variables while they are executing...
[01:10:02]    = note: ...therefore, they cannot allow references to captured variables to escape
[01:10:02] error: aborting due to previous error
[01:10:02] 
[01:10:02] 
[01:10:02] ------------------------------------------
[01:10:02] ------------------------------------------
[01:10:02] 
[01:10:02] 
[01:10:02] ---- [ui] ui/issues/issue-40510-3.rs stdout ----
[01:10:02] 
[01:10:02] error: test compilation failed although it shouldn't!
[01:10:02] status: exit code: 1
[01:10:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-40510-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40510-3" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40510-3/auxiliary" "-A" "unused"
[01:10:02] ------------------------------------------
[01:10:02] 
[01:10:02] ------------------------------------------
[01:10:02] stderr:
[01:10:02] stderr:
[01:10:02] ------------------------------------------
[01:10:02] error: captured variable cannot escape `FnMut` closure body
[01:10:02]    |
[01:10:02] LL |       || {
[01:10:02] LL |       || {
[01:10:02]    |        - inferred to be a `FnMut` closure
[01:10:02] LL | |             x.push(())
[01:10:02] LL | |         }
[01:10:02]    | |_________^ returns a closure that contains a reference to a captured variable, which then escapes the closure body
[01:10:02]    |
[01:10:02]    |
[01:10:02]    = note: `FnMut` closures only have access to their captured variables while they are executing...
[01:10:02]    = note: ...therefore, they cannot allow references to captured variables to escape
[01:10:02] error: aborting due to previous error
[01:10:02] 
[01:10:02] 
[01:10:02] ------------------------------------------
---
[01:10:02] - warning[E0713]: borrow may still be in use when destructor runs
[01:10:02] + error[E0713]: borrow may still be in use when destructor runs
[01:10:02] 2   --> $DIR/issue-45696-scribble-on-boxed-borrow.rs:51:5
[01:10:02] 3    |
[01:10:02] 4 LL | fn scribbled<'a>(s: Scribble<'a>) -> &'a mut u32 {
[01:10:02] 8 ...
[01:10:02] 9 LL | }
[01:10:02] 9 LL | }
[01:10:02] 10    | - here, drop of `s` needs exclusive access to `*s.0`, because the type `Scribble<'_>` implements the `Drop` trait
[01:10:02] -    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:10:02] -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:10:02] 14 
[01:10:02] - warning[E0713]: borrow may still be in use when destructor runs
[01:10:02] - warning[E0713]: borrow may still be in use when destructor runs
[01:10:02] + error[E0713]: borrow may still be in use when destructor runs
[01:10:02] 16   --> $DIR/issue-45696-scribble-on-boxed-borrow.rs:62:5
[01:10:02] 17    |
[01:10:02] 18 LL | fn boxed_scribbled<'a>(s: Box<Scribble<'a>>) -> &'a mut u32 {
[01:10:02] 22 ...
[01:10:02] 23 LL | }
[01:10:02] 23 LL | }
[01:10:02] 24    | - here, drop of `s` needs exclusive access to `*s.0`, because the type `Scribble<'_>` implements the `Drop` trait
[01:10:02] -    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:10:02] -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:10:02] 28 
[01:10:02] - warning[E0713]: borrow may still be in use when destructor runs
[01:10:02] - warning[E0713]: borrow may still be in use when destructor runs
[01:10:02] + error[E0713]: borrow may still be in use when destructor runs
[01:10:02] 30   --> $DIR/issue-45696-scribble-on-boxed-borrow.rs:73:5
[01:10:02] 31    |
[01:10:02] 32 LL | fn boxed_boxed_scribbled<'a>(s: Box<Box<Scribble<'a>>>) -> &'a mut u32 {
[01:10:02] 36 ...
[01:10:02] 37 LL | }
[01:10:02] 37 LL | }
[01:10:02] 38    | - here, drop of `s` needs exclusive access to `*s.0`, because the type `Scribble<'_>` implements the `Drop` trait
[01:10:02] -    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:10:02] -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:10:02] 42 
[01:10:02] - error: compilation successful
[01:10:02] - error: compilation successful
[01:10:02] -   --> $DIR/issue-45696-scribble-on-boxed-borrow.rs:80:1
[01:10:02] -    |
[01:10:02] - LL | / fn main() {
[01:10:02] - LL | |     let mut x = 1;
[01:10:02] - LL | |     {
[01:10:02] - LL | |         let mut long_lived = Scribble(&mut x);
[01:10:02] - ...  |
[01:10:02] - LL | |     *boxed_boxed_scribbled(Box::new(Box::new(Scribble(&mut x)))) += 10;
[01:10:02] - LL | | }
[01:10:02] - 
[01:10:02] - error: aborting due to previous error
[01:10:02] + error: aborting due to 3 previous errors
[01:10:02] 56 
[01:10:02] 56 
[01:10:02] 57 For more information about this error, try `rustc --explain E0713`.
[01:10:02] 58 
[01:10:02] 
[01:10:02] 
[01:10:02] The actual stderr differed from the expected stderr.
[01:10:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45696-scribble-on-boxed-borrow.migrate/issue-45696-scribble-on-boxed-borrow.migrate.stderr
[01:10:02] To update references, rerun the tests and pass the `--bless` flag
[01:10:02] To only update this specific test, also pass `--test-args issues/issue-45696-scribble-on-boxed-borrow.rs`
[01:10:02] 
[01:10:02] error in revision `migrate`: 1 errors occurred comparing output.
[01:10:02] status: exit code: 1
[01:10:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-45696-scribble-on-boxed-borrow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "migrate" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45696-scribble-on-boxed-borrow.migrate" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45696-scribble-on-boxed-borrow.migrate/auxiliary" "-A" "unused"
[01:10:02] ------------------------------------------
[01:10:02] 
[01:10:02] ------------------------------------------
[01:10:02] stderr:
[01:10:02] stderr:
[01:10:02] ------------------------------------------
[01:10:02] error[E0713]: borrow may still be in use when destructor runs
[01:10:02]   --> /checkout/src/test/ui/issues/issue-45696-scribble-on-boxed-borrow.rs:51:5
[01:10:02]    |
[01:10:02] LL | fn scribbled<'a>(s: Scribble<'a>) -> &'a mut u32 {
[01:10:02]    |              -- lifetime `'a` defined here
[01:10:02] LL |     &mut *s.0 //[nll]~ ERROR borrow may still be in use when destructor runs [E0713]
[01:10:02]    |     ^^^^^^^^^ returning this value requires that `*s.0` is borrowed for `'a`
[01:10:02] LL | }
[01:10:02] LL | }
[01:10:02]    | - here, drop of `s` needs exclusive access to `*s.0`, because the type `Scribble<'_>` implements the `Drop` trait
[01:10:02] error[E0713]: borrow may still be in use when destructor runs
[01:10:02]   --> /checkout/src/test/ui/issues/issue-45696-scribble-on-boxed-borrow.rs:62:5
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL | fn boxed_scribbled<'a>(s: Box<Scribble<'a>>) -> &'a mut u32 {
[01:10:02]    |                    -- lifetime `'a` defined here
[01:10:02] LL |     &mut *(*s).0 //[nll]~ ERROR borrow may still be in use when destructor runs [E0713]
[01:10:02]    |     ^^^^^^^^^^^^ returning this value requires that `*s.0` is borrowed for `'a`
[01:10:02] LL | }
[01:10:02] LL | }
[01:10:02]    | - here, drop of `s` needs exclusive access to `*s.0`, because the type `Scribble<'_>` implements the `Drop` trait
[01:10:02] error[E0713]: borrow may still be in use when destructor runs
[01:10:02]   --> /checkout/src/test/ui/issues/issue-45696-scribble-on-boxed-borrow.rs:73:5
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL | fn boxed_boxed_scribbled<'a>(s: Box<Box<Scribble<'a>>>) -> &'a mut u32 {
[01:10:02]    |                          -- lifetime `'a` defined here
[01:10:02] LL |     &mut *(**s).0 //[nll]~ ERROR borrow may still be in use when destructor runs [E0713]
[01:10:02]    |     ^^^^^^^^^^^^^ returning this value requires that `*s.0` is borrowed for `'a`
[01:10:02] LL | }
[01:10:02] LL | }
[01:10:02]    | - here, drop of `s` needs exclusive access to `*s.0`, because the type `Scribble<'_>` implements the `Drop` trait
[01:10:02] error: aborting due to 3 previous errors
[01:10:02] 
[01:10:02] For more information about this error, try `rustc --explain E0713`.
[01:10:02] 
[01:10:02] 
[01:10:02] ------------------------------------------
[01:10:02] 
[01:10:02] 
[01:10:02] ---- [ui] ui/issues/issue-49824.rs stdout ----
[01:10:02] diff of stderr:
[01:10:02] 
[01:10:02] - warning: captured variable cannot escape `FnMut` closure body
[01:10:02] + error: captured variable cannot escape `FnMut` closure body
[01:10:02] 3    |
[01:10:02] 4 LL |       || {
[01:10:02] 
[01:10:02] 13    |
[01:10:02] 13    |
[01:10:02] 14    = note: `FnMut` closures only have access to their captured variables while they are executing...
[01:10:02] 15    = note: ...therefore, they cannot allow references to captured variables to escape
[01:10:02] -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:10:02] - 
[01:10:02] - error: compilation successful
[01:10:02] -   --> $DIR/issue-49824.rs:6:1
---
[01:10:02] 
[01:10:02] 
[01:10:02] The actual stderr differed from the expected stderr.
[01:10:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-49824/issue-49824.stderr
[01:10:02] To update references, rerun the tests and pass the `--bless` flag
[01:10:02] To only update this specific test, also pass `--test-args issues/issue-49824.rs`
[01:10:02] error: 1 errors occurred comparing output.
[01:10:02] status: exit code: 1
[01:10:02] status: exit code: 1
[01:10:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-49824.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-49824" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-49824/auxiliary" "-A" "unused"
[01:10:02] ------------------------------------------
[01:10:02] 
[01:10:02] ------------------------------------------
[01:10:02] stderr:
[01:10:02] stderr:
[01:10:02] ------------------------------------------
[01:10:02] error: captured variable cannot escape `FnMut` closure body
[01:10:02]    |
[01:10:02] LL |       || {
[01:10:02] LL |       || {
[01:10:02]    |        - inferred to be a `FnMut` closure
[01:10:02] LL | /         || {
[01:10:02] LL | |         //~^ WARNING captured variable cannot escape `FnMut` closure body
[01:10:02] LL | |         //~| WARNING this error has been downgraded to a warning
[01:10:02] LL | |         //~| WARNING this warning will become a hard error in the future
[01:10:02] LL | |             let _y = &mut x;
[01:10:02]    | |_________^ returns a closure that contains a reference to a captured variable, which then escapes the closure body
[01:10:02]    |
[01:10:02]    |
[01:10:02]    = note: `FnMut` closures only have access to their captured variables while they are executing...
[01:10:02]    = note: ...therefore, they cannot allow references to captured variables to escape
[01:10:02] error: aborting due to previous error
[01:10:02] 
[01:10:02] 
[01:10:02] ------------------------------------------
[01:10:02] ------------------------------------------
[01:10:02] 
[01:10:02] 
[01:10:02] ---- [ui] ui/pattern/pattern-bindings-after-at.rs stdout ----
[01:10:02] diff of stderr:
[01:10:02] 
[01:10:02] 4 LL |         ref mut z @ &mut Some(ref a) => {
[01:10:02] 5    |                               ^^^^^ not allowed after `@`
[01:10:02] 6 
[01:10:02] - warning[E0502]: cannot borrow `_` as immutable because it is also borrowed as mutable
[01:10:02] + error[E0502]: cannot borrow `_` as immutable because it is also borrowed as mutable
[01:10:02] 9    |
[01:10:02] 9    |
[01:10:02] 10 LL |         ref mut z @ &mut Some(ref a) => {
[01:10:02] 15 ...
[01:10:02] 15 ...
[01:10:02] 16 LL |             **z = None;
[01:10:02] 17    |             ---------- mutable borrow later used here
[01:10:02] -    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:10:02] -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:10:02] 21 
[01:10:02] - error: aborting due to previous error
[01:10:02] - error: aborting due to previous error
[01:10:02] + error: aborting due to 2 previous errors
[01:10:02] 23 
[01:10:02] 24 Some errors have detailed explanations: E0303, E0502.
[01:10:02] 25 For more information about an error, try `rustc --explain E0303`.
[01:10:02] 
[01:10:02] 
[01:10:02] The actual stderr differed from the expected stderr.
[01:10:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pattern-bindings-after-at/pattern-bindings-after-at.stderr
[01:10:02] To update references, rerun the tests and pass the `--bless` flag
[01:10:02] To only update this specific test, also pass `--test-args pattern/pattern-bindings-after-at.rs`
[01:10:02] error: 1 errors occurred comparing output.
[01:10:02] status: exit code: 1
[01:10:02] status: exit code: 1
[01:10:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/pattern-bindings-after-at.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pattern-bindings-after-at" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pattern-bindings-after-at/auxiliary" "-A" "unused"
[01:10:02] ------------------------------------------
[01:10:02] 
[01:10:02] ------------------------------------------
[01:10:02] stderr:
[01:10:02] stderr:
[01:10:02] ------------------------------------------
[01:10:02] error[E0303]: pattern bindings are not allowed after an `@`
[01:10:02]   --> /checkout/src/test/ui/pattern/pattern-bindings-after-at.rs:8:31
[01:10:02]    |
[01:10:02] LL |         ref mut z @ &mut Some(ref a) => {
[01:10:02]    |                               ^^^^^ not allowed after `@`
[01:10:02] error[E0502]: cannot borrow `_` as immutable because it is also borrowed as mutable
[01:10:02]   --> /checkout/src/test/ui/pattern/pattern-bindings-after-at.rs:8:31
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL |         ref mut z @ &mut Some(ref a) => {
[01:10:02]    |         |                     |
[01:10:02]    |         |                     immutable borrow occurs here
[01:10:02]    |         mutable borrow occurs here
[01:10:02] ...
[01:10:02] ...
[01:10:02] LL |             **z = None;
[01:10:02]    |             ---------- mutable borrow later used here
[01:10:02] error: aborting due to 2 previous errors
[01:10:02] 
[01:10:02] Some errors have detailed explanations: E0303, E0502.
[01:10:02] For more information about an error, try `rustc --explain E0303`.
[01:10:02] For more information about an error, try `rustc --explain E0303`.
[01:10:02] 
[01:10:02] ------------------------------------------
[01:10:02] 
[01:10:02] 
[01:10:02] ---- [ui] ui/recursion/recursive-types-are-not-uninhabited.rs stdout ----
[01:10:02] diff of stderr:
[01:10:02] 
[01:10:02] 4 LL |     let Ok(x) = res;
[01:10:02] 6 
[01:10:02] 6 
[01:10:02] - warning[E0381]: use of possibly uninitialized variable: `x`
[01:10:02] + error[E0381]: use of possibly uninitialized variable: `x`
[01:10:02] 9    |
[01:10:02] 10 LL |     x
[01:10:02] 
[01:10:02] 
[01:10:02] 11    |     ^ use of possibly uninitialized `x`
[01:10:02] -    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:10:02] -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:10:02] 15 
[01:10:02] - error: aborting due to previous error
---
[01:10:02] 
[01:10:02] 
[01:10:02] The actual stderr differed from the expected stderr.
[01:10:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/recursive-types-are-not-uninhabited/recursive-types-are-not-uninhabited.stderr
[01:10:02] To update references, rerun the tests and pass the `--bless` flag
[01:10:02] To only update this specific test, also pass `--test-args recursion/recursive-types-are-not-uninhabited.rs`
[01:10:02] error: 1 errors occurred comparing output.
[01:10:02] status: exit code: 1
[01:10:02] status: exit code: 1
[01:10:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/recursion/recursive-types-are-not-uninhabited.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/recursive-types-are-not-uninhabited" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/recursive-types-are-not-uninhabited/auxiliary" "-A" "unused"
[01:10:02] ------------------------------------------
[01:10:02] 
[01:10:02] ------------------------------------------
[01:10:02] stderr:
[01:10:02] stderr:
[01:10:02] ------------------------------------------
[01:10:02] error[E0005]: refutable pattern in local binding: `Err(_)` not covered
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL |     let Ok(x) = res;
[01:10:02] 
[01:10:02] 
[01:10:02] error[E0381]: use of possibly uninitialized variable: `x`
[01:10:02]    |
[01:10:02] LL |     x
[01:10:02] LL |     x
[01:10:02]    |     ^ use of possibly uninitialized `x`
[01:10:02] error: aborting due to 2 previous errors
[01:10:02] 
[01:10:02] Some errors have detailed explanations: E0005, E0381.
[01:10:02] For more information about an error, try `rustc --explain E0005`.
[01:10:02] For more information about an error, try `rustc --explain E0005`.
[01:10:02] 
[01:10:02] ------------------------------------------
[01:10:02] 
[01:10:02] 
[01:10:02] ---- [ui] ui/thread-local-in-ctfe.rs stdout ----
[01:10:02] diff of stderr:
[01:10:02] 
[01:10:02] 10 LL | static C: &u32 = &A;
[01:10:02] 12 
[01:10:02] - warning[E0712]: thread-local variable borrowed past end of function
[01:10:02] + error[E0712]: thread-local variable borrowed past end of function
[01:10:02] 14   --> $DIR/thread-local-in-ctfe.rs:9:18
[01:10:02] 14   --> $DIR/thread-local-in-ctfe.rs:9:18
[01:10:02] 15    |
[01:10:02] 16 LL | static C: &u32 = &A;
[01:10:02] 
[01:10:02] 17    |                  ^^- end of enclosing function is here
[01:10:02] 19    |                  thread-local variables cannot be borrowed beyond the end of the function
[01:10:02] -    |
[01:10:02] -    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:10:02] -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:10:02] -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:10:02] 23 
[01:10:02] 24 error[E0625]: thread-local statics cannot be accessed at compile-time
[01:10:02] 25   --> $DIR/thread-local-in-ctfe.rs:15:16
[01:10:02] 
[01:10:02] 33 LL | const E: &u32 = &A;
[01:10:02] 35 
[01:10:02] - warning[E0712]: thread-local variable borrowed past end of function
[01:10:02] + error[E0712]: thread-local variable borrowed past end of function
[01:10:02] 37   --> $DIR/thread-local-in-ctfe.rs:18:17
[01:10:02] 37   --> $DIR/thread-local-in-ctfe.rs:18:17
[01:10:02] 38    |
[01:10:02] 39 LL | const E: &u32 = &A;
[01:10:02] 
[01:10:02] 40    |                 ^^- end of enclosing function is here
[01:10:02] 42    |                 thread-local variables cannot be borrowed beyond the end of the function
[01:10:02] -    |
[01:10:02] -    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:10:02] -    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
---
[01:10:02] 56 
[01:10:02] 
[01:10:02] 
[01:10:02] The actual stderr differed from the expected stderr.
[01:10:02] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thread-local-in-ctfe/thread-local-in-ctfe.stderr
[01:10:02] To update references, rerun the tests and pass the `--bless` flag
[01:10:02] To only update this specific test, also pass `--test-args thread-local-in-ctfe.rs`
[01:10:02] error: 1 errors occurred comparing output.
[01:10:02] status: exit code: 1
[01:10:02] status: exit code: 1
[01:10:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/thread-local-in-ctfe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thread-local-in-ctfe" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thread-local-in-ctfe/auxiliary" "-A" "unused"
[01:10:02] ------------------------------------------
[01:10:02] 
[01:10:02] ------------------------------------------
[01:10:02] stderr:
[01:10:02] stderr:
[01:10:02] ------------------------------------------
[01:10:02] error[E0625]: thread-local statics cannot be accessed at compile-time
[01:10:02]   --> /checkout/src/test/ui/thread-local-in-ctfe.rs:6:17
[01:10:02]    |
[01:10:02] LL | static B: u32 = A;
[01:10:02] 
[01:10:02] error[E0625]: thread-local statics cannot be accessed at compile-time
[01:10:02]   --> /checkout/src/test/ui/thread-local-in-ctfe.rs:9:18
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL | static C: &u32 = &A;
[01:10:02] 
[01:10:02] error[E0712]: thread-local variable borrowed past end of function
[01:10:02]   --> /checkout/src/test/ui/thread-local-in-ctfe.rs:9:18
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL | static C: &u32 = &A;
[01:10:02]    |                  ^^- end of enclosing function is here
[01:10:02]    |                  thread-local variables cannot be borrowed beyond the end of the function
[01:10:02] 
[01:10:02] error[E0625]: thread-local statics cannot be accessed at compile-time
[01:10:02]   --> /checkout/src/test/ui/thread-local-in-ctfe.rs:15:16
[01:10:02]   --> /checkout/src/test/ui/thread-local-in-ctfe.rs:15:16
[01:10:02]    |
[01:10:02] LL | const D: u32 = A;
[01:10:02] 
[01:10:02] error[E0625]: thread-local statics cannot be accessed at compile-time
[01:10:02]   --> /checkout/src/test/ui/thread-local-in-ctfe.rs:18:17
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL | const E: &u32 = &A;
[01:10:02]    |                 ^^
[01:10:02] make: *** [check] Error 1
[01:10:02] error[E0712]: thread-local variable borrowed past end of function
[01:10:02]   --> /checkout/src/test/ui/thread-local-in-ctfe.rs:18:17
[01:10:02]    |
[01:10:02]    |
[01:10:02] LL | const E: &u32 = &A;
[01:10:02]    |                 ^^- end of enclosing function is here
[01:10:02]    |                 thread-local variables cannot be borrowed beyond the end of the function
[01:10:02] 
[01:10:02] error[E0625]: thread-local statics cannot be accessed at compile-time
[01:10:02]   --> /checkout/src/test/ui/thread-local-in-ctfe.rs:25:5
---
[01:10:02] test result: FAILED. 5494 passed; 18 failed; 21 ignored; 0 measured; 0 filtered out
[01:10:02] 
[01:10:02] 
[01:10:02] 
[01:10:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:10:02] 
[01:10:02] 
[01:10:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:02] Build completed unsuccessfully in 0:04:21
---
travis_time:end:03d0e1d6:start=1558100133650296009,finish=1558100133654614224,duration=4318215
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:021b20cc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1b7db430
travis_time:start:1b7db430
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2da034cb
$ dmesg | grep -i kill

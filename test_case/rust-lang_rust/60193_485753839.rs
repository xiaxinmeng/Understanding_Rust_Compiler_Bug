plain
travis_time:end:1067dd26:start=1556012498048409945,finish=1556012636488580474,duration=138440170529
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:43] 
[01:04:43] running 5459 tests
[01:04:46] ......F............................................................................................. 100/5459
[01:04:50] ...............................................................................................F.... 200/5459
[01:04:53] .......F.F.........F.FF...........F..........................F...................................... 300/5459
[01:04:56] .........F.............................................F..F...........F.FF..........FFF............. 400/5459
[01:05:00] ........................F................................................F................i......... 500/5459
[01:05:07] .................................................................................................... 700/5459
[01:05:12] .................................................................................................... 800/5459
[01:05:18] .....................................................i...............i.............................. 900/5459
[01:05:18] .....................................................i...............i.............................. 900/5459
[01:05:21] .........................................................F..........F.................iiiii......... 1000/5459
[01:05:27] .................................................................................................... 1200/5459
[01:05:27] .................................................................................................... 1200/5459
[01:05:30] ...................................F................................................................ 1300/5459
[01:05:36] .................................................................................................... 1500/5459
[01:05:36] .................................................................................................... 1500/5459
[01:05:38] ......................................................................................F............i 1600/5459
[01:05:42] .................................................................................................... 1700/5459
[01:05:46] ..........................F......................................................................... 1800/5459
[01:05:53] .................................................................................................... 2000/5459
[01:05:56] ...................................i................................................................ 2100/5459
[01:06:00] .................................................................................................... 2200/5459
[01:06:04] .................................................................................................... 2300/5459
[01:06:04] .................................................................................................... 2300/5459
[01:06:08] .................................................................................................... 2400/5459
[01:06:13] .................................................................................................... 2500/5459
[01:06:17] .................................................................................................... 2600/5459
[01:06:21] .................................................................................................... 2700/5459
[01:06:25] .................................................F...............F.................................. 2800/5459
[01:06:30] ...............................................F.F.................................................. 2900/5459
[01:06:36] .................................................................................................... 3100/5459
[01:06:41] .................................................................................................... 3200/5459
[01:06:45] .............................................................i...................................... 3300/5459
[01:06:48] .................................................................................................... 3400/5459
[01:06:48] .................................................................................................... 3400/5459
[01:06:52] ...................................ii...i..ii................................FF..................... 3500/5459
[01:06:56] ....F........................F.......F...............F...........F.................................. 3600/5459
[01:06:59] F................................................................................................... 3700/5459
[01:07:05] ............................................................i....................................... 3900/5459
[01:07:07] .................................................................................................... 4000/5459
[01:07:09] ....................i............................................................................... 4100/5459
[01:07:13] .................................................................................................... 4200/5459
[01:07:13] .................................................................................................... 4200/5459
[01:07:24] .................................................................................................... 4300/5459
[01:07:27] .................................................................................................... 4400/5459
[01:07:31] .................................................................................................... 4500/5459
[01:07:35] .........................................................................FF......................... 4600/5459
[01:07:45] .................................................................................................... 4800/5459
[01:07:48] .................................................................................................... 4900/5459
[01:07:48] .................................................................................................... 4900/5459
[01:07:52] ................................................................F................................... 5000/5459
[01:07:56] .........F.......................................................................................... 5100/5459
[01:08:00] .......................................................................................F............ 5200/5459
[01:08:03] .............FF..................................................................................... 5300/5459
[01:08:05] .................................................................................................i.. 5400/5459
[01:08:07] failures:
[01:08:07] 
[01:08:07] ---- [ui] ui/E0594.rs stdout ----
[01:08:07] diff of stderr:
---
[01:08:07] 9 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/E0594/E0594.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args E0594.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/E0594.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/E0594/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/E0594/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to immutable static item `NUM`
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     NUM = 20; //~ ERROR cannot assign to immutable static item `NUM`
[01:08:07] 
[01:08:07] error: aborting due to previous error
[01:08:07] 
[01:08:07] For more information about this error, try `rustc --explain E0594`.
---
[01:08:07] 
[01:08:07] 72 
[01:08:07] 73 error: aborting due to 6 previous errors
[01:08:07] 74 
[01:08:07] - For more information about this error, try `rustc --explain E0596`.
[01:08:07] + Some errors have detailed explanations: E0594, E0596.
[01:08:07] 76 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrow-immutable-upvar-mutation/borrow-immutable-upvar-mutation.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args borrowck/borrow-immutable-upvar-mutation.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrow-immutable-upvar-mutation.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrow-immutable-upvar-mutation/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrow-immutable-upvar-mutation/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `x`, as it is a captured variable in a `Fn` closure
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         let _f = to_fn(|| x = 42); //~ ERROR cannot assign
[01:08:07]    |
[01:08:07]    |
[01:08:07] help: consider changing this to accept closures that implement `FnMut`
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         let _f = to_fn(|| x = 42); //~ ERROR cannot assign
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `y` as mutable, as it is a captured variable in a `Fn` closure
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         let _g = to_fn(|| set(&mut y)); //~ ERROR cannot borrow
[01:08:07]    |                               ^^^^^^ cannot borrow as mutable
[01:08:07]    |
[01:08:07] help: consider changing this to accept closures that implement `FnMut`
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         let _g = to_fn(|| set(&mut y)); //~ ERROR cannot borrow
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `z`, as it is a captured variable in a `Fn` closure
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         let _h = to_fn_mut(|| { set(&mut z); to_fn(|| z = 42); }); //~ ERROR cannot assign
[01:08:07]    |
[01:08:07]    |
[01:08:07] help: consider changing this to accept closures that implement `FnMut`
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         let _h = to_fn_mut(|| { set(&mut z); to_fn(|| z = 42); }); //~ ERROR cannot assign
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `x`, as it is a captured variable in a `Fn` closure
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         let _f = to_fn(move || x = 42); //~ ERROR cannot assign
[01:08:07]    |
[01:08:07]    |
[01:08:07] help: consider changing this to accept closures that implement `FnMut`
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         let _f = to_fn(move || x = 42); //~ ERROR cannot assign
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `y` as mutable, as it is a captured variable in a `Fn` closure
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         let _g = to_fn(move || set(&mut y)); //~ ERROR cannot borrow
[01:08:07]    |                                    ^^^^^^ cannot borrow as mutable
[01:08:07]    |
[01:08:07] help: consider changing this to accept closures that implement `FnMut`
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         let _g = to_fn(move || set(&mut y)); //~ ERROR cannot borrow
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `z`, as it is a captured variable in a `Fn` closure
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         let _h = to_fn_mut(move || { set(&mut z); to_fn(move || z = 42); }); //~ ERROR cannot assign
[01:08:07]    |
[01:08:07]    |
[01:08:07] help: consider changing this to accept closures that implement `FnMut`
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         let _h = to_fn_mut(move || { set(&mut z); to_fn(move || z = 42); }); //~ ERROR cannot assign
[01:08:07] 
[01:08:07] error: aborting due to 6 previous errors
[01:08:07] 
[01:08:07] Some errors have detailed explanations: E0594, E0596.
---
[01:08:07] 19 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-assign-to-andmut-in-aliasable-loc/borrowck-assign-to-andmut-in-aliasable-loc.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args borrowck/borrowck-assign-to-andmut-in-aliasable-loc.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-assign-to-andmut-in-aliasable-loc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-assign-to-andmut-in-aliasable-loc/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-assign-to-andmut-in-aliasable-loc/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `*s.pointer` which is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL | fn a(s: &S) {
[01:08:07]    |         -- help: consider changing this to be a mutable reference: `&mut S<'_>`
[01:08:07] LL |     *s.pointer += 1; //~ ERROR cannot assign
[01:08:07]    |     ^^^^^^^^^^^^^^^ `s` is a `&` reference, so the data it refers to cannot be written
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `*s.pointer` which is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL | fn c(s: & &mut S) {
[01:08:07]    |         -------- help: consider changing this to be a mutable reference: `&mut &mut S<'_>`
[01:08:07] LL |     *s.pointer += 1; //~ ERROR cannot assign
[01:08:07]    |     ^^^^^^^^^^^^^^^ `s` is a `&` reference, so the data it refers to cannot be written
[01:08:07] error: aborting due to 2 previous errors
[01:08:07] 
[01:08:07] For more information about this error, try `rustc --explain E0594`.
[01:08:07] 
---
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-assign-to-constants/borrowck-assign-to-constants.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args borrowck/borrowck-assign-to-constants.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-assign-to-constants.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-assign-to-constants/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-assign-to-constants/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
---
[01:08:07] 
[01:08:07] 27 
[01:08:07] 28 error: aborting due to 3 previous errors
[01:08:07] 29 
[01:08:07] - Some errors have detailed explanations: E0502, E0596.
[01:08:07] + Some errors have detailed explanations: E0502, E0594, E0596.
[01:08:07] 32 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-borrow-mut-base-ptr-in-aliasable-loc/borrowck-borrow-mut-base-ptr-in-aliasable-loc.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args borrowck/borrowck-borrow-mut-base-ptr-in-aliasable-loc.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-borrow-mut-base-ptr-in-aliasable-loc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-borrow-mut-base-ptr-in-aliasable-loc/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-borrow-mut-base-ptr-in-aliasable-loc/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `**t1` which is behind a `&` reference
[01:08:07]    |
[01:08:07] LL |     let t1 = t0;
[01:08:07]    |         -- help: consider changing this to be a mutable reference: `&mut &mut isize`
[01:08:07]    |         -- help: consider changing this to be a mutable reference: `&mut &mut isize`
[01:08:07] LL |     let p: &isize = &**t0;
[01:08:07] LL |     **t1 = 22; //~ ERROR cannot assign
[01:08:07]    |     ^^^^^^^^^ `t1` is a `&` reference, so the data it refers to cannot be written
[01:08:07] 
[01:08:07] error[E0502]: cannot borrow `**t0` as immutable because it is also borrowed as mutable
[01:08:07]    |
[01:08:07] LL |     let t1 = &mut *t0;
[01:08:07]    |              -------- mutable borrow occurs here
[01:08:07]    |              -------- mutable borrow occurs here
[01:08:07] LL |     let p: &isize = &**t0; //~ ERROR cannot borrow
[01:08:07] LL |     **t1 = 22;
[01:08:07]    |     --------- mutable borrow later used here
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `**t0` as mutable, as it is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL | fn foo4(t0: & &mut isize) {
[01:08:07]    |             ------------ help: consider changing this to be a mutable reference: `&mut &mut isize`
[01:08:07] LL |     let x:  &mut isize = &mut **t0; //~ ERROR cannot borrow
[01:08:07]    |                          ^^^^^^^^^ `t0` is a `&` reference, so the data it refers to cannot be borrowed as mutable
[01:08:07] error: aborting due to 3 previous errors
[01:08:07] 
[01:08:07] Some errors have detailed explanations: E0502, E0594, E0596.
[01:08:07] For more information about an error, try `rustc --explain E0502`.
---
[01:08:07] 
[01:08:07] 42 
[01:08:07] 43 error: aborting due to 7 previous errors
[01:08:07] 44 
[01:08:07] - For more information about this error, try `rustc --explain E0596`.
[01:08:07] + Some errors have detailed explanations: E0594, E0596.
[01:08:07] 46 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-borrow-overloaded-deref/borrowck-borrow-overloaded-deref.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args borrowck/borrowck-borrow-overloaded-deref.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-borrow-overloaded-deref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-borrow-overloaded-deref/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-borrow-overloaded-deref/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0596]: cannot borrow data in a `&` reference as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     let __isize = &mut *x; //~ ERROR cannot borrow
[01:08:07]    |                   ^^^^^^^ cannot borrow as mutable
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow data in a `&` reference as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     let __isize = &mut *x; //~ ERROR cannot borrow
[01:08:07]    |                   ^^^^^^^ cannot borrow as mutable
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow data in a `&` reference as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     &mut **x //~ ERROR cannot borrow
[01:08:07]    |     ^^^^^^^^ cannot borrow as mutable
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow data in a `&` reference as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     &mut **x //~ ERROR cannot borrow
[01:08:07]    |     ^^^^^^^^ cannot borrow as mutable
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to data in a `&` reference
[01:08:07]    |
[01:08:07] LL |     *x = 3; //~ ERROR cannot assign
[01:08:07]    |     ^^^^^^ cannot assign
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to data in a `&` reference
[01:08:07]    |
[01:08:07] LL |     **x = 3; //~ ERROR cannot assign
[01:08:07]    |     ^^^^^^^ cannot assign
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to data in a `&` reference
[01:08:07]    |
[01:08:07] LL |     **x = 3; //~ ERROR cannot assign
[01:08:07]    |     ^^^^^^^ cannot assign
[01:08:07] 
---
[01:08:07] 
[01:08:07] 84 
[01:08:07] 85 error: aborting due to 14 previous errors
[01:08:07] 86 
[01:08:07] - For more information about this error, try `rustc --explain E0596`.
[01:08:07] + Some errors have detailed explanations: E0594, E0596.
[01:08:07] 88 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-borrow-overloaded-auto-deref/borrowck-borrow-overloaded-auto-deref.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args borrowck/borrowck-borrow-overloaded-auto-deref.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-borrow-overloaded-auto-deref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-borrow-overloaded-auto-deref/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-borrow-overloaded-auto-deref/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0596]: cannot borrow data in a `&` reference as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     let __isize = &mut x.y; //~ ERROR cannot borrow
[01:08:07]    |                   ^^^^^^^^ cannot borrow as mutable
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow data in a `&` reference as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     let __isize = &mut x.y; //~ ERROR cannot borrow
[01:08:07]    |                   ^^^^^^^^ cannot borrow as mutable
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow data in a `&` reference as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     &mut x.y //~ ERROR cannot borrow
[01:08:07]    |     ^^^^^^^^ cannot borrow as mutable
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow data in a `&` reference as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     &mut x.y //~ ERROR cannot borrow
[01:08:07]    |     ^^^^^^^^ cannot borrow as mutable
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to data in a `&` reference
[01:08:07]    |
[01:08:07] LL |     x.y = 3; //~ ERROR cannot assign
[01:08:07]    |     ^^^^^^^ cannot assign
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to data in a `&` reference
[01:08:07]    |
[01:08:07] LL |     x.y = 3; //~ ERROR cannot assign
[01:08:07]    |     ^^^^^^^ cannot assign
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to data in a `&` reference
[01:08:07]    |
[01:08:07] LL |     x.y = 3; //~ ERROR cannot assign
[01:08:07]    |     ^^^^^^^ cannot assign
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow data in a `&` reference as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     x.set(0, 0); //~ ERROR cannot borrow
[01:08:07]    |     ^ cannot borrow as mutable
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow data in a `&` reference as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     x.set(0, 0); //~ ERROR cannot borrow
[01:08:07]    |     ^ cannot borrow as mutable
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow data in a `&` reference as mutable
[01:08:07]    |
[01:08:07] LL |     x.y_mut() //~ ERROR cannot borrow
[01:08:07]    |     ^ cannot borrow as mutable
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow data in a `&` reference as mutable
[01:08:07]    |
[01:08:07] LL |     x.y_mut() //~ ERROR cannot borrow
[01:08:07]    |     ^ cannot borrow as mutable
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow data in a `&` reference as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     *x.y_mut() = 3; //~ ERROR cannot borrow
[01:08:07]    |      ^ cannot borrow as mutable
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow data in a `&` reference as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     *x.y_mut() = 3; //~ ERROR cannot borrow
[01:08:07]    |      ^ cannot borrow as mutable
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow data in a `&` reference as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     *x.y_mut() = 3; //~ ERROR cannot borrow
[01:08:07]    |      ^ cannot borrow as mutable
[01:08:07] error: aborting due to 14 previous errors
[01:08:07] 
[01:08:07] Some errors have detailed explanations: E0594, E0596.
[01:08:07] For more information about an error, try `rustc --explain E0594`.
---
[01:08:07] 
[01:08:07] 50 
[01:08:07] 51 error: aborting due to 4 previous errors
[01:08:07] 52 
[01:08:07] - For more information about this error, try `rustc --explain E0500`.
[01:08:07] + Some errors have detailed explanations: E0500, E0594.
[01:08:07] 54 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-unique/borrowck-closures-unique.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args borrowck/borrowck-closures-unique.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-closures-unique.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-unique/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-unique/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0500]: closure requires unique access to `x` but it is already borrowed
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     let c1 = || get(x);
[01:08:07]    |              --     - first borrow occurs due to use of `x` in closure
[01:08:07]    |              borrow occurs here
[01:08:07]    |              borrow occurs here
[01:08:07] LL |     let c2 = || set(x); //~ ERROR closure requires unique access to `x`
[01:08:07]    |              ^^     - second borrow occurs due to use of `x` in closure
[01:08:07]    |              closure construction occurs here
[01:08:07] LL |     c1;
[01:08:07]    |     -- first borrow later used here
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0500]: closure requires unique access to `x` but it is already borrowed
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     let c1 = || get(x);
[01:08:07]    |              --     - first borrow occurs due to use of `x` in closure
[01:08:07]    |              borrow occurs here
[01:08:07]    |              borrow occurs here
[01:08:07] LL |     let c2 = || { get(x); set(x); }; //~ ERROR closure requires unique access to `x`
[01:08:07]    |              ^^       - second borrow occurs due to use of `x` in closure
[01:08:07]    |              closure construction occurs here
[01:08:07] LL |     c1;
[01:08:07]    |     -- first borrow later used here
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0524]: two closures require unique access to `x` at the same time
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     let c1 = || set(x);
[01:08:07]    |              --     - first borrow occurs due to use of `x` in closure
[01:08:07]    |              first closure is constructed here
[01:08:07]    |              first closure is constructed here
[01:08:07] LL |     let c2 = || set(x); //~ ERROR two closures require unique access to `x` at the same time
[01:08:07]    |              ^^     - second borrow occurs due to use of `x` in closure
[01:08:07]    |              second closure is constructed here
[01:08:07] LL |     c1;
[01:08:07]    |     -- first borrow later used here
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `x`, as it is not declared as mutable
[01:08:07]    |
[01:08:07] LL | fn e(x: &'static mut isize) {
[01:08:07]    |      - help: consider changing this to be mutable: `mut x`
[01:08:07]    |      - help: consider changing this to be mutable: `mut x`
[01:08:07] LL |     let c1 = |y: &'static mut isize| x = y;
[01:08:07] 
[01:08:07] error: aborting due to 4 previous errors
[01:08:07] 
[01:08:07] Some errors have detailed explanations: E0500, E0594.
---
[01:08:07] 
[01:08:07] 96 
[01:08:07] 97 error: aborting due to 9 previous errors
[01:08:07] 98 
[01:08:07] - For more information about this error, try `rustc --explain E0506`.
[01:08:07] + Some errors have detailed explanations: E0506, E0594.
[01:08:07] 100 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-issue-14498/borrowck-issue-14498.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args borrowck/borrowck-issue-14498.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-issue-14498.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-issue-14498/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-issue-14498/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `***p` which is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     let p = &y;
[01:08:07]    |             -- help: consider changing this to be a mutable reference: `&mut y`
[01:08:07] LL |     ***p = 2; //~ ERROR cannot assign to `***p`
[01:08:07]    |     ^^^^^^^^ `p` is a `&` reference, so the data it refers to cannot be written
[01:08:07] 
[01:08:07] error[E0506]: cannot assign to `**y` because it is borrowed
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     let p = &y;
[01:08:07]    |             -- borrow of `**y` occurs here
[01:08:07] LL |     let q = &***p;
[01:08:07] LL |     **y = 2; //~ ERROR cannot assign to `**y` because it is borrowed
[01:08:07]    |     ^^^^^^^ assignment to borrowed `**y` occurs here
[01:08:07] LL |     drop(p);
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0506]: cannot assign to `**y` because it is borrowed
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     let p = &y;
[01:08:07]    |             -- borrow of `**y` occurs here
[01:08:07] LL |     let q = &***p;
[01:08:07] LL |     **y = 2; //~ ERROR cannot assign to `**y` because it is borrowed
[01:08:07]    |     ^^^^^^^ assignment to borrowed `**y` occurs here
[01:08:07] LL |     drop(p);
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0506]: cannot assign to `**y` because it is borrowed
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     let p = &y;
[01:08:07]    |             -- borrow of `**y` occurs here
[01:08:07] LL |     let q = &***p;
[01:08:07] LL |     **y = 2; //~ ERROR cannot assign to `**y` because it is borrowed
[01:08:07]    |     ^^^^^^^ assignment to borrowed `**y` occurs here
[01:08:07] LL |     drop(p);
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0506]: cannot assign to `**y` because it is borrowed
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     let p = &y;
[01:08:07]    |             -- borrow of `**y` occurs here
[01:08:07] LL |     let q = &***p;
[01:08:07] LL |     **y = 2; //~ ERROR cannot assign to `**y` because it is borrowed
[01:08:07]    |     ^^^^^^^ assignment to borrowed `**y` occurs here
[01:08:07] LL |     drop(p);
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0506]: cannot assign to `**y.a` because it is borrowed
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     let p = &y.a;
[01:08:07]    |             ---- borrow of `**y.a` occurs here
[01:08:07] LL |     let q = &***p;
[01:08:07] LL |     **y.a = 2; //~ ERROR cannot assign to `**y.a` because it is borrowed
[01:08:07]    |     ^^^^^^^^^ assignment to borrowed `**y.a` occurs here
[01:08:07] LL |     drop(p);
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0506]: cannot assign to `**y.a` because it is borrowed
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     let p = &y.a;
[01:08:07]    |             ---- borrow of `**y.a` occurs here
[01:08:07] LL |     let q = &***p;
[01:08:07] LL |     **y.a = 2; //~ ERROR cannot assign to `**y.a` because it is borrowed
[01:08:07]    |     ^^^^^^^^^ assignment to borrowed `**y.a` occurs here
[01:08:07] LL |     drop(p);
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0506]: cannot assign to `**y.a` because it is borrowed
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     let p = &y.a;
[01:08:07]    |             ---- borrow of `**y.a` occurs here
[01:08:07] LL |     let q = &***p;
[01:08:07] LL |     **y.a = 2; //~ ERROR cannot assign to `**y.a` because it is borrowed
[01:08:07]    |     ^^^^^^^^^ assignment to borrowed `**y.a` occurs here
[01:08:07] LL |     drop(p);
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0506]: cannot assign to `**y.a` because it is borrowed
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     let p = &y.a;
[01:08:07]    |             ---- borrow of `**y.a` occurs here
[01:08:07] LL |     let q = &***p;
[01:08:07] LL |     **y.a = 2; //~ ERROR cannot assign to `**y.a` because it is borrowed
[01:08:07]    |     ^^^^^^^^^ assignment to borrowed `**y.a` occurs here
[01:08:07] LL |     drop(p);
[01:08:07] 
[01:08:07] error: aborting due to 9 previous errors
[01:08:07] 
[01:08:07] Some errors have detailed explanations: E0506, E0594.
---
[01:08:07] 
[01:08:07] 29 
[01:08:07] 30 error: aborting due to 3 previous errors
[01:08:07] 31 
[01:08:07] - For more information about this error, try `rustc --explain E0502`.
[01:08:07] + Some errors have detailed explanations: E0502, E0594.
[01:08:07] 33 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-overloaded-index-ref-index/borrowck-overloaded-index-ref-index.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args borrowck/borrowck-overloaded-index-ref-index.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-overloaded-index-ref-index.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-overloaded-index-ref-index/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-overloaded-index-ref-index/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0502]: cannot borrow `s` as immutable because it is also borrowed as mutable
[01:08:07]   --> /checkout/src/test/ui/borrowck/borrowck-overloaded-index-ref-index.rs:49:22
[01:08:07]    |
[01:08:07] LL |     let rs = &mut s;
[01:08:07]    |              ------ mutable borrow occurs here
[01:08:07] LL |     println!("{}", f[&s]);
[01:08:07]    |                      ^^ immutable borrow occurs here
[01:08:07] LL |     drop(rs);
[01:08:07]    |          -- mutable borrow later used here
[01:08:07] 
[01:08:07] error[E0502]: cannot borrow `s` as immutable because it is also borrowed as mutable
[01:08:07] error[E0502]: cannot borrow `s` as immutable because it is also borrowed as mutable
[01:08:07]   --> /checkout/src/test/ui/borrowck/borrowck-overloaded-index-ref-index.rs:51:7
[01:08:07]    |
[01:08:07] LL |     let rs = &mut s;
[01:08:07]    |              ------ mutable borrow occurs here
[01:08:07] ...
[01:08:07] LL |     f[&s] = 10;
[01:08:07]    |       ^^ immutable borrow occurs here
[01:08:07] LL |     drop(rs);
[01:08:07]    |          -- mutable borrow later used here
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to data in a `&` reference
[01:08:07]    |
[01:08:07] LL |     s[2] = 20;
[01:08:07]    |     ^^^^^^^^^ cannot assign
[01:08:07] 
---
[01:08:07] 
[01:08:07] 22 
[01:08:07] 23 error: aborting due to 3 previous errors
[01:08:07] 24 
[01:08:07] - For more information about this error, try `rustc --explain E0596`.
[01:08:07] + Some errors have detailed explanations: E0594, E0596.
[01:08:07] 26 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/index-mut-help/index-mut-help.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args borrowck/index-mut-help.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/index-mut-help.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/index-mut-help/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/index-mut-help/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0596]: cannot borrow data in a `&` reference as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     map["peter"].clear();           //~ ERROR
[01:08:07]    |     ^^^^^^^^^^^^ cannot borrow as mutable
[01:08:07]    |
[01:08:07]    = help: trait `IndexMut` is required to modify indexed content, but it is not implemented for `std::collections::HashMap<&str, std::string::String>`
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to data in a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     map["peter"] = "0".to_string(); //~ ERROR
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow data in a `&` reference as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     let _ = &mut map["peter"];      //~ ERROR
[01:08:07]    |             ^^^^^^^^^^^^^^^^^ cannot borrow as mutable
[01:08:07]    |
[01:08:07]    = help: trait `IndexMut` is required to modify indexed content, but it is not implemented for `std::collections::HashMap<&str, std::string::String>`
[01:08:07] error: aborting due to 3 previous errors
[01:08:07] 
[01:08:07] Some errors have detailed explanations: E0594, E0596.
[01:08:07] For more information about an error, try `rustc --explain E0594`.
---
[01:08:07] 21 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-45983.nll/issue-45983.nll.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args borrowck/issue-45983.rs`
[01:08:07] 
[01:08:07] error in revision `nll`: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-45983.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-45983.nll/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-45983.nll/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0521]: borrowed data escapes outside of closure
[01:08:07]   --> /checkout/src/test/ui/borrowck/issue-45983.rs:19:18
[01:08:07]    |
[01:08:07] LL |     let x = None;
[01:08:07]    |         - `x` is declared here, outside of the closure body
[01:08:07] LL |     give_any(|y| x = Some(y));
[01:08:07]    |               -  ^^^^^^^^^^^ `y` escapes the closure body here
[01:08:07]    |               `y` is a reference that is only valid in the closure body
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `x`, as it is not declared as mutable
[01:08:07]    |
[01:08:07] LL |     let x = None;
[01:08:07]    |         - help: consider changing this to be mutable: `mut x`
[01:08:07]    |         - help: consider changing this to be mutable: `mut x`
[01:08:07] LL |     give_any(|y| x = Some(y));
[01:08:07] 
[01:08:07] error: aborting due to 2 previous errors
[01:08:07] 
[01:08:07] For more information about this error, try `rustc --explain E0594`.
---
[01:08:07] 
[01:08:07] 84 
[01:08:07] 85 error: aborting due to 9 previous errors
[01:08:07] 86 
[01:08:07] - For more information about this error, try `rustc --explain E0382`.
[01:08:07] + Some errors have detailed explanations: E0382, E0594.
[01:08:07] 88 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-54499-field-mutation-of-moved-out/issue-54499-field-mutation-of-moved-out.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args borrowck/issue-54499-field-mutation-of-moved-out.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-54499-field-mutation-of-moved-out.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-54499-field-mutation-of-moved-out/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-54499-field-mutation-of-moved-out/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `t.0`, as `t` is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         let t: Tuple = (S(0), 0);
[01:08:07] LL |         drop(t);
[01:08:07] LL |         drop(t);
[01:08:07] LL |         t.0 = S(1);
[01:08:07] 
[01:08:07] error[E0382]: assign to part of moved value: `t`
[01:08:07]   --> /checkout/src/test/ui/borrowck/issue-54499-field-mutation-of-moved-out.rs:13:9
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         let t: Tuple = (S(0), 0);
[01:08:07]    |             - move occurs because `t` has type `(S, i32)`, which does not implement the `Copy` trait
[01:08:07] LL |         drop(t);
[01:08:07]    |              - value moved here
[01:08:07] LL |         t.0 = S(1);
[01:08:07]    |         ^^^^^^^^^^ value partially assigned here after move
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `t.1`, as `t` is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         let t: Tuple = (S(0), 0);
[01:08:07] ...
[01:08:07] ...
[01:08:07] LL |         t.1 = 2;
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `u.0`, as `u` is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         let u: Tpair = Tpair(S(0), 0);
[01:08:07] LL |         drop(u);
[01:08:07] LL |         drop(u);
[01:08:07] LL |         u.0 = S(1);
[01:08:07] 
[01:08:07] error[E0382]: assign to part of moved value: `u`
[01:08:07]   --> /checkout/src/test/ui/borrowck/issue-54499-field-mutation-of-moved-out.rs:24:9
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         let u: Tpair = Tpair(S(0), 0);
[01:08:07]    |             - move occurs because `u` has type `Tpair`, which does not implement the `Copy` trait
[01:08:07] LL |         drop(u);
[01:08:07]    |              - value moved here
[01:08:07] LL |         u.0 = S(1);
[01:08:07]    |         ^^^^^^^^^^ value partially assigned here after move
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `u.1`, as `u` is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         let u: Tpair = Tpair(S(0), 0);
[01:08:07] ...
[01:08:07] LL |         u.1 = 2;
[01:08:07]    |         ^^^^^^^ cannot assign
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `v.x`, as `v` is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         let v: Spair = Spair { x: S(0), y: 0 };
[01:08:07]    |             - help: consider changing this to be mutable: `mut v`
[01:08:07] LL |         drop(v);
[01:08:07] LL |         v.x = S(1);
[01:08:07] 
[01:08:07] error[E0382]: assign to part of moved value: `v`
[01:08:07]   --> /checkout/src/test/ui/borrowck/issue-54499-field-mutation-of-moved-out.rs:35:9
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         let v: Spair = Spair { x: S(0), y: 0 };
[01:08:07]    |             - move occurs because `v` has type `Spair`, which does not implement the `Copy` trait
[01:08:07] LL |         drop(v);
[01:08:07]    |              - value moved here
[01:08:07] LL |         v.x = S(1);
[01:08:07]    |         ^^^^^^^^^^ value partially assigned here after move
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `v.y`, as `v` is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         let v: Spair = Spair { x: S(0), y: 0 };
[01:08:07]    |             - help: consider changing this to be mutable: `mut v`
[01:08:07] ...
[01:08:07] LL |         v.y = 2;
[01:08:07] 
[01:08:07] error: aborting due to 9 previous errors
[01:08:07] 
[01:08:07] Some errors have detailed explanations: E0382, E0594.
---
[01:08:07] 
[01:08:07] 50 
[01:08:07] 51 error: aborting due to 6 previous errors
[01:08:07] 52 
[01:08:07] - For more information about this error, try `rustc --explain E0596`.
[01:08:07] + Some errors have detailed explanations: E0594, E0596.
[01:08:07] 54 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.migrate/issue-55492-borrowck-migrate-scans-parents.migrate.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args borrowck/issue-55492-borrowck-migrate-scans-parents.rs`
[01:08:07] 
[01:08:07] error in revision `migrate`: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "migrate" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.migrate/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.migrate/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `x`, as it is not declared as mutable
[01:08:07]    |
[01:08:07] LL |     pub fn e(x: &'static mut isize) {
[01:08:07]    |              - help: consider changing this to be mutable: `mut x`
[01:08:07]    |              - help: consider changing this to be mutable: `mut x`
[01:08:07] LL |         static mut Y: isize = 3;
[01:08:07] LL |         let mut c1 = |y: &'static mut isize| x = y;
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `x`, as it is not declared as mutable
[01:08:07]    |
[01:08:07] LL |     pub fn ee(x: &'static mut isize) {
[01:08:07]    |               - help: consider changing this to be mutable: `mut x`
[01:08:07] ...
[01:08:07] ...
[01:08:07] LL |             let mut c2 = |y: &'static mut isize| x = y;
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `x`, as it is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     pub fn capture_assign_whole(x: (i32,)) {
[01:08:07]    |                                 - help: consider changing this to be mutable: `mut x`
[01:08:07] LL |         || { x = (1,); };
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `x.0`, as `x` is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     pub fn capture_assign_part(x: (i32,)) {
[01:08:07]    |                                - help: consider changing this to be mutable: `mut x`
[01:08:07] LL |         || { x.0 = 1; };
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
[01:08:07]   --> /checkout/src/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs:52:14
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     pub fn capture_reborrow_whole(x: (i32,)) {
[01:08:07]    |                                   - help: consider changing this to be mutable: `mut x`
[01:08:07] LL |         || { &mut x; };
[01:08:07]    |              ^^^^^^ cannot borrow as mutable
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `x.0` as mutable, as `x` is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     pub fn capture_reborrow_part(x: (i32,)) {
[01:08:07]    |                                  - help: consider changing this to be mutable: `mut x`
[01:08:07] LL |         || { &mut x.0; };
[01:08:07]    |              ^^^^^^^^ cannot borrow as mutable
[01:08:07] error: aborting due to 6 previous errors
[01:08:07] 
[01:08:07] Some errors have detailed explanations: E0594, E0596.
[01:08:07] For more information about an error, try `rustc --explain E0594`.
---
[01:08:07] 
[01:08:07] 50 
[01:08:07] 51 error: aborting due to 6 previous errors
[01:08:07] 52 
[01:08:07] - For more information about this error, try `rustc --explain E0596`.
[01:08:07] + Some errors have detailed explanations: E0594, E0596.
[01:08:07] 54 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.nll/issue-55492-borrowck-migrate-scans-parents.nll.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args borrowck/issue-55492-borrowck-migrate-scans-parents.rs`
[01:08:07] 
[01:08:07] error in revision `nll`: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.nll/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.nll/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `x`, as it is not declared as mutable
[01:08:07]    |
[01:08:07] LL |     pub fn e(x: &'static mut isize) {
[01:08:07]    |              - help: consider changing this to be mutable: `mut x`
[01:08:07]    |              - help: consider changing this to be mutable: `mut x`
[01:08:07] LL |         static mut Y: isize = 3;
[01:08:07] LL |         let mut c1 = |y: &'static mut isize| x = y;
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `x`, as it is not declared as mutable
[01:08:07]    |
[01:08:07] LL |     pub fn ee(x: &'static mut isize) {
[01:08:07]    |               - help: consider changing this to be mutable: `mut x`
[01:08:07] ...
[01:08:07] ...
[01:08:07] LL |             let mut c2 = |y: &'static mut isize| x = y;
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `x`, as it is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     pub fn capture_assign_whole(x: (i32,)) {
[01:08:07]    |                                 - help: consider changing this to be mutable: `mut x`
[01:08:07] LL |         || { x = (1,); };
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `x.0`, as `x` is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     pub fn capture_assign_part(x: (i32,)) {
[01:08:07]    |                                - help: consider changing this to be mutable: `mut x`
[01:08:07] LL |         || { x.0 = 1; };
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
[01:08:07]   --> /checkout/src/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs:52:14
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     pub fn capture_reborrow_whole(x: (i32,)) {
[01:08:07]    |                                   - help: consider changing this to be mutable: `mut x`
[01:08:07] LL |         || { &mut x; };
[01:08:07]    |              ^^^^^^ cannot borrow as mutable
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `x.0` as mutable, as `x` is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     pub fn capture_reborrow_part(x: (i32,)) {
[01:08:07]    |                                  - help: consider changing this to be mutable: `mut x`
[01:08:07] LL |         || { &mut x.0; };
[01:08:07]    |              ^^^^^^^^ cannot borrow as mutable
[01:08:07] error: aborting due to 6 previous errors
[01:08:07] 
[01:08:07] Some errors have detailed explanations: E0594, E0596.
[01:08:07] For more information about an error, try `rustc --explain E0594`.
---
[01:08:07] 
[01:08:07] 375 
[01:08:07] 376 error: aborting due to 38 previous errors
[01:08:07] 377 
[01:08:07] - For more information about this error, try `rustc --explain E0596`.
[01:08:07] + Some errors have detailed explanations: E0594, E0596.
[01:08:07] 379 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/mutability-errors/mutability-errors.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args borrowck/mutability-errors.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/mutability-errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/mutability-errors/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/mutability-errors/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `*x` which is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL | fn named_ref(x: &(i32,)) {
[01:08:07]    |                 ------- help: consider changing this to be a mutable reference: `&mut (i32,)`
[01:08:07] LL |     *x = (1,); //~ ERROR
[01:08:07]    |     ^^^^^^^^^ `x` is a `&` reference, so the data it refers to cannot be written
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `x.0` which is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL | fn named_ref(x: &(i32,)) {
[01:08:07]    |                 ------- help: consider changing this to be a mutable reference: `&mut (i32,)`
[01:08:07] LL |     *x = (1,); //~ ERROR
[01:08:07] LL |     x.0 = 1; //~ ERROR
[01:08:07]    |     ^^^^^^^ `x` is a `&` reference, so the data it refers to cannot be written
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `*x` as mutable, as it is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL | fn named_ref(x: &(i32,)) {
[01:08:07]    |                 ------- help: consider changing this to be a mutable reference: `&mut (i32,)`
[01:08:07] ...
[01:08:07] LL |     &mut *x; //~ ERROR
[01:08:07]    |     ^^^^^^^ `x` is a `&` reference, so the data it refers to cannot be borrowed as mutable
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `x.0` as mutable, as it is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL | fn named_ref(x: &(i32,)) {
[01:08:07]    |                 ------- help: consider changing this to be a mutable reference: `&mut (i32,)`
[01:08:07] ...
[01:08:07] LL |     &mut x.0; //~ ERROR
[01:08:07]    |     ^^^^^^^^ `x` is a `&` reference, so the data it refers to cannot be borrowed as mutable
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to data in a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     *f() = (1,); //~ ERROR
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to data in a `&` reference
[01:08:07]    |
[01:08:07] LL |     f().0 = 1; //~ ERROR
[01:08:07]    |     ^^^^^^^^^ cannot assign
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow data in a `&` reference as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     &mut *f(); //~ ERROR
[01:08:07]    |     ^^^^^^^^^ cannot borrow as mutable
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow data in a `&` reference as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     &mut f().0; //~ ERROR
[01:08:07]    |     ^^^^^^^^^^ cannot borrow as mutable
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `*x` which is behind a `*const` pointer
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL | unsafe fn named_ptr(x: *const (i32,)) {
[01:08:07]    |                        ------------- help: consider changing this to be a mutable pointer: `*mut (i32,)`
[01:08:07] LL |     *x = (1,); //~ ERROR
[01:08:07]    |     ^^^^^^^^^ `x` is a `*const` pointer, so the data it refers to cannot be written
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `x.0` which is behind a `*const` pointer
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL | unsafe fn named_ptr(x: *const (i32,)) {
[01:08:07]    |                        ------------- help: consider changing this to be a mutable pointer: `*mut (i32,)`
[01:08:07] LL |     *x = (1,); //~ ERROR
[01:08:07] LL |     (*x).0 = 1; //~ ERROR
[01:08:07]    |     ^^^^^^^^^^ `x` is a `*const` pointer, so the data it refers to cannot be written
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `*x` as mutable, as it is behind a `*const` pointer
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL | unsafe fn named_ptr(x: *const (i32,)) {
[01:08:07]    |                        ------------- help: consider changing this to be a mutable pointer: `*mut (i32,)`
[01:08:07] ...
[01:08:07] LL |     &mut *x; //~ ERROR
[01:08:07]    |     ^^^^^^^ `x` is a `*const` pointer, so the data it refers to cannot be borrowed as mutable
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `x.0` as mutable, as it is behind a `*const` pointer
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL | unsafe fn named_ptr(x: *const (i32,)) {
[01:08:07]    |                        ------------- help: consider changing this to be a mutable pointer: `*mut (i32,)`
[01:08:07] ...
[01:08:07] LL |     &mut (*x).0; //~ ERROR
[01:08:07]    |     ^^^^^^^^^^^ `x` is a `*const` pointer, so the data it refers to cannot be borrowed as mutable
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to data in a `*const` pointer
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     *f() = (1,); //~ ERROR
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to data in a `*const` pointer
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     (*f()).0 = 1; //~ ERROR
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow data in a `*const` pointer as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     &mut *f(); //~ ERROR
[01:08:07]    |     ^^^^^^^^^ cannot borrow as mutable
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow data in a `*const` pointer as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     &mut (*f()).0; //~ ERROR
[01:08:07]    |     ^^^^^^^^^^^^^ cannot borrow as mutable
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `x`, as it is a captured variable in a `Fn` closure
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         x = (1,); //~ ERROR
[01:08:07]    |
[01:08:07]    |
[01:08:07] help: consider changing this to accept closures that implement `FnMut`
[01:08:07]    |
[01:08:07] LL |       fn_ref(|| {
[01:08:07]    |  ____________^
[01:08:07]    |  ____________^
[01:08:07] LL | |         x = (1,); //~ ERROR
[01:08:07] LL | |         x.0 = 1; //~ ERROR
[01:08:07] LL | |         &mut x; //~ ERROR
[01:08:07] LL | |         &mut x.0; //~ ERROR
[01:08:07] LL | |     });
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `x.0`, as `Fn` closures cannot mutate their captured variables
[01:08:07]    |
[01:08:07] LL |         x.0 = 1; //~ ERROR
[01:08:07]    |         ^^^^^^^ cannot assign
[01:08:07]    |
[01:08:07]    |
[01:08:07] help: consider changing this to accept closures that implement `FnMut`
[01:08:07]    |
[01:08:07] LL |       fn_ref(|| {
[01:08:07]    |  ____________^
[01:08:07]    |  ____________^
[01:08:07] LL | |         x = (1,); //~ ERROR
[01:08:07] LL | |         x.0 = 1; //~ ERROR
[01:08:07] LL | |         &mut x; //~ ERROR
[01:08:07] LL | |         &mut x.0; //~ ERROR
[01:08:07] LL | |     });
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `x` as mutable, as it is a captured variable in a `Fn` closure
[01:08:07]    |
[01:08:07] LL |         &mut x; //~ ERROR
[01:08:07]    |         ^^^^^^ cannot borrow as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] help: consider changing this to accept closures that implement `FnMut`
[01:08:07]    |
[01:08:07] LL |       fn_ref(|| {
[01:08:07]    |  ____________^
[01:08:07]    |  ____________^
[01:08:07] LL | |         x = (1,); //~ ERROR
[01:08:07] LL | |         x.0 = 1; //~ ERROR
[01:08:07] LL | |         &mut x; //~ ERROR
[01:08:07] LL | |         &mut x.0; //~ ERROR
[01:08:07] LL | |     });
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `x.0` as mutable, as `Fn` closures cannot mutate their captured variables
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         &mut x.0; //~ ERROR
[01:08:07]    |         ^^^^^^^^ cannot borrow as mutable
[01:08:07]    |
[01:08:07] help: consider changing this to accept closures that implement `FnMut`
[01:08:07]    |
[01:08:07] LL |       fn_ref(|| {
[01:08:07]    |  ____________^
[01:08:07]    |  ____________^
[01:08:07] LL | |         x = (1,); //~ ERROR
[01:08:07] LL | |         x.0 = 1; //~ ERROR
[01:08:07] LL | |         &mut x; //~ ERROR
[01:08:07] LL | |         &mut x.0; //~ ERROR
[01:08:07] LL | |     });
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `x`, as it is a captured variable in a `Fn` closure
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         x = (1,); //~ ERROR
[01:08:07]    |
[01:08:07]    |
[01:08:07] help: consider changing this to accept closures that implement `FnMut`
[01:08:07]    |
[01:08:07] LL |       fn_ref(move || {
[01:08:07]    |  ____________^
[01:08:07]    |  ____________^
[01:08:07] LL | |         x = (1,); //~ ERROR
[01:08:07] LL | |         x.0 = 1; //~ ERROR
[01:08:07] LL | |         &mut x; //~ ERROR
[01:08:07] LL | |         &mut x.0; //~ ERROR
[01:08:07] LL | |     });
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `x.0`, as `Fn` closures cannot mutate their captured variables
[01:08:07]    |
[01:08:07] LL |         x.0 = 1; //~ ERROR
[01:08:07]    |         ^^^^^^^ cannot assign
[01:08:07]    |
[01:08:07]    |
[01:08:07] help: consider changing this to accept closures that implement `FnMut`
[01:08:07]    |
[01:08:07] LL |       fn_ref(move || {
[01:08:07]    |  ____________^
[01:08:07]    |  ____________^
[01:08:07] LL | |         x = (1,); //~ ERROR
[01:08:07] LL | |         x.0 = 1; //~ ERROR
[01:08:07] LL | |         &mut x; //~ ERROR
[01:08:07] LL | |         &mut x.0; //~ ERROR
[01:08:07] LL | |     });
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `x` as mutable, as it is a captured variable in a `Fn` closure
[01:08:07]    |
[01:08:07] LL |         &mut x; //~ ERROR
[01:08:07]    |         ^^^^^^ cannot borrow as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] help: consider changing this to accept closures that implement `FnMut`
[01:08:07]    |
[01:08:07] LL |       fn_ref(move || {
[01:08:07]    |  ____________^
[01:08:07]    |  ____________^
[01:08:07] LL | |         x = (1,); //~ ERROR
[01:08:07] LL | |         x.0 = 1; //~ ERROR
[01:08:07] LL | |         &mut x; //~ ERROR
[01:08:07] LL | |         &mut x.0; //~ ERROR
[01:08:07] LL | |     });
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `x.0` as mutable, as `Fn` closures cannot mutate their captured variables
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         &mut x.0; //~ ERROR
[01:08:07]    |         ^^^^^^^^ cannot borrow as mutable
[01:08:07]    |
[01:08:07] help: consider changing this to accept closures that implement `FnMut`
[01:08:07]    |
[01:08:07] LL |       fn_ref(move || {
[01:08:07]    |  ____________^
[01:08:07]    |  ____________^
[01:08:07] LL | |         x = (1,); //~ ERROR
[01:08:07] LL | |         x.0 = 1; //~ ERROR
[01:08:07] LL | |         &mut x; //~ ERROR
[01:08:07] LL | |         &mut x.0; //~ ERROR
[01:08:07] LL | |     });
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
[01:08:07]   --> /checkout/src/test/ui/borrowck/mutability-errors.rs:54:5
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL | fn imm_local(x: (i32,)) {
[01:08:07] LL |     &mut x; //~ ERROR
[01:08:07]    |     ^^^^^^ cannot borrow as mutable
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `x.0` as mutable, as `x` is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL | fn imm_local(x: (i32,)) {
[01:08:07] LL |     &mut x; //~ ERROR
[01:08:07] LL |     &mut x; //~ ERROR
[01:08:07] LL |     &mut x.0; //~ ERROR
[01:08:07]    |     ^^^^^^^^ cannot borrow as mutable
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `x`, as it is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL | fn imm_capture(x: (i32,)) {
[01:08:07] LL |     || {
[01:08:07] LL |     || {
[01:08:07] LL |         x = (1,); //~ ERROR
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `x.0`, as `x` is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL | fn imm_capture(x: (i32,)) {
[01:08:07] ...
[01:08:07] LL |         x.0 = 1; //~ ERROR
[01:08:07]    |         ^^^^^^^ cannot assign
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
[01:08:07]   --> /checkout/src/test/ui/borrowck/mutability-errors.rs:62:9
[01:08:07]    |
[01:08:07] LL | fn imm_capture(x: (i32,)) {
[01:08:07] ...
[01:08:07] LL |         &mut x; //~ ERROR
[01:08:07]    |         ^^^^^^ cannot borrow as mutable
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `x.0` as mutable, as `x` is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL | fn imm_capture(x: (i32,)) {
[01:08:07] ...
[01:08:07] ...
[01:08:07] LL |         &mut x.0; //~ ERROR
[01:08:07]    |         ^^^^^^^^ cannot borrow as mutable
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `x`, as it is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL | fn imm_capture(x: (i32,)) {
[01:08:07] ...
[01:08:07] ...
[01:08:07] LL |         x = (1,); //~ ERROR
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `x.0`, as `x` is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL | fn imm_capture(x: (i32,)) {
[01:08:07] ...
[01:08:07] LL |         x.0 = 1; //~ ERROR
[01:08:07]    |         ^^^^^^^ cannot assign
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
[01:08:07]   --> /checkout/src/test/ui/borrowck/mutability-errors.rs:68:9
[01:08:07]    |
[01:08:07] LL | fn imm_capture(x: (i32,)) {
[01:08:07] ...
[01:08:07] LL |         &mut x; //~ ERROR
[01:08:07]    |         ^^^^^^ cannot borrow as mutable
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `x.0` as mutable, as `x` is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL | fn imm_capture(x: (i32,)) {
[01:08:07] ...
[01:08:07] ...
[01:08:07] LL |         &mut x.0; //~ ERROR
[01:08:07]    |         ^^^^^^^^ cannot borrow as mutable
[01:08:07] error[E0594]: cannot assign to immutable static item `X`
[01:08:07]   --> /checkout/src/test/ui/borrowck/mutability-errors.rs:76:5
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     X = (1,); //~ ERROR
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `X.0`, as `X` is an immutable static item
[01:08:07]    |
[01:08:07] LL |     X.0 = 1; //~ ERROR
[01:08:07]    |     ^^^^^^^ cannot assign
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow immutable static item `X` as mutable
[01:08:07]    |
[01:08:07] LL |     &mut X; //~ ERROR
[01:08:07]    |     ^^^^^^ cannot borrow as mutable
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `X.0` as mutable, as `X` is an immutable static item
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     &mut X.0; //~ ERROR
[01:08:07]    |     ^^^^^^^^ cannot borrow as mutable
[01:08:07] error: aborting due to 38 previous errors
[01:08:07] 
[01:08:07] Some errors have detailed explanations: E0594, E0596.
[01:08:07] For more information about an error, try `rustc --explain E0594`.
---
[01:08:07] 
[01:08:07] 15 
[01:08:07] 16 error: aborting due to 2 previous errors
[01:08:07] 17 
[01:08:07] - For more information about this error, try `rustc --explain E0381`.
[01:08:07] + Some errors have detailed explanations: E0381, E0594.
[01:08:07] 19 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/reassignment_immutable_fields_overlapping/reassignment_immutable_fields_overlapping.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args borrowck/reassignment_immutable_fields_overlapping.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/reassignment_immutable_fields_overlapping.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/reassignment_immutable_fields_overlapping/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/reassignment_immutable_fields_overlapping/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0381]: assign to part of possibly uninitialized variable: `x`
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     x.a = 1;  //~ ERROR
[01:08:07]    |     ^^^^^^^ use of possibly uninitialized `x`
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `x.b`, as `x` is not declared as mutable
[01:08:07]    |
[01:08:07] LL |     let x: Foo;
[01:08:07]    |         - help: consider changing this to be mutable: `mut x`
[01:08:07]    |         - help: consider changing this to be mutable: `mut x`
[01:08:07] LL |     x.a = 1;  //~ ERROR
[01:08:07] LL |     x.b = 22; //~ ERROR
[01:08:07] 
[01:08:07] error: aborting due to 2 previous errors
[01:08:07] 
[01:08:07] Some errors have detailed explanations: E0381, E0594.
---
[01:08:07] 
[01:08:07] 15 
[01:08:07] 16 error: aborting due to 2 previous errors
[01:08:07] 17 
[01:08:07] - For more information about this error, try `rustc --explain E0381`.
[01:08:07] + Some errors have detailed explanations: E0381, E0594.
[01:08:07] 19 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/reassignment_immutable_fields_twice/reassignment_immutable_fields_twice.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args borrowck/reassignment_immutable_fields_twice.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/reassignment_immutable_fields_twice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/reassignment_immutable_fields_twice/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/reassignment_immutable_fields_twice/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `x.0`, as `x` is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     let x: (u32, u32);
[01:08:07] LL |     x = (22, 44);
[01:08:07] LL |     x.0 = 1; //~ ERROR
[01:08:07]    |     ^^^^^^^ cannot assign
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0381]: assign to part of possibly uninitialized variable: `x`
[01:08:07]    |
[01:08:07] LL |     x.0 = 1; //~ ERROR
[01:08:07] LL |     x.0 = 1; //~ ERROR
[01:08:07]    |     ^^^^^^^ use of possibly uninitialized `x`
[01:08:07] error: aborting due to 2 previous errors
[01:08:07] 
[01:08:07] Some errors have detailed explanations: E0381, E0594.
[01:08:07] For more information about an error, try `rustc --explain E0381`.
---
[01:08:07] 
[01:08:07] 16 
[01:08:07] 17 error: aborting due to 2 previous errors
[01:08:07] 18 
[01:08:07] - For more information about this error, try `rustc --explain E0596`.
[01:08:07] + Some errors have detailed explanations: E0594, E0596.
[01:08:07] 20 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cannot-mutate-captured-non-mut-var/cannot-mutate-captured-non-mut-var.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args cannot-mutate-captured-non-mut-var.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cannot-mutate-captured-non-mut-var.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cannot-mutate-captured-non-mut-var/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cannot-mutate-captured-non-mut-var/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `x`, as it is not declared as mutable
[01:08:07]    |
[01:08:07] LL |     let x = 1;
[01:08:07]    |         - help: consider changing this to be mutable: `mut x`
[01:08:07]    |         - help: consider changing this to be mutable: `mut x`
[01:08:07] LL |     to_fn_once(move|| { x = 2; });
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `s` as mutable, as it is not declared as mutable
[01:08:07]    |
[01:08:07] LL |     let s = std::io::stdin();
[01:08:07]    |         - help: consider changing this to be mutable: `mut s`
[01:08:07]    |         - help: consider changing this to be mutable: `mut s`
[01:08:07] LL |     to_fn_once(move|| { s.read_to_end(&mut Vec::new()); });
[01:08:07]    |                         ^ cannot borrow as mutable
[01:08:07] error: aborting due to 2 previous errors
[01:08:07] 
[01:08:07] Some errors have detailed explanations: E0594, E0596.
[01:08:07] For more information about an error, try `rustc --explain E0594`.
---
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-immutable-outer-variable/closure-immutable-outer-variable.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args closures/closure-immutable-outer-variable.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/closure-immutable-outer-variable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-immutable-outer-variable/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-immutable-outer-variable/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `y`, as it is not declared as mutable
[01:08:07]   --> /checkout/src/test/ui/closures/closure-immutable-outer-variable.rs:11:26
[01:08:07]    |
[01:08:07] LL |     let y = true;
[01:08:07]    |         - help: consider changing this to be mutable: `mut y`
[01:08:07] LL |     foo(Box::new(move || y = false) as Box<_>);
[01:08:07] 
[01:08:07] error: aborting due to previous error
[01:08:07] 
[01:08:07] For more information about this error, try `rustc --explain E0594`.
---
[01:08:07] 
[01:08:07] 24 
[01:08:07] 25 error: aborting due to 3 previous errors
[01:08:07] 26 
[01:08:07] - For more information about this error, try `rustc --explain E0596`.
[01:08:07] + Some errors have detailed explanations: E0594, E0596.
[01:08:07] 28 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-35937/issue-35937.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args did_you_mean/issue-35937.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/issue-35937.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-35937/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-35937/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0596]: cannot borrow `f.v` as mutable, as `f` is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     let f = Foo { v: Vec::new() };
[01:08:07]    |         - help: consider changing this to be mutable: `mut f`
[01:08:07] LL |     f.v.push("cat".to_string()); //~ ERROR cannot borrow
[01:08:07]    |     ^^^ cannot borrow as mutable
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `s.x`, as `s` is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     let s = S { x: 42 };
[01:08:07]    |         - help: consider changing this to be mutable: `mut s`
[01:08:07] LL |     s.x += 1; //~ ERROR cannot assign
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `s.x`, as `s` is not declared as mutable
[01:08:07]    |
[01:08:07] LL | fn bar(s: S) {
[01:08:07]    |        - help: consider changing this to be mutable: `mut s`
[01:08:07]    |        - help: consider changing this to be mutable: `mut s`
[01:08:07] LL |     s.x += 1; //~ ERROR cannot assign
[01:08:07] 
[01:08:07] error: aborting due to 3 previous errors
[01:08:07] 
[01:08:07] Some errors have detailed explanations: E0594, E0596.
---
[01:08:07] 
[01:08:07] 98 
[01:08:07] 99 error: aborting due to 12 previous errors
[01:08:07] 100 
[01:08:07] - For more information about this error, try `rustc --explain E0596`.
[01:08:07] + Some errors have detailed explanations: E0594, E0596.
[01:08:07] 102 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-39544/issue-39544.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args did_you_mean/issue-39544.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/issue-39544.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-39544/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-39544/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0596]: cannot borrow `z.x` as mutable, as `z` is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     let z = Z { x: X::Y };
[01:08:07]    |         - help: consider changing this to be mutable: `mut z`
[01:08:07] LL |     let _ = &mut z.x; //~ ERROR cannot borrow
[01:08:07]    |             ^^^^^^^^ cannot borrow as mutable
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `self.x` as mutable, as it is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     fn foo<'z>(&'z self) {
[01:08:07]    |                -------- help: consider changing this to be a mutable reference: `&'z mut self`
[01:08:07] LL |         let _ = &mut self.x; //~ ERROR cannot borrow
[01:08:07]    |                 ^^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `self.x` as mutable, as it is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     fn foo1(&self, other: &Z) {
[01:08:07]    |             ----- help: consider changing this to be a mutable reference: `&mut self`
[01:08:07] LL |         let _ = &mut self.x; //~ ERROR cannot borrow
[01:08:07]    |                 ^^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `other.x` as mutable, as it is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     fn foo1(&self, other: &Z) {
[01:08:07]    |                           -- help: consider changing this to be a mutable reference: `&mut Z`
[01:08:07] LL |         let _ = &mut self.x; //~ ERROR cannot borrow
[01:08:07] LL |         let _ = &mut other.x; //~ ERROR cannot borrow
[01:08:07]    |                 ^^^^^^^^^^^^ `other` is a `&` reference, so the data it refers to cannot be borrowed as mutable
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `self.x` as mutable, as it is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     fn foo2<'a>(&'a self, other: &Z) {
[01:08:07]    |                 -------- help: consider changing this to be a mutable reference: `&'a mut self`
[01:08:07] LL |         let _ = &mut self.x; //~ ERROR cannot borrow
[01:08:07]    |                 ^^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `other.x` as mutable, as it is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     fn foo2<'a>(&'a self, other: &Z) {
[01:08:07]    |                                  -- help: consider changing this to be a mutable reference: `&mut Z`
[01:08:07] LL |         let _ = &mut self.x; //~ ERROR cannot borrow
[01:08:07] LL |         let _ = &mut other.x; //~ ERROR cannot borrow
[01:08:07]    |                 ^^^^^^^^^^^^ `other` is a `&` reference, so the data it refers to cannot be borrowed as mutable
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `self.x` as mutable, as it is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     fn foo3<'a>(self: &'a Self, other: &Z) {
[01:08:07]    |                       -------- help: consider changing this to be a mutable reference: `&'a mut Self`
[01:08:07] LL |         let _ = &mut self.x; //~ ERROR cannot borrow
[01:08:07]    |                 ^^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `other.x` as mutable, as it is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     fn foo3<'a>(self: &'a Self, other: &Z) {
[01:08:07]    |                                        -- help: consider changing this to be a mutable reference: `&mut Z`
[01:08:07] LL |         let _ = &mut self.x; //~ ERROR cannot borrow
[01:08:07] LL |         let _ = &mut other.x; //~ ERROR cannot borrow
[01:08:07]    |                 ^^^^^^^^^^^^ `other` is a `&` reference, so the data it refers to cannot be borrowed as mutable
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `other.x` as mutable, as it is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     fn foo4(other: &Z) {
[01:08:07]    |                    -- help: consider changing this to be a mutable reference: `&mut Z`
[01:08:07] LL |         let _ = &mut other.x; //~ ERROR cannot borrow
[01:08:07]    |                 ^^^^^^^^^^^^ `other` is a `&` reference, so the data it refers to cannot be borrowed as mutable
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `z.x` as mutable, as `z` is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL | pub fn with_arg(z: Z, w: &Z) {
[01:08:07]    |                 - help: consider changing this to be mutable: `mut z`
[01:08:07] LL |     let _ = &mut z.x; //~ ERROR cannot borrow
[01:08:07]    |             ^^^^^^^^ cannot borrow as mutable
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `w.x` as mutable, as it is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL | pub fn with_arg(z: Z, w: &Z) {
[01:08:07]    |                          -- help: consider changing this to be a mutable reference: `&mut Z`
[01:08:07] LL |     let _ = &mut z.x; //~ ERROR cannot borrow
[01:08:07] LL |     let _ = &mut w.x; //~ ERROR cannot borrow
[01:08:07]    |             ^^^^^^^^ `w` is a `&` reference, so the data it refers to cannot be borrowed as mutable
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `*x.0` which is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     *x.0 = 1;
[01:08:07] 
[01:08:07] error: aborting due to 12 previous errors
[01:08:07] 
[01:08:07] Some errors have detailed explanations: E0594, E0596.
---
[01:08:07] 11 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0389/E0389.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args error-codes/E0389.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0389.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0389/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0389/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `fancy_ref.num` which is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     let fancy_ref = &(&mut fancy);
[01:08:07]    |                     ------------- help: consider changing this to be a mutable reference: `&mut (&mut fancy)`
[01:08:07] LL |     fancy_ref.num = 6; //~ ERROR cannot assign to `fancy_ref.num` which is behind a `&` reference
[01:08:07]    |     ^^^^^^^^^^^^^^^^^ `fancy_ref` is a `&` reference, so the data it refers to cannot be written
[01:08:07] error: aborting due to previous error
[01:08:07] 
[01:08:07] For more information about this error, try `rustc --explain E0594`.
[01:08:07] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
---
[01:08:07] 15 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/fn-closure-mutable-capture/fn-closure-mutable-capture.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args fn/fn-closure-mutable-capture.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fn/fn-closure-mutable-capture.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/fn-closure-mutable-capture/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/fn-closure-mutable-capture/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `x`, as it is a captured variable in a `Fn` closure
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     bar(move || x = 1);
[01:08:07]    |
[01:08:07]    |
[01:08:07] help: consider changing this to accept closures that implement `FnMut`
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     bar(move || x = 1);
[01:08:07] 
[01:08:07] error: aborting due to previous error
[01:08:07] 
[01:08:07] For more information about this error, try `rustc --explain E0594`.
---
[01:08:07] 19 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/immut-function-arguments/immut-function-arguments.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args immut-function-arguments.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/immut-function-arguments.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/immut-function-arguments/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/immut-function-arguments/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `*y`, as `y` is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL | fn f(y: Box<isize>) {
[01:08:07]    |      - help: consider changing this to be mutable: `mut y`
[01:08:07] LL |     *y = 5; //~ ERROR cannot assign
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `*q`, as `q` is not declared as mutable
[01:08:07]   --> /checkout/src/test/ui/immut-function-arguments.rs:6:35
[01:08:07] make: *** [check] Error 1
[01:08:07]    |
[01:08:07] LL |     let _frob = |q: Box<isize>| { *q = 2; }; //~ ERROR cannot assign
[01:08:07]    |                  |
[01:08:07]    |                  help: consider changing this to be mutable: `mut q`
[01:08:07] 
[01:08:07] error: aborting due to 2 previous errors
---
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46023/issue-46023.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args issues/issue-46023.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-46023.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46023/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46023/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `x`, as it is not declared as mutable
[01:08:07]    |
[01:08:07] LL |     let x = 0;
[01:08:07]    |         - help: consider changing this to be mutable: `mut x`
[01:08:07] ...
---
[01:08:07] 
[01:08:07] 12 
[01:08:07] 13 error: aborting due to 2 previous errors
[01:08:07] 14 
[01:08:07] - For more information about this error, try `rustc --explain E0017`.
[01:08:07] + Some errors have detailed explanations: E0017, E0594.
[01:08:07] 16 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46604/issue-46604.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args issues/issue-46604.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-46604.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46604/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46604/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0017]: references in statics may only refer to immutable values
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL | static buf: &mut [u8] = &mut [1u8,2,3,4,5,7];   //~ ERROR E0017
[01:08:07]    |                         ^^^^^^^^^^^^^^^^^^^^ statics require immutable values
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `buf[_]`, as `buf` is an immutable static item
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     buf[0]=2;                                   //~ ERROR E0594
[01:08:07] 
[01:08:07] error: aborting due to 2 previous errors
[01:08:07] 
[01:08:07] Some errors have detailed explanations: E0017, E0594.
---
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51244/issue-51244.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args issues/issue-51244.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-51244.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51244/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51244/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `*my_ref` which is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     let ref my_ref @ _ = 0;
[01:08:07]    |         -------------- help: consider changing this to be a mutable reference: `ref mut my_ref @ _`
[01:08:07] LL |     *my_ref = 0; //~ ERROR cannot assign to `*my_ref` which is behind a `&` reference [E0594]
[01:08:07]    |     ^^^^^^^^^^^ `my_ref` is a `&` reference, so the data it refers to cannot be written
[01:08:07] error: aborting due to previous error
[01:08:07] 
[01:08:07] For more information about this error, try `rustc --explain E0594`.
[01:08:07] 
---
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51515/issue-51515.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args issues/issue-51515.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-51515.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51515/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51515/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `*foo` which is behind a `&` reference
[01:08:07]    |
[01:08:07] LL |     let foo = &16;
[01:08:07]    |               --- help: consider changing this to be a mutable reference: `&mut 16`
[01:08:07] ...
[01:08:07] ...
[01:08:07] LL |     *foo = 32;
[01:08:07]    |     ^^^^^^^^^ `foo` is a `&` reference, so the data it refers to cannot be written
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `*bar` which is behind a `&` reference
[01:08:07]    |
[01:08:07] LL |     let bar = foo;
[01:08:07]    |         --- help: consider changing this to be a mutable reference: `&mut i32`
[01:08:07] ...
[01:08:07] ...
[01:08:07] LL |     *bar = 64;
[01:08:07]    |     ^^^^^^^^^ `bar` is a `&` reference, so the data it refers to cannot be written
[01:08:07] error: aborting due to 2 previous errors
[01:08:07] 
[01:08:07] For more information about this error, try `rustc --explain E0594`.
[01:08:07] 
---
[01:08:07] 11 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mut/mutable-class-fields-2/mutable-class-fields-2.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args mut/mutable-class-fields-2.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mut/mutable-class-fields-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mut/mutable-class-fields-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mut/mutable-class-fields-2/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `self.how_hungry` which is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |   pub fn eat(&self) {
[01:08:07]    |              ----- help: consider changing this to be a mutable reference: `&mut self`
[01:08:07] LL |     self.how_hungry -= 5; //~ ERROR cannot assign
[01:08:07]    |     ^^^^^^^^^^^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be written
[01:08:07] error: aborting due to previous error
[01:08:07] 
[01:08:07] For more information about this error, try `rustc --explain E0594`.
[01:08:07] 
---
[01:08:07] 11 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mut/mutable-class-fields/mutable-class-fields.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args mut/mutable-class-fields.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mut/mutable-class-fields.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mut/mutable-class-fields/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mut/mutable-class-fields/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `nyan.how_hungry`, as `nyan` is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |   let nyan : Cat = cat(52, 99);
[01:08:07]    |       ---- help: consider changing this to be mutable: `mut nyan`
[01:08:07] LL |   nyan.how_hungry = 0; //~ ERROR cannot assign
[01:08:07] 
[01:08:07] error: aborting due to previous error
[01:08:07] 
[01:08:07] For more information about this error, try `rustc --explain E0594`.
---
[01:08:07] 
[01:08:07] 156 
[01:08:07] 157 error: aborting due to 12 previous errors
[01:08:07] 158 
[01:08:07] - For more information about this error, try `rustc --explain E0596`.
[01:08:07] + Some errors have detailed explanations: E0594, E0596.
[01:08:07] 160 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-captures/closure-captures.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args nll/closure-captures.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-captures.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-captures/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-captures/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `x`, as it is not declared as mutable
[01:08:07]    |
[01:08:07] LL | fn one_closure(x: i32) {
[01:08:07]    |                - help: consider changing this to be mutable: `mut x`
[01:08:07] LL |     ||
[01:08:07] LL |     ||
[01:08:07] LL |     x = 1; //~ ERROR
[01:08:07]    |     ^^^^^ cannot assign
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `x`, as it is not declared as mutable
[01:08:07]    |
[01:08:07] LL | fn one_closure(x: i32) {
[01:08:07]    |                - help: consider changing this to be mutable: `mut x`
[01:08:07] ...
[01:08:07] ...
[01:08:07] LL |     x = 1; //~ ERROR
[01:08:07]    |     ^^^^^ cannot assign
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `x`, as it is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL | fn two_closures(x: i32) {
[01:08:07] ...
[01:08:07] LL |         x = 1; //~ ERROR
[01:08:07]    |         ^^^^^ cannot assign
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `x`, as it is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL | fn two_closures(x: i32) {
[01:08:07] ...
[01:08:07] LL |         x = 1; //~ ERROR
[01:08:07]    |         ^^^^^ cannot assign
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `x` as mutable, as it is a captured variable in a `Fn` closure
[01:08:07]    |
[01:08:07] LL |         || //~ ERROR
[01:08:07]    |         ^^ cannot borrow as mutable
[01:08:07] LL |          x = 1;}
[01:08:07] LL |          x = 1;}
[01:08:07]    |          - mutable borrow occurs due to use of `x` in closure
[01:08:07]    |
[01:08:07] help: consider changing this to accept closures that implement `FnMut`
[01:08:07]    |
[01:08:07] LL |       fn_ref(|| {
[01:08:07]    |  ____________^
[01:08:07] LL | |         || //~ ERROR
[01:08:07] LL | |         || //~ ERROR
[01:08:07] LL | |          x = 1;}
[01:08:07]    | |________________^
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `x` as mutable, as it is a captured variable in a `Fn` closure
[01:08:07]    |
[01:08:07] LL |         ||  //~ ERROR
[01:08:07]    |         ^^ cannot borrow as mutable
[01:08:07] LL |     x = 1;});
[01:08:07] LL |     x = 1;});
[01:08:07]    |     - mutable borrow occurs due to use of `x` in closure
[01:08:07]    |
[01:08:07] help: consider changing this to accept closures that implement `FnMut`
[01:08:07]    |
[01:08:07] LL |       fn_ref(move || {
[01:08:07]    |  ____________^
[01:08:07] LL | |         ||  //~ ERROR
[01:08:07] LL | |         ||  //~ ERROR
[01:08:07] LL | |     x = 1;});
[01:08:07]    | |___________^
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `x`, as it is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL | fn two_closures_ref(x: i32) {
[01:08:07] ...
[01:08:07] LL |          x = 1;} //~ ERROR
[01:08:07]    |          ^^^^^ cannot assign
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `x` as mutable, as it is a captured variable in a `Fn` closure
[01:08:07]    |
[01:08:07] LL |         || //~ ERROR
[01:08:07]    |         ^^ cannot borrow as mutable
[01:08:07] LL |          x = 1;} //~ ERROR
[01:08:07] LL |          x = 1;} //~ ERROR
[01:08:07]    |          - mutable borrow occurs due to use of `x` in closure
[01:08:07]    |
[01:08:07] help: consider changing this to accept closures that implement `FnMut`
[01:08:07]    |
[01:08:07] LL |       fn_ref(|| {
[01:08:07]    |  ____________^
[01:08:07] LL | |         || //~ ERROR
[01:08:07] LL | |         || //~ ERROR
[01:08:07] LL | |          x = 1;} //~ ERROR
[01:08:07]    | |________________^
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `x`, as it is not declared as mutable
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL | fn two_closures_ref(x: i32) {
[01:08:07] ...
[01:08:07] LL |     x = 1;}); //~ ERROR
[01:08:07]    |     ^^^^^ cannot assign
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `x` as mutable, as it is a captured variable in a `Fn` closure
[01:08:07]    |
[01:08:07] LL |         ||  //~ ERROR
[01:08:07]    |         ^^ cannot borrow as mutable
[01:08:07] LL |     x = 1;}); //~ ERROR
[01:08:07] LL |     x = 1;}); //~ ERROR
[01:08:07]    |     - mutable borrow occurs due to use of `x` in closure
[01:08:07]    |
[01:08:07] help: consider changing this to accept closures that implement `FnMut`
[01:08:07]    |
[01:08:07] LL |       fn_ref(move || {
[01:08:07]    |  ____________^
[01:08:07] LL | |         ||  //~ ERROR
[01:08:07] LL | |         ||  //~ ERROR
[01:08:07] LL | |     x = 1;}); //~ ERROR
[01:08:07]    | |___________^
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `x` as mutable, as it is a captured variable in a `Fn` closure
[01:08:07]    |
[01:08:07] LL |         || //~ ERROR
[01:08:07]    |         ^^ cannot borrow as mutable
[01:08:07]    |         ^^ cannot borrow as mutable
[01:08:07] LL |         *x = 1;});
[01:08:07]    |          - mutable borrow occurs due to use of `x` in closure
[01:08:07]    |
[01:08:07] help: consider changing this to accept closures that implement `FnMut`
[01:08:07]    |
[01:08:07] LL |       fn_ref(|| {
[01:08:07]    |  ____________^
[01:08:07] LL | |         || //~ ERROR
[01:08:07] LL | |         || //~ ERROR
[01:08:07] LL | |         *x = 1;});
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `x` as mutable, as it is a captured variable in a `Fn` closure
[01:08:07]    |
[01:08:07] LL |         || //~ ERROR
[01:08:07]    |         ^^ cannot borrow as mutable
[01:08:07]    |         ^^ cannot borrow as mutable
[01:08:07] LL |         *x = 1;});
[01:08:07]    |          - mutable borrow occurs due to use of `x` in closure
[01:08:07]    |
[01:08:07] help: consider changing this to accept closures that implement `FnMut`
[01:08:07]    |
[01:08:07] LL |       fn_ref(move || {
[01:08:07]    |  ____________^
[01:08:07] LL | |         || //~ ERROR
[01:08:07] LL | |         || //~ ERROR
[01:08:07] LL | |         *x = 1;});
[01:08:07] 
[01:08:07] error: aborting due to 12 previous errors
[01:08:07] 
[01:08:07] Some errors have detailed explanations: E0594, E0596.
---
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/constant-thread-locals-issue-47053/constant-thread-locals-issue-47053.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args nll/constant-thread-locals-issue-47053.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/constant-thread-locals-issue-47053.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/constant-thread-locals-issue-47053/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/constant-thread-locals-issue-47053/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
---
[01:08:07] 12 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/generator-upvar-mutability/generator-upvar-mutability.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args nll/generator-upvar-mutability.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/generator-upvar-mutability.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/generator-upvar-mutability/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/generator-upvar-mutability/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `x`, as it is not declared as mutable
[01:08:07]    |
[01:08:07] LL |     let x = 0;
[01:08:07]    |         - help: consider changing this to be mutable: `mut x`
[01:08:07] LL |     move || {
---
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-47388/issue-47388.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args nll/issue-47388.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-47388.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-47388/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-47388/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `fancy_ref.num` which is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     let fancy_ref = &(&mut fancy);
[01:08:07]    |                     ------------- help: consider changing this to be a mutable reference: `&mut (&mut fancy)`
[01:08:07] LL |     fancy_ref.num = 6; //~ ERROR E0594
[01:08:07]    |     ^^^^^^^^^^^^^^^^^ `fancy_ref` is a `&` reference, so the data it refers to cannot be written
[01:08:07] error: aborting due to previous error
[01:08:07] 
[01:08:07] For more information about this error, try `rustc --explain E0594`.
[01:08:07] 
---
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-51244/issue-51244.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args nll/issue-51244.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-51244.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-51244/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-51244/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `*my_ref` which is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     let ref my_ref @ _ = 0;
[01:08:07]    |         -------------- help: consider changing this to be a mutable reference: `ref mut my_ref @ _`
[01:08:07] LL |     *my_ref = 0;
[01:08:07]    |     ^^^^^^^^^^^ `my_ref` is a `&` reference, so the data it refers to cannot be written
[01:08:07] error: aborting due to previous error
[01:08:07] 
[01:08:07] For more information about this error, try `rustc --explain E0594`.
[01:08:07] 
---
[01:08:07] 
[01:08:07] 20 
[01:08:07] 21 error: aborting due to 2 previous errors
[01:08:07] 22 
[01:08:07] - For more information about this error, try `rustc --explain E0506`.
[01:08:07] + Some errors have detailed explanations: E0506, E0594.
[01:08:07] 24 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-57989/issue-57989.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args nll/issue-57989.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-57989.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-57989/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-57989/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `*x` which is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL | fn f(x: &i32) {
[01:08:07]    |         ---- help: consider changing this to be a mutable reference: `&mut i32`
[01:08:07] LL |     let g = &x;
[01:08:07] LL |     *x = 0;     //~ ERROR cannot assign to `*x` which is behind a `&` reference
[01:08:07]    |     ^^^^^^ `x` is a `&` reference, so the data it refers to cannot be written
[01:08:07] 
[01:08:07] error[E0506]: cannot assign to `*x` because it is borrowed
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     let g = &x;
[01:08:07]    |             -- borrow of `*x` occurs here
[01:08:07] LL |     *x = 0;     //~ ERROR cannot assign to `*x` which is behind a `&` reference
[01:08:07]    |     ^^^^^^ assignment to borrowed `*x` occurs here
[01:08:07] LL |                 //~| ERROR cannot assign to `*x` because it is borrowed
[01:08:07] LL |     g;
[01:08:07] 
[01:08:07] error: aborting due to 2 previous errors
[01:08:07] 
[01:08:07] Some errors have detailed explanations: E0506, E0594.
---
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/enum/enum.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args rfc-2005-default-binding-mode/enum.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/enum/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/enum/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `*x` which is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     *x += 1; //~ ERROR cannot assign to `*x` which is behind a `&` reference
[01:08:07]    |     ^^^^^^^ `x` is a `&` reference, so the data it refers to cannot be written
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `*x` which is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         *x += 1; //~ ERROR cannot assign to `*x` which is behind a `&` reference
[01:08:07]    |         ^^^^^^^ `x` is a `&` reference, so the data it refers to cannot be written
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `*x` which is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         *x += 1; //~ ERROR cannot assign to `*x` which is behind a `&` reference
[01:08:07]    |         ^^^^^^^ `x` is a `&` reference, so the data it refers to cannot be written
[01:08:07] error: aborting due to 3 previous errors
[01:08:07] 
[01:08:07] For more information about this error, try `rustc --explain E0594`.
[01:08:07] 
---
[01:08:07] 21 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/explicit-mut/explicit-mut.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args rfc-2005-default-binding-mode/explicit-mut.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/explicit-mut/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/explicit-mut/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `*n` which is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |             *n += 1; //~ ERROR cannot assign to `*n` which is behind a `&` reference
[01:08:07]    |             ^^^^^^^ `n` is a `&` reference, so the data it refers to cannot be written
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `*n` which is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |             *n += 1; //~ ERROR cannot assign to `*n` which is behind a `&` reference
[01:08:07]    |             ^^^^^^^ `n` is a `&` reference, so the data it refers to cannot be written
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `*n` which is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |             *n += 1; //~ ERROR cannot assign to `*n` which is behind a `&` reference
[01:08:07]    |             ^^^^^^^ `n` is a `&` reference, so the data it refers to cannot be written
[01:08:07] error: aborting due to 3 previous errors
[01:08:07] 
[01:08:07] For more information about this error, try `rustc --explain E0594`.
[01:08:07] 
---
[01:08:07] 38 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-ref-mut/suggest-ref-mut.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args suggestions/suggest-ref-mut.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/suggest-ref-mut.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-ref-mut/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-ref-mut/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `self.0` which is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |     fn zap(&self) {
[01:08:07]    |            ----- help: consider changing this to be a mutable reference: `&mut self`
[01:08:07] LL |         self.0 = 32;
[01:08:07] LL |         self.0 = 32;
[01:08:07]    |         ^^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be written
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `*foo` which is behind a `&` reference
[01:08:07]    |
[01:08:07] LL |     let ref foo = 16;
[01:08:07]    |         ------- help: consider changing this to be a mutable reference: `ref mut foo`
[01:08:07] ...
[01:08:07] ...
[01:08:07] LL |     *foo = 32;
[01:08:07]    |     ^^^^^^^^^ `foo` is a `&` reference, so the data it refers to cannot be written
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `*bar` which is behind a `&` reference
[01:08:07]    |
[01:08:07] LL |     if let Some(ref bar) = Some(16) {
[01:08:07]    |                 ------- help: consider changing this to be a mutable reference: `ref mut bar`
[01:08:07] ...
[01:08:07] ...
[01:08:07] LL |         *bar = 32;
[01:08:07]    |         ^^^^^^^^^ `bar` is a `&` reference, so the data it refers to cannot be written
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `*quo` which is behind a `&` reference
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         ref quo => { *quo = 32; },
[01:08:07]    |         -------      ^^^^^^^^^ `quo` is a `&` reference, so the data it refers to cannot be written
[01:08:07]    |         |
[01:08:07]    |         help: consider changing this to be a mutable reference: `ref mut quo`
[01:08:07] error: aborting due to 4 previous errors
[01:08:07] 
[01:08:07] For more information about this error, try `rustc --explain E0594`.
[01:08:07] 
---
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thread-local-mutation/thread-local-mutation.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args thread-local-mutation.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/thread-local-mutation.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thread-local-mutation/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thread-local-mutation/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to immutable static item `S`
[01:08:07]   --> /checkout/src/test/ui/thread-local-mutation.rs:11:5
[01:08:07]    |
[01:08:07] LL |     S = "after"; //~ ERROR cannot assign to immutable
[01:08:07] 
[01:08:07] error: aborting due to previous error
[01:08:07] 
[01:08:07] For more information about this error, try `rustc --explain E0594`.
---
[01:08:07] 
[01:08:07] 71 
[01:08:07] 72 error: aborting due to 8 previous errors
[01:08:07] 73 
[01:08:07] - For more information about this error, try `rustc --explain E0596`.
[01:08:07] + Some errors have detailed explanations: E0594, E0596.
[01:08:07] 75 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closure-immutable-capture/unboxed-closure-immutable-capture.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args unboxed-closures/unboxed-closure-immutable-capture.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unboxed-closures/unboxed-closure-immutable-capture.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closure-immutable-capture/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closure-immutable-capture/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `x`, as it is not declared as mutable
[01:08:07]    |
[01:08:07] LL |     let x = 0;
[01:08:07]    |         - help: consider changing this to be mutable: `mut x`
[01:08:07] LL |     move || x = 1; //~ ERROR cannot assign
[01:08:07] LL |     move || x = 1; //~ ERROR cannot assign
[01:08:07]    |             ^^^^^ cannot assign
[01:08:07] 
[01:08:07] error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
[01:08:07]   --> /checkout/src/test/ui/unboxed-closures/unboxed-closure-immutable-capture.rs:10:17
[01:08:07]    |
[01:08:07] LL |     let x = 0;
[01:08:07]    |         - help: consider changing this to be mutable: `mut x`
[01:08:07] LL |     move || x = 1; //~ ERROR cannot assign
[01:08:07] LL |     move || set(&mut x); //~ ERROR cannot borrow
[01:08:07]    |                 ^^^^^^ cannot borrow as mutable
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `x`, as it is not declared as mutable
[01:08:07]    |
[01:08:07] LL |     let x = 0;
[01:08:07]    |         - help: consider changing this to be mutable: `mut x`
[01:08:07] ...
---
[01:08:07]    |
[01:08:07] LL |     let x = 0;
[01:08:07]    |         - help: consider changing this to be mutable: `mut x`
[01:08:07] ...
[01:08:07] LL |     move || set(&mut x); //~ ERROR cannot borrow
[01:08:07]    |                 ^^^^^^ cannot borrow as mutable
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `x`, as it is not declared as mutable
[01:08:07]    |
[01:08:07] LL |     let x = 0;
[01:08:07]    |         - help: consider changing this to be mutable: `mut x`
[01:08:07] ...
---
[01:08:07]    |
[01:08:07] LL |     let x = 0;
[01:08:07]    |         - help: consider changing this to be mutable: `mut x`
[01:08:07] ...
[01:08:07] LL |     || set(&mut x); //~ ERROR cannot borrow
[01:08:07]    |            ^^^^^^ cannot borrow as mutable
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `x`, as it is not declared as mutable
[01:08:07]    |
[01:08:07] LL |     let x = 0;
[01:08:07]    |         - help: consider changing this to be mutable: `mut x`
[01:08:07] ...
---
[01:08:07]    |
[01:08:07] LL |     let x = 0;
[01:08:07]    |         - help: consider changing this to be mutable: `mut x`
[01:08:07] ...
[01:08:07] LL |     || set(&mut x); //~ ERROR cannot borrow
[01:08:07]    |            ^^^^^^ cannot borrow as mutable
[01:08:07] error: aborting due to 8 previous errors
[01:08:07] 
[01:08:07] Some errors have detailed explanations: E0594, E0596.
[01:08:07] For more information about an error, try `rustc --explain E0594`.
---
[01:08:07] 19 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-mutated-upvar-from-fn-closure/unboxed-closures-mutated-upvar-from-fn-closure.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args unboxed-closures/unboxed-closures-mutated-upvar-from-fn-closure.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unboxed-closures/unboxed-closures-mutated-upvar-from-fn-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-mutated-upvar-from-fn-closure/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-mutated-upvar-from-fn-closure/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `counter`, as it is a captured variable in a `Fn` closure
[01:08:07]    |
[01:08:07] LL |         counter += 1;
[01:08:07]    |         ^^^^^^^^^^^^ cannot assign
[01:08:07]    |
[01:08:07]    |
[01:08:07] help: consider changing this to accept closures that implement `FnMut`
[01:08:07]    |
[01:08:07] LL |       call(|| {
[01:08:07]    |  __________^
[01:08:07] LL | |         counter += 1;
[01:08:07] LL | |         counter += 1;
[01:08:07] LL | |         //~^ ERROR cannot assign to `counter`
[01:08:07] LL | |     });
[01:08:07] 
[01:08:07] error: aborting due to previous error
[01:08:07] 
[01:08:07] For more information about this error, try `rustc --explain E0594`.
---
[01:08:07] 45 
[01:08:07] 
[01:08:07] 
[01:08:07] The actual stderr differed from the expected stderr.
[01:08:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-mutate-upvar/unboxed-closures-mutate-upvar.stderr
[01:08:07] To update references, rerun the tests and pass the `--bless` flag
[01:08:07] To only update this specific test, also pass `--test-args unboxed-closures/unboxed-closures-mutate-upvar.rs`
[01:08:07] error: 1 errors occurred comparing output.
[01:08:07] status: exit code: 1
[01:08:07] status: exit code: 1
[01:08:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unboxed-closures/unboxed-closures-mutate-upvar.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-mutate-upvar/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-mutate-upvar/auxiliary" "-A" "unused"
[01:08:07] ------------------------------------------
[01:08:07] 
[01:08:07] ------------------------------------------
[01:08:07] stderr:
[01:08:07] stderr:
[01:08:07] ------------------------------------------
[01:08:07] error[E0594]: cannot assign to `n`, as it is not declared as mutable
[01:08:07]    |
[01:08:07] LL |     let n = 0;
[01:08:07]    |         - help: consider changing this to be mutable: `mut n`
[01:08:07]    |         - help: consider changing this to be mutable: `mut n`
[01:08:07] LL |     let mut f = to_fn_mut(|| {
[01:08:07] LL |         n += 1; //~ ERROR cannot assign to `n`, as it is not declared as mutable
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `n`, as it is not declared as mutable
[01:08:07]    |
[01:08:07] LL |     let n = 0;
[01:08:07]    |         - help: consider changing this to be mutable: `mut n`
[01:08:07] ...
[01:08:07] ...
[01:08:07] LL |         n += 1; //~ ERROR cannot assign
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `n`, as it is not declared as mutable
[01:08:07]    |
[01:08:07] LL |     let n = 0;
[01:08:07]    |         - help: consider changing this to be mutable: `mut n`
[01:08:07]    |         - help: consider changing this to be mutable: `mut n`
[01:08:07] LL |     let mut f = to_fn(move || {
[01:08:07] LL |         n += 1; //~ ERROR cannot assign
[01:08:07] 
[01:08:07] 
[01:08:07] error[E0594]: cannot assign to `n`, as it is a captured variable in a `Fn` closure
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |         n += 1; //~ ERROR cannot assign
[01:08:07]    |
[01:08:07]    |
[01:08:07] help: consider changing this to accept closures that implement `FnMut`
[01:08:07]    |
[01:08:07]    |
[01:08:07] LL |       let mut f = to_fn(move || {
[01:08:07]    |  _______________________^
[01:08:07] LL | |         n += 1; //~ ERROR cannot assign
[01:08:07] LL | |     });
[01:08:07] 
[01:08:07] error: aborting due to 4 previous errors
[01:08:07] 
[01:08:07] For more information about this error, try `rustc --explain E0594`.
---
[01:08:07] test result: FAILED. 5394 passed; 44 failed; 21 ignored; 0 measured; 0 filtered out
[01:08:07] 
[01:08:07] 
[01:08:07] 
[01:08:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:08:07] 
[01:08:07] 
[01:08:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:08:07] Build completed unsuccessfully in 0:04:25
---
travis_time:end:089f30d0:start=1556016734727247637,finish=1556016734734155590,duration=6907953
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:091071c0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0eeedb60
$ dmesg | grep -i kill

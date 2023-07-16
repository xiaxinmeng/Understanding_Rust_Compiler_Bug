plain
travis_time:end:00ce012e:start=1559170912974055375,finish=1559170913759644398,duration=785589023
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
[01:07:36] 
[01:07:36] running 5600 tests
[01:07:39] ...F................................................................................................ 100/5600
[01:07:45] .................................................................................................... 200/5600
[01:07:49] .......................................................................F....................F..F.... 300/5600
[01:07:52] .F.................F..............................................................F................. 400/5600
[01:07:55] ...................................................................................................i 500/5600
[01:08:02] .................................................................................................... 700/5600
[01:08:07] .................................................................................................... 800/5600
[01:08:11] .....................................................................................i...........i.. 900/5600
[01:08:15] .................................................................................................... 1000/5600
---
[01:08:43] .................................................................................................... 1900/5600
[01:08:46] .................................................................................................... 2000/5600
[01:08:50] ...................................................................................i................ 2100/5600
[01:08:53] .................................................................................................... 2200/5600
[01:08:57] ...................................F................................................................ 2300/5600
[01:09:06] .................................................................................................... 2500/5600
[01:09:09] .................................................................................................... 2600/5600
[01:09:09] .................................................................................................... 2600/5600
[01:09:13] ......................................................................................FF............ 2700/5600
[01:09:21] .................................................................................................... 2900/5600
[01:09:26] .................................................................................................... 3000/5600
[01:09:29] .................................................................................................... 3100/5600
[01:09:32] .................................................................................................... 3200/5600
[01:09:32] .................................................................................................... 3200/5600
[01:09:36] .................................................................................................... 3300/5600
[01:09:40] ........................i........................................................................... 3400/5600
[01:09:43] ..................................................................................................ii 3500/5600
[01:09:46] ...i..ii.......................................................F.................................... 3600/5600
[01:09:51] .....................................................................................F.............. 3700/5600
[01:09:57] ...ii............................................................................................... 3900/5600
[01:09:59] ........................i........................................................................... 4000/5600
[01:10:01] ........................................................................................i........... 4100/5600
[01:10:03] .................................................................................................... 4200/5600
---
[01:10:32] .................................................................................................... 4700/5600
[01:10:38] .................................................................................................... 4800/5600
[01:10:44] .................................................................................................... 4900/5600
[01:10:47] .................................................................................................... 5000/5600
[01:10:51] .....................F...............................................F.............................. 5100/5600
[01:10:59] .................................................................................................... 5300/5600
[01:11:02] .................................................................................................... 5400/5600
[01:11:05] .................................................................................................... 5500/5600
[01:11:08] ......................................i............................................................. 5600/5600
[01:11:08] ......................................i............................................................. 5600/5600
[01:11:08] 
[01:11:08] failures:
[01:11:08] 
[01:11:08] ---- [ui] ui/access-mode-in-closures.rs stdout ----
[01:11:08] diff of stderr:
[01:11:08] 
[01:11:08] 5    |               ^^     - data moved here
[01:11:08] 7    |               cannot move out of borrowed content
[01:11:08] 7    |               cannot move out of borrowed content
[01:11:08] -    |               help: consider removing the `*`: `s`
[01:11:08] 9    |
[01:11:08] 10 note: move occurs because `v` has type `std::vec::Vec<isize>`, which does not implement the `Copy` trait
[01:11:08] 
[01:11:08] 
[01:11:08] The actual stderr differed from the expected stderr.
[01:11:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/access-mode-in-closures/access-mode-in-closures.stderr
[01:11:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/access-mode-in-closures/access-mode-in-closures.stderr
[01:11:08] To update references, rerun the tests and pass the `--bless` flag
[01:11:08] To only update this specific test, also pass `--test-args access-mode-in-closures.rs`
[01:11:08] error: 1 errors occurred comparing output.
[01:11:08] status: exit code: 1
[01:11:08] status: exit code: 1
[01:11:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/access-mode-in-closures.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/access-mode-in-closures" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/access-mode-in-closures/auxiliary" "-A" "unused"
[01:11:08] ------------------------------------------
[01:11:08] 
[01:11:08] ------------------------------------------
[01:11:08] stderr:
[01:11:08] stderr:
[01:11:08] ------------------------------------------
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/access-mode-in-closures.rs:8:15
[01:11:08]    |
[01:11:08] LL |         match *s { S(v) => v } //~ ERROR cannot move out
[01:11:08]    |               ^^     - data moved here
[01:11:08]    |               cannot move out of borrowed content
[01:11:08]    |
[01:11:08]    |
[01:11:08] note: move occurs because `v` has type `std::vec::Vec<isize>`, which does not implement the `Copy` trait
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |         match *s { S(v) => v } //~ ERROR cannot move out
[01:11:08] 
[01:11:08] error: aborting due to previous error
[01:11:08] 
[01:11:08] For more information about this error, try `rustc --explain E0507`.
---
[01:11:08] 3    |
[01:11:08] 4 LL |         let _b = *y;
[01:11:08] -    |                  ^^
[01:11:08] -    |                  |
[01:11:08] -    |                  cannot move out of borrowed content
[01:11:08] -    |                  help: consider removing the `*`: `y`
[01:11:08] +    |                  ^^ cannot move out of borrowed content
[01:11:08] 10 error: aborting due to previous error
[01:11:08] 11 
[01:11:08] 
[01:11:08] 
[01:11:08] 
[01:11:08] The actual stderr differed from the expected stderr.
[01:11:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-issue-2657-2/borrowck-issue-2657-2.stderr
[01:11:08] To update references, rerun the tests and pass the `--bless` flag
[01:11:08] To only update this specific test, also pass `--test-args borrowck/borrowck-issue-2657-2.rs`
[01:11:08] error: 1 errors occurred comparing output.
[01:11:08] status: exit code: 1
[01:11:08] status: exit code: 1
[01:11:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-issue-2657-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-issue-2657-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-issue-2657-2/auxiliary" "-A" "unused"
[01:11:08] ------------------------------------------
[01:11:08] 
[01:11:08] ------------------------------------------
[01:11:08] stderr:
---
[01:11:08] 3    |
[01:11:08] 4 LL |     match *f {
[01:11:08] -    |           ^^
[01:11:08] -    |           |
[01:11:08] -    |           cannot move out of borrowed content
[01:11:08] -    |           help: consider removing the `*`: `f`
[01:11:08] +    |           ^^ cannot move out of borrowed content
[01:11:08] 9 LL |
[01:11:08] 10 LL |         Foo::Foo1(num1,
[01:11:08] 11    |                   ---- data moved here
[01:11:08] 
[01:11:08] The actual stderr differed from the expected stderr.
[01:11:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-error-with-note/borrowck-move-error-with-note.stderr
[01:11:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-error-with-note/borrowck-move-error-with-note.stderr
[01:11:08] To update references, rerun the tests and pass the `--bless` flag
[01:11:08] To only update this specific test, also pass `--test-args borrowck/borrowck-move-error-with-note.rs`
[01:11:08] error: 1 errors occurred comparing output.
[01:11:08] status: exit code: 1
[01:11:08] status: exit code: 1
[01:11:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-move-error-with-note.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-error-with-note" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-error-with-note/auxiliary" "-A" "unused"
[01:11:08] ------------------------------------------
[01:11:08] 
[01:11:08] ------------------------------------------
[01:11:08] stderr:
[01:11:08] stderr:
[01:11:08] ------------------------------------------
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/borrowck/borrowck-move-error-with-note.rs:11:11
[01:11:08]    |
[01:11:08] LL |     match *f {             //~ ERROR cannot move out of
[01:11:08]    |           ^^ cannot move out of borrowed content
[01:11:08] LL |                            //~| cannot move out
[01:11:08] LL |         Foo::Foo1(num1,
[01:11:08]    |                   ---- data moved here
[01:11:08] LL |                   num2) => (),
[01:11:08]    |                   ---- ...and here
[01:11:08] LL |         Foo::Foo2(num) => (),
[01:11:08]    |                   --- ...and here
[01:11:08] note: move occurs because these variables have types that don't implement the `Copy` trait
[01:11:08]   --> /checkout/src/test/ui/borrowck/borrowck-move-error-with-note.rs:13:19
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |         Foo::Foo1(num1,
[01:11:08] LL |                   num2) => (),
[01:11:08]    |                   ^^^^
[01:11:08]    |                   ^^^^
[01:11:08] LL |         Foo::Foo2(num) => (),
[01:11:08] 
[01:11:08] error[E0509]: cannot move out of type `S`, which implements the `Drop` trait
[01:11:08]   --> /checkout/src/test/ui/borrowck/borrowck-move-error-with-note.rs:29:11
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     match (S {f: "foo".to_string(), g: "bar".to_string()}) {
[01:11:08]    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot move out of here
[01:11:08] ...
[01:11:08] LL |             f: _s,
[01:11:08]    |                -- data moved here
[01:11:08] LL |             g: _t
[01:11:08]    |                -- ...and here
[01:11:08] note: move occurs because these variables have types that don't implement the `Copy` trait
[01:11:08]   --> /checkout/src/test/ui/borrowck/borrowck-move-error-with-note.rs:33:16
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |             f: _s,
[01:11:08] LL |             g: _t
[01:11:08]    |                ^^
[01:11:08] 
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/borrowck/borrowck-move-error-with-note.rs:48:11
[01:11:08]    |
[01:11:08] LL |     match a.a {           //~ ERROR cannot move out of
[01:11:08]    |           |
[01:11:08]    |           cannot move out of borrowed content
[01:11:08]    |           help: consider borrowing here: `&a.a`
[01:11:08] LL |                           //~| cannot move out
[01:11:08] LL |                           //~| cannot move out
[01:11:08] LL |         n => {
[01:11:08]    |         - data moved here
[01:11:08]    |
[01:11:08] note: move occurs because `n` has type `std::boxed::Box<isize>`, which does not implement the `Copy` trait
[01:11:08]    |
[01:11:08] LL |         n => {
[01:11:08]    |         ^
[01:11:08] 
---
[01:11:08] 3    |
[01:11:08] 4 LL |     let y = *x;
[01:11:08] -    |             ^^
[01:11:08] -    |             |
[01:11:08] -    |             cannot move out of dereference of raw pointer
[01:11:08] -    |             help: consider removing the `*`: `x`
[01:11:08] +    |             ^^ cannot move out of dereference of raw pointer
[01:11:08] 10 error: aborting due to previous error
[01:11:08] 11 
[01:11:08] 
[01:11:08] 
[01:11:08] 
[01:11:08] The actual stderr differed from the expected stderr.
[01:11:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-from-unsafe-ptr/borrowck-move-from-unsafe-ptr.stderr
[01:11:08] To update references, rerun the tests and pass the `--bless` flag
[01:11:08] To only update this specific test, also pass `--test-args borrowck/borrowck-move-from-unsafe-ptr.rs`
[01:11:08] error: 1 errors occurred comparing output.
[01:11:08] status: exit code: 1
[01:11:08] status: exit code: 1
[01:11:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-move-from-unsafe-ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-from-unsafe-ptr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-from-unsafe-ptr/auxiliary" "-A" "unused"
[01:11:08] ------------------------------------------
[01:11:08] 
[01:11:08] ------------------------------------------
[01:11:08] stderr:
[01:11:08] stderr:
[01:11:08] ------------------------------------------
[01:11:08] error[E0507]: cannot move out of dereference of raw pointer
[01:11:08]   --> /checkout/src/test/ui/borrowck/borrowck-move-from-unsafe-ptr.rs:2:13
[01:11:08]    |
[01:11:08] LL |     let y = *x; //~ ERROR cannot move out of dereference of raw pointer
[01:11:08]    |             ^^ cannot move out of dereference of raw pointer
[01:11:08] error: aborting due to previous error
[01:11:08] 
[01:11:08] For more information about this error, try `rustc --explain E0507`.
[01:11:08] 
---
[01:11:08] diff of stderr:
[01:11:08] 
[01:11:08] 2   --> $DIR/borrowck-move-out-of-overloaded-deref.rs:4:14
[01:11:08] 3    |
[01:11:08] 4 LL |     let _x = *Rc::new("hi".to_string());
[01:11:08] -    |              |
[01:11:08] -    |              cannot move out of an `Rc`
[01:11:08] -    |              cannot move out of an `Rc`
[01:11:08] -    |              help: consider removing the `*`: `Rc::new("hi".to_string())`
[01:11:08] +    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot move out of an `Rc`
[01:11:08] 10 error: aborting due to previous error
[01:11:08] 11 
[01:11:08] 
[01:11:08] 
[01:11:08] 
[01:11:08] The actual stderr differed from the expected stderr.
[01:11:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-out-of-overloaded-deref/borrowck-move-out-of-overloaded-deref.stderr
[01:11:08] To update references, rerun the tests and pass the `--bless` flag
[01:11:08] To only update this specific test, also pass `--test-args borrowck/borrowck-move-out-of-overloaded-deref.rs`
[01:11:08] error: 1 errors occurred comparing output.
[01:11:08] status: exit code: 1
[01:11:08] status: exit code: 1
[01:11:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-move-out-of-overloaded-deref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-out-of-overloaded-deref" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-out-of-overloaded-deref/auxiliary" "-A" "unused"
[01:11:08] ------------------------------------------
[01:11:08] 
[01:11:08] ------------------------------------------
[01:11:08] stderr:
[01:11:08] stderr:
[01:11:08] ------------------------------------------
[01:11:08] error[E0507]: cannot move out of an `Rc`
[01:11:08]   --> /checkout/src/test/ui/borrowck/borrowck-move-out-of-overloaded-deref.rs:4:14
[01:11:08]    |
[01:11:08] LL |     let _x = *Rc::new("hi".to_string());
[01:11:08]    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot move out of an `Rc`
[01:11:08] error: aborting due to previous error
[01:11:08] 
[01:11:08] For more information about this error, try `rustc --explain E0507`.
[01:11:08] 
---
[01:11:08] diff of stderr:
[01:11:08] 
[01:11:08] 2   --> $DIR/borrowck-overloaded-index-move-from-vec.rs:20:15
[01:11:08] 3    |
[01:11:08] 4 LL |     let bad = v[0];
[01:11:08] -    |               |
[01:11:08] -    |               cannot move out of borrowed content
[01:11:08] -    |               help: consider borrowing here: `&v[0]`
[01:11:08] +    |               ^^^^ cannot move out of borrowed content
[01:11:08] +    |               ^^^^ cannot move out of borrowed content
[01:11:08] 9 
[01:11:08] 10 error: aborting due to previous error
[01:11:08] 11 
[01:11:08] 
[01:11:08] 
[01:11:08] The actual stderr differed from the expected stderr.
[01:11:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-overloaded-index-move-from-vec/borrowck-overloaded-index-move-from-vec.stderr
[01:11:08] To update references, rerun the tests and pass the `--bless` flag
[01:11:08] To only update this specific test, also pass `--test-args borrowck/borrowck-overloaded-index-move-from-vec.rs`
[01:11:08] error: 1 errors occurred comparing output.
[01:11:08] status: exit code: 1
[01:11:08] status: exit code: 1
[01:11:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-overloaded-index-move-from-vec.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-overloaded-index-move-from-vec" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-overloaded-index-move-from-vec/auxiliary" "-A" "unused"
[01:11:08] ------------------------------------------
[01:11:08] 
[01:11:08] ------------------------------------------
[01:11:08] stderr:
[01:11:08] stderr:
[01:11:08] ------------------------------------------
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/borrowck/borrowck-overloaded-index-move-from-vec.rs:20:15
[01:11:08]    |
[01:11:08] LL |     let bad = v[0];
[01:11:08]    |               ^^^^ cannot move out of borrowed content
[01:11:08] error: aborting due to previous error
[01:11:08] 
[01:11:08] For more information about this error, try `rustc --explain E0507`.
[01:11:08] 
---
[01:11:08] 3    |
[01:11:08] 4 LL |             *array
[01:11:08] -    |             ^^^^^^
[01:11:08] -    |             |
[01:11:08] -    |             cannot move out of borrowed content
[01:11:08] -    |             help: consider removing the `*`: `array`
[01:11:08] +    |             ^^^^^^ cannot move out of borrowed content
[01:11:08] 10 error: aborting due to previous error
[01:11:08] 11 
[01:11:08] 
[01:11:08] 
[01:11:08] 
[01:11:08] The actual stderr differed from the expected stderr.
[01:11:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-54597-reject-move-out-of-borrow-via-pat/issue-54597-reject-move-out-of-borrow-via-pat.stderr
[01:11:08] To update references, rerun the tests and pass the `--bless` flag
[01:11:08] To only update this specific test, also pass `--test-args borrowck/issue-54597-reject-move-out-of-borrow-via-pat.rs`
[01:11:08] error: 1 errors occurred comparing output.
[01:11:08] status: exit code: 1
[01:11:08] status: exit code: 1
[01:11:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-54597-reject-move-out-of-borrow-via-pat.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-54597-reject-move-out-of-borrow-via-pat" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-54597-reject-move-out-of-borrow-via-pat/auxiliary" "-A" "unused"
[01:11:08] ------------------------------------------
[01:11:08] 
[01:11:08] ------------------------------------------
[01:11:08] stderr:
[01:11:08] stderr:
[01:11:08] ------------------------------------------
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/borrowck/issue-54597-reject-move-out-of-borrow-via-pat.rs:14:13
[01:11:08]    |
[01:11:08] LL |             *array //~ ERROR cannot move out of borrowed content
[01:11:08]    |             ^^^^^^ cannot move out of borrowed content
[01:11:08] error: aborting due to previous error
[01:11:08] 
[01:11:08] For more information about this error, try `rustc --explain E0507`.
[01:11:08] 
---
[01:11:08] diff of stderr:
[01:11:08] 
[01:11:08] 2   --> $DIR/issue-20801.rs:26:22
[01:11:08] 3    |
[01:11:08] 4 LL |     let a = unsafe { *mut_ref() };
[01:11:08] -    |                      |
[01:11:08] -    |                      cannot move out of borrowed content
[01:11:08] -    |                      cannot move out of borrowed content
[01:11:08] -    |                      help: consider removing the `*`: `mut_ref()`
[01:11:08] +    |                      ^^^^^^^^^^ cannot move out of borrowed content
[01:11:08] 10 error[E0507]: cannot move out of borrowed content
[01:11:08] 11   --> $DIR/issue-20801.rs:29:22
[01:11:08] 
[01:11:08] 12    |
[01:11:08] 12    |
[01:11:08] 13 LL |     let b = unsafe { *imm_ref() };
[01:11:08] -    |                      |
[01:11:08] -    |                      cannot move out of borrowed content
[01:11:08] -    |                      cannot move out of borrowed content
[01:11:08] -    |                      help: consider removing the `*`: `imm_ref()`
[01:11:08] +    |                      ^^^^^^^^^^ cannot move out of borrowed content
[01:11:08] 19 error[E0507]: cannot move out of dereference of raw pointer
[01:11:08] 20   --> $DIR/issue-20801.rs:32:22
[01:11:08] 
[01:11:08] 21    |
[01:11:08] 21    |
[01:11:08] 22 LL |     let c = unsafe { *mut_ptr() };
[01:11:08] -    |                      |
[01:11:08] -    |                      cannot move out of dereference of raw pointer
[01:11:08] -    |                      cannot move out of dereference of raw pointer
[01:11:08] -    |                      help: consider removing the `*`: `mut_ptr()`
[01:11:08] +    |                      ^^^^^^^^^^ cannot move out of dereference of raw pointer
[01:11:08] 28 error[E0507]: cannot move out of dereference of raw pointer
[01:11:08] 29   --> $DIR/issue-20801.rs:35:22
[01:11:08] 
[01:11:08] 30    |
[01:11:08] 30    |
[01:11:08] 31 LL |     let d = unsafe { *const_ptr() };
[01:11:08] -    |                      |
[01:11:08] -    |                      cannot move out of dereference of raw pointer
[01:11:08] -    |                      cannot move out of dereference of raw pointer
[01:11:08] -    |                      help: consider removing the `*`: `const_ptr()`
[01:11:08] +    |                      ^^^^^^^^^^^^ cannot move out of dereference of raw pointer
[01:11:08] 37 error: aborting due to 4 previous errors
[01:11:08] 38 
[01:11:08] 
[01:11:08] 
[01:11:08] 
[01:11:08] The actual stderr differed from the expected stderr.
[01:11:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20801/issue-20801.stderr
[01:11:08] To update references, rerun the tests and pass the `--bless` flag
[01:11:08] To only update this specific test, also pass `--test-args issues/issue-20801.rs`
[01:11:08] error: 1 errors occurred comparing output.
[01:11:08] status: exit code: 1
[01:11:08] status: exit code: 1
[01:11:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-20801.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20801" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20801/auxiliary" "-A" "unused"
[01:11:08] ------------------------------------------
[01:11:08] 
[01:11:08] ------------------------------------------
[01:11:08] stderr:
[01:11:08] stderr:
[01:11:08] ------------------------------------------
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/issues/issue-20801.rs:26:22
[01:11:08]    |
[01:11:08] LL |     let a = unsafe { *mut_ref() };
[01:11:08]    |                      ^^^^^^^^^^ cannot move out of borrowed content
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/issues/issue-20801.rs:29:22
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     let b = unsafe { *imm_ref() };
[01:11:08]    |                      ^^^^^^^^^^ cannot move out of borrowed content
[01:11:08] error[E0507]: cannot move out of dereference of raw pointer
[01:11:08]   --> /checkout/src/test/ui/issues/issue-20801.rs:32:22
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     let c = unsafe { *mut_ptr() };
[01:11:08]    |                      ^^^^^^^^^^ cannot move out of dereference of raw pointer
[01:11:08] error[E0507]: cannot move out of dereference of raw pointer
[01:11:08]   --> /checkout/src/test/ui/issues/issue-20801.rs:35:22
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     let d = unsafe { *const_ptr() };
[01:11:08]    |                      ^^^^^^^^^^^^ cannot move out of dereference of raw pointer
[01:11:08] error: aborting due to 4 previous errors
[01:11:08] 
[01:11:08] For more information about this error, try `rustc --explain E0507`.
[01:11:08] 
---
[01:11:08] diff of stderr:
[01:11:08] 
[01:11:08] 2   --> $DIR/issue-40402-1.rs:9:13
[01:11:08] 3    |
[01:11:08] 4 LL |     let e = f.v[0];
[01:11:08] -    |             |
[01:11:08] -    |             cannot move out of borrowed content
[01:11:08] -    |             cannot move out of borrowed content
[01:11:08] -    |             help: consider borrowing here: `&f.v[0]`
[01:11:08] +    |             ^^^^^^ cannot move out of borrowed content
[01:11:08] 10 error: aborting due to previous error
[01:11:08] 11 
[01:11:08] 
[01:11:08] 
[01:11:08] 
[01:11:08] The actual stderr differed from the expected stderr.
[01:11:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40402-ref-hints/issue-40402-1/issue-40402-1.stderr
[01:11:08] To update references, rerun the tests and pass the `--bless` flag
[01:11:08] To only update this specific test, also pass `--test-args issues/issue-40402-ref-hints/issue-40402-1.rs`
[01:11:08] error: 1 errors occurred comparing output.
[01:11:08] status: exit code: 1
[01:11:08] status: exit code: 1
[01:11:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-40402-ref-hints/issue-40402-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40402-ref-hints/issue-40402-1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40402-ref-hints/issue-40402-1/auxiliary" "-A" "unused"
[01:11:08] ------------------------------------------
[01:11:08] 
[01:11:08] ------------------------------------------
[01:11:08] stderr:
[01:11:08] stderr:
[01:11:08] ------------------------------------------
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/issues/issue-40402-ref-hints/issue-40402-1.rs:9:13
[01:11:08]    |
[01:11:08] LL |     let e = f.v[0]; //~ ERROR cannot move out of borrowed content
[01:11:08]    |             ^^^^^^ cannot move out of borrowed content
[01:11:08] error: aborting due to previous error
[01:11:08] 
[01:11:08] For more information about this error, try `rustc --explain E0507`.
[01:11:08] 
---
[01:11:08] -    |          |  |    cannot move out of borrowed content
[01:11:08] -    |          |  |    help: consider borrowing here: `&x[0]`
[01:11:08] +    |          -  -    ^^^^ cannot move out of borrowed content
[01:11:08] +    |          |  |
[01:11:08] 9    |          |  ...and here
[01:11:08] 10    |          data moved here
[01:11:08] 
[01:11:08] 
[01:11:08] The actual stderr differed from the expected stderr.
[01:11:08] The actual stderr differed from the expected stderr.
[01:11:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40402-ref-hints/issue-40402-2/issue-40402-2.stderr
[01:11:08] To update references, rerun the tests and pass the `--bless` flag
[01:11:08] To only update this specific test, also pass `--test-args issues/issue-40402-ref-hints/issue-40402-2.rs`
[01:11:08] error: 1 errors occurred comparing output.
[01:11:08] status: exit code: 1
[01:11:08] status: exit code: 1
[01:11:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-40402-ref-hints/issue-40402-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40402-ref-hints/issue-40402-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40402-ref-hints/issue-40402-2/auxiliary" "-A" "unused"
[01:11:08] ------------------------------------------
[01:11:08] 
[01:11:08] ------------------------------------------
[01:11:08] stderr:
[01:11:08] stderr:
[01:11:08] ------------------------------------------
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/issues/issue-40402-ref-hints/issue-40402-2.rs:5:18
[01:11:08]    |
[01:11:08] LL |     let (a, b) = x[0]; //~ ERROR cannot move out of borrowed content
[01:11:08]    |          -  -    ^^^^ cannot move out of borrowed content
[01:11:08]    |          |  |
[01:11:08]    |          |  ...and here
[01:11:08]    |          data moved here
[01:11:08] note: move occurs because these variables have types that don't implement the `Copy` trait
[01:11:08]   --> /checkout/src/test/ui/issues/issue-40402-ref-hints/issue-40402-2.rs:5:10
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     let (a, b) = x[0]; //~ ERROR cannot move out of borrowed content
[01:11:08]    |          ^  ^
[01:11:08] error: aborting due to previous error
[01:11:08] 
[01:11:08] For more information about this error, try `rustc --explain E0507`.
[01:11:08] 
---
[01:11:08] diff of stderr:
[01:11:08] 
[01:11:08] 2   --> $DIR/cannot-move-block-spans.rs:5:15
[01:11:08] 3    |
[01:11:08] 4 LL |     let x = { *r };
[01:11:08] -    |               |
[01:11:08] -    |               cannot move out of borrowed content
[01:11:08] -    |               cannot move out of borrowed content
[01:11:08] -    |               help: consider removing the `*`: `r`
[01:11:08] +    |               ^^ cannot move out of borrowed content
[01:11:08] 10 error[E0507]: cannot move out of borrowed content
[01:11:08] 11   --> $DIR/cannot-move-block-spans.rs:6:22
[01:11:08] 
[01:11:08] 12    |
[01:11:08] 12    |
[01:11:08] 13 LL |     let y = unsafe { *r };
[01:11:08] -    |                      ^^
[01:11:08] -    |                      |
[01:11:08] -    |                      cannot move out of borrowed content
[01:11:08] -    |                      help: consider removing the `*`: `r`
[01:11:08] +    |                      ^^ cannot move out of borrowed content
[01:11:08] 19 error[E0507]: cannot move out of borrowed content
[01:11:08] 20   --> $DIR/cannot-move-block-spans.rs:7:26
[01:11:08] 
[01:11:08] 21    |
[01:11:08] 21    |
[01:11:08] 22 LL |     let z = loop { break *r; };
[01:11:08] -    |                          |
[01:11:08] -    |                          cannot move out of borrowed content
[01:11:08] -    |                          cannot move out of borrowed content
[01:11:08] -    |                          help: consider removing the `*`: `r`
[01:11:08] +    |                          ^^ cannot move out of borrowed content
[01:11:08] 27 
[01:11:08] 28 error[E0508]: cannot move out of type `[std::string::String; 2]`, a non-copy array
[01:11:08] 
[01:11:08] 56   --> $DIR/cannot-move-block-spans.rs:17:38
[01:11:08] 57    |
[01:11:08] 57    |
[01:11:08] 58 LL |     let x = { let mut u = 0; u += 1; *r };
[01:11:08] -    |                                      |
[01:11:08] -    |                                      cannot move out of borrowed content
[01:11:08] -    |                                      cannot move out of borrowed content
[01:11:08] -    |                                      help: consider removing the `*`: `r`
[01:11:08] +    |                                      ^^ cannot move out of borrowed content
[01:11:08] 64 error[E0507]: cannot move out of borrowed content
[01:11:08] 65   --> $DIR/cannot-move-block-spans.rs:18:45
[01:11:08] 
[01:11:08] 66    |
[01:11:08] 66    |
[01:11:08] 67 LL |     let y = unsafe { let mut u = 0; u += 1; *r };
[01:11:08] -    |                                             |
[01:11:08] -    |                                             cannot move out of borrowed content
[01:11:08] -    |                                             cannot move out of borrowed content
[01:11:08] -    |                                             help: consider removing the `*`: `r`
[01:11:08] +    |                                             ^^ cannot move out of borrowed content
[01:11:08] 73 error[E0507]: cannot move out of borrowed content
[01:11:08] 74   --> $DIR/cannot-move-block-spans.rs:19:49
[01:11:08] 
[01:11:08] 75    |
[01:11:08] 75    |
[01:11:08] 76 LL |     let z = loop { let mut u = 0; u += 1; break *r; u += 2; };
[01:11:08] -    |                                                 |
[01:11:08] -    |                                                 cannot move out of borrowed content
[01:11:08] -    |                                                 cannot move out of borrowed content
[01:11:08] -    |                                                 help: consider removing the `*`: `r`
[01:11:08] +    |                                                 ^^ cannot move out of borrowed content
[01:11:08] 82 error: aborting due to 9 previous errors
[01:11:08] 83 
[01:11:08] 
[01:11:08] 
[01:11:08] 
[01:11:08] The actual stderr differed from the expected stderr.
[01:11:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/cannot-move-block-spans/cannot-move-block-spans.stderr
[01:11:08] To update references, rerun the tests and pass the `--bless` flag
[01:11:08] To only update this specific test, also pass `--test-args nll/cannot-move-block-spans.rs`
[01:11:08] error: 1 errors occurred comparing output.
[01:11:08] status: exit code: 1
[01:11:08] status: exit code: 1
[01:11:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/cannot-move-block-spans.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/cannot-move-block-spans" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/cannot-move-block-spans/auxiliary" "-A" "unused"
[01:11:08] ------------------------------------------
[01:11:08] 
[01:11:08] ------------------------------------------
[01:11:08] stderr:
[01:11:08] stderr:
[01:11:08] ------------------------------------------
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/nll/cannot-move-block-spans.rs:5:15
[01:11:08]    |
[01:11:08] LL |     let x = { *r }; //~ ERROR
[01:11:08]    |               ^^ cannot move out of borrowed content
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/nll/cannot-move-block-spans.rs:6:22
[01:11:08]    |
[01:11:08] LL |     let y = unsafe { *r }; //~ ERROR
[01:11:08] LL |     let y = unsafe { *r }; //~ ERROR
[01:11:08]    |                      ^^ cannot move out of borrowed content
[01:11:08] 
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/nll/cannot-move-block-spans.rs:7:26
[01:11:08]    |
[01:11:08] LL |     let z = loop { break *r; }; //~ ERROR
[01:11:08]    |                          ^^ cannot move out of borrowed content
[01:11:08] 
[01:11:08] error[E0508]: cannot move out of type `[std::string::String; 2]`, a non-copy array
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     let x = { arr[0] }; //~ ERROR
[01:11:08]    |               |
[01:11:08]    |               cannot move out of here
[01:11:08]    |               help: consider borrowing here: `&arr[0]`
[01:11:08] 
[01:11:08] 
[01:11:08] error[E0508]: cannot move out of type `[std::string::String; 2]`, a non-copy array
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     let y = unsafe { arr[0] }; //~ ERROR
[01:11:08]    |                      |
[01:11:08]    |                      cannot move out of here
[01:11:08]    |                      help: consider borrowing here: `&arr[0]`
[01:11:08] 
[01:11:08] 
[01:11:08] error[E0508]: cannot move out of type `[std::string::String; 2]`, a non-copy array
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     let z = loop { break arr[0]; }; //~ ERROR
[01:11:08]    |                          |
[01:11:08]    |                          cannot move out of here
[01:11:08]    |                          help: consider borrowing here: `&arr[0]`
[01:11:08] 
[01:11:08] 
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/nll/cannot-move-block-spans.rs:17:38
[01:11:08]    |
[01:11:08] LL |     let x = { let mut u = 0; u += 1; *r }; //~ ERROR
[01:11:08]    |                                      ^^ cannot move out of borrowed content
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/nll/cannot-move-block-spans.rs:18:45
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     let y = unsafe { let mut u = 0; u += 1; *r }; //~ ERROR
[01:11:08]    |                                             ^^ cannot move out of borrowed content
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/nll/cannot-move-block-spans.rs:19:49
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     let z = loop { let mut u = 0; u += 1; break *r; u += 2; }; //~ ERROR
[01:11:08]    |                                                 ^^ cannot move out of borrowed content
[01:11:08] error: aborting due to 9 previous errors
[01:11:08] 
[01:11:08] Some errors have detailed explanations: E0507, E0508.
[01:11:08] For more information about an error, try `rustc --explain E0507`.
---
[01:11:08] 3    |
[01:11:08] 4 LL |     let b = *a;
[01:11:08] -    |             ^^
[01:11:08] -    |             |
[01:11:08] -    |             cannot move out of borrowed content
[01:11:08] -    |             help: consider removing the `*`: `a`
[01:11:08] +    |             ^^ cannot move out of borrowed content
[01:11:08] 9 
[01:11:08] 10 error[E0508]: cannot move out of type `[A; 1]`, a non-copy array
[01:11:08] 
[01:11:08] 20   --> $DIR/move-errors.rs:19:13
[01:11:08] 21    |
[01:11:08] 22 LL |     let s = **r;
[01:11:08] 22 LL |     let s = **r;
[01:11:08] -    |             ^^^
[01:11:08] -    |             |
[01:11:08] -    |             cannot move out of borrowed content
[01:11:08] -    |             help: consider removing the `*`: `*r`
[01:11:08] +    |             ^^^ cannot move out of borrowed content
[01:11:08] 28 error[E0507]: cannot move out of an `Rc`
[01:11:08] 29   --> $DIR/move-errors.rs:27:13
[01:11:08] 
[01:11:08] 30    |
[01:11:08] 30    |
[01:11:08] 31 LL |     let s = *r;
[01:11:08] -    |             ^^
[01:11:08] -    |             |
[01:11:08] -    |             cannot move out of an `Rc`
[01:11:08] -    |             help: consider removing the `*`: `r`
[01:11:08] +    |             ^^ cannot move out of an `Rc`
[01:11:08] 36 
[01:11:08] 37 error[E0508]: cannot move out of type `[A; 1]`, a non-copy array
[01:11:08] 
[01:11:08] 47   --> $DIR/move-errors.rs:38:16
[01:11:08] 48    |
[01:11:08] 48    |
[01:11:08] 49 LL |     let A(s) = *a;
[01:11:08] -    |           |    |
[01:11:08] -    |           |    cannot move out of borrowed content
[01:11:08] -    |           |    cannot move out of borrowed content
[01:11:08] -    |           |    help: consider removing the `*`: `a`
[01:11:08] +    |           -    ^^ cannot move out of borrowed content
[01:11:08] 54    |           data moved here
[01:11:08] 55    |
[01:11:08] 55    |
[01:11:08] 56 note: move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait
[01:11:08] 167   --> $DIR/move-errors.rs:110:11
[01:11:08] 168    |
[01:11:08] 169 LL |     match *x {
[01:11:08] -    |           ^^
[01:11:08] -    |           ^^
[01:11:08] -    |           |
[01:11:08] -    |           cannot move out of borrowed content
[01:11:08] -    |           help: consider removing the `*`: `x`
[01:11:08] +    |           ^^ cannot move out of borrowed content
[01:11:08] 174 LL |
[01:11:08] 175 LL |         Ok(s) | Err(s) => (),
[01:11:08] 176    |            - data moved here
[01:11:08] 
[01:11:08] The actual stderr differed from the expected stderr.
[01:11:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/move-errors/move-errors.stderr
[01:11:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/move-errors/move-errors.stderr
[01:11:08] To update references, rerun the tests and pass the `--bless` flag
[01:11:08] To only update this specific test, also pass `--test-args nll/move-errors.rs`
[01:11:08] error: 1 errors occurred comparing output.
[01:11:08] status: exit code: 1
[01:11:08] status: exit code: 1
[01:11:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/move-errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/move-errors" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/move-errors/auxiliary" "-A" "unused"
[01:11:08] ------------------------------------------
[01:11:08] 
[01:11:08] ------------------------------------------
[01:11:08] stderr:
[01:11:08] stderr:
[01:11:08] ------------------------------------------
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/nll/move-errors.rs:6:13
[01:11:08]    |
[01:11:08] LL |     let b = *a;
[01:11:08]    |             ^^ cannot move out of borrowed content
[01:11:08] 
[01:11:08] error[E0508]: cannot move out of type `[A; 1]`, a non-copy array
[01:11:08]    |
[01:11:08] LL |     let b = a[0];
[01:11:08]    |             ^^^^
[01:11:08]    |             |
---
[01:11:08]    |
[01:11:08] LL |     let s = *r;
[01:11:08]    |             ^^ cannot move out of an `Rc`
[01:11:08] 
[01:11:08] error[E0508]: cannot move out of type `[A; 1]`, a non-copy array
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     let a = [A("".to_string())][0];
[01:11:08]    |             |
[01:11:08]    |             cannot move out of here
[01:11:08]    |             cannot move out of here
[01:11:08]    |             help: consider borrowing here: `&[A("".to_string())][0]`
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/nll/move-errors.rs:38:16
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     let A(s) = *a;
[01:11:08]    |           -    ^^ cannot move out of borrowed content
[01:11:08]    |           data moved here
[01:11:08]    |
[01:11:08]    |
[01:11:08] note: move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     let A(s) = *a;
[01:11:08] 
[01:11:08] error[E0509]: cannot move out of type `D`, which implements the `Drop` trait
[01:11:08]   --> /checkout/src/test/ui/nll/move-errors.rs:44:19
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     let C(D(s)) = c;
[01:11:08]    |             -     ^ cannot move out of here
[01:11:08]    |             data moved here
[01:11:08]    |
[01:11:08]    |
[01:11:08] note: move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     let C(D(s)) = c;
[01:11:08] 
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/nll/move-errors.rs:51:9
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     b = *a;
[01:11:08]    |         ^^ cannot move out of borrowed content
[01:11:08] 
[01:11:08] error[E0508]: cannot move out of type `[B; 1]`, a non-copy array
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     match x[0] {
[01:11:08]    |           |
[01:11:08]    |           cannot move out of here
[01:11:08]    |           help: consider borrowing here: `&x[0]`
[01:11:08] LL |     //~^ ERROR
[01:11:08] LL |     //~^ ERROR
[01:11:08] LL |         B::U(d) => (),
[01:11:08]    |              - data moved here
[01:11:08] LL |         B::V(s) => (),
[01:11:08]    |              - ...and here
[01:11:08] note: move occurs because these variables have types that don't implement the `Copy` trait
[01:11:08]   --> /checkout/src/test/ui/nll/move-errors.rs:76:14
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |         B::U(d) => (),
[01:11:08]    |              ^
[01:11:08] LL |         B::V(s) => (),
[01:11:08] 
[01:11:08] error[E0509]: cannot move out of type `D`, which implements the `Drop` trait
[01:11:08]   --> /checkout/src/test/ui/nll/move-errors.rs:83:11
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     match x {
[01:11:08]    |           ^ cannot move out of here
[01:11:08] ...
[01:11:08] LL |         B::U(D(s)) => (),
[01:11:08]    |                - data moved here
[01:11:08]    |
[01:11:08] note: move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |         B::U(D(s)) => (),
[01:11:08] 
[01:11:08] error[E0509]: cannot move out of type `D`, which implements the `Drop` trait
[01:11:08]   --> /checkout/src/test/ui/nll/move-errors.rs:92:11
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     match x {
[01:11:08]    |           ^ cannot move out of here
[01:11:08] ...
[01:11:08] LL |         (D(s), &t) => (),
[01:11:08]    |            - data moved here
[01:11:08]    |
[01:11:08] note: move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |         (D(s), &t) => (),
[01:11:08] 
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/nll/move-errors.rs:92:11
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     match x {
[01:11:08]    |           ^ cannot move out of borrowed content
[01:11:08] ...
[01:11:08] LL |         (D(s), &t) => (),
[01:11:08]    |                 - data moved here
[01:11:08] note: move occurs because `t` has type `std::string::String`, which does not implement the `Copy` trait
[01:11:08]   --> /checkout/src/test/ui/nll/move-errors.rs:95:17
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |         (D(s), &t) => (),
[01:11:08] 
[01:11:08] error[E0509]: cannot move out of type `F`, which implements the `Drop` trait
[01:11:08]   --> /checkout/src/test/ui/nll/move-errors.rs:102:11
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     match x {
[01:11:08]    |           ^ cannot move out of here
[01:11:08] LL |     //~^ ERROR
[01:11:08] LL |         F(s, mut t) => (),
[01:11:08]    |           -  ----- ...and here
[01:11:08]    |           data moved here
[01:11:08]    |
[01:11:08] note: move occurs because these variables have types that don't implement the `Copy` trait
[01:11:08]   --> /checkout/src/test/ui/nll/move-errors.rs:104:11
---
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/nll/move-errors.rs:110:11
[01:11:08]    |
[01:11:08] LL |     match *x {
[01:11:08]    |           ^^ cannot move out of borrowed content
[01:11:08] LL |     //~^ ERROR
[01:11:08] LL |         Ok(s) | Err(s) => (),
[01:11:08]    |            - data moved here
[01:11:08]    |
[01:11:08] note: move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |         Ok(s) | Err(s) => (),
[01:11:08] 
[01:11:08] error: aborting due to 14 previous errors
[01:11:08] 
[01:11:08] Some errors have detailed explanations: E0507, E0508, E0509.
---
[01:11:08] 3    |
[01:11:08] 4 LL |     let x = *&x;
[01:11:08] -    |             ^^^
[01:11:08] -    |             |
[01:11:08] -    |             cannot move out of borrowed content
[01:11:08] -    |             help: consider removing the `*`: `&x`
[01:11:08] +    |             ^^^ cannot move out of borrowed content
[01:11:08] 10 error[E0507]: cannot move out of borrowed content
[01:11:08] 11   --> $DIR/std-uncopyable-atomics.rs:11:13
[01:11:08] 
[01:11:08] 12    |
[01:11:08] 12    |
[01:11:08] 13 LL |     let x = *&x;
[01:11:08] -    |             ^^^
[01:11:08] -    |             |
[01:11:08] -    |             cannot move out of borrowed content
[01:11:08] -    |             help: consider removing the `*`: `&x`
[01:11:08] +    |             ^^^ cannot move out of borrowed content
[01:11:08] 19 error[E0507]: cannot move out of borrowed content
[01:11:08] 20   --> $DIR/std-uncopyable-atomics.rs:13:13
[01:11:08] 
[01:11:08] 21    |
[01:11:08] 21    |
[01:11:08] 22 LL |     let x = *&x;
[01:11:08] -    |             ^^^
[01:11:08] -    |             |
[01:11:08] -    |             cannot move out of borrowed content
[01:11:08] -    |             help: consider removing the `*`: `&x`
[01:11:08] +    |             ^^^ cannot move out of borrowed content
[01:11:08] 28 error[E0507]: cannot move out of borrowed content
[01:11:08] 29   --> $DIR/std-uncopyable-atomics.rs:15:13
[01:11:08] 
[01:11:08] 30    |
[01:11:08] 30    |
[01:11:08] 31 LL |     let x = *&x;
[01:11:08] -    |             ^^^
[01:11:08] -    |             |
[01:11:08] -    |             cannot move out of borrowed content
[01:11:08] -    |             help: consider removing the `*`: `&x`
[01:11:08] +    |             ^^^ cannot move out of borrowed content
[01:11:08] 37 error: aborting due to 4 previous errors
[01:11:08] 38 
[01:11:08] 
[01:11:08] 
[01:11:08] 
[01:11:08] The actual stderr differed from the expected stderr.
[01:11:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/std-uncopyable-atomics/std-uncopyable-atomics.stderr
[01:11:08] To update references, rerun the tests and pass the `--bless` flag
[01:11:08] To only update this specific test, also pass `--test-args std-uncopyable-atomics.rs`
[01:11:08] error: 1 errors occurred comparing output.
[01:11:08] status: exit code: 1
[01:11:08] status: exit code: 1
[01:11:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/std-uncopyable-atomics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/std-uncopyable-atomics" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/std-uncopyable-atomics/auxiliary" "-A" "unused"
[01:11:08] ------------------------------------------
[01:11:08] 
[01:11:08] ------------------------------------------
[01:11:08] stderr:
[01:11:08] stderr:
[01:11:08] ------------------------------------------
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/std-uncopyable-atomics.rs:9:13
[01:11:08]    |
[01:11:08] LL |     let x = *&x; //~ ERROR: cannot move out of borrowed content
[01:11:08]    |             ^^^ cannot move out of borrowed content
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/std-uncopyable-atomics.rs:11:13
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     let x = *&x; //~ ERROR: cannot move out of borrowed content
[01:11:08]    |             ^^^ cannot move out of borrowed content
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/std-uncopyable-atomics.rs:13:13
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     let x = *&x; //~ ERROR: cannot move out of borrowed content
[01:11:08]    |             ^^^ cannot move out of borrowed content
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/std-uncopyable-atomics.rs:15:13
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     let x = *&x; //~ ERROR: cannot move out of borrowed content
[01:11:08]    |             ^^^ cannot move out of borrowed content
[01:11:08] error: aborting due to 4 previous errors
[01:11:08] 
[01:11:08] For more information about this error, try `rustc --explain E0507`.
[01:11:08] 
---
[01:11:08] diff of stderr:
[01:11:08] 
[01:11:08] 2   --> $DIR/simple.rs:38:17
[01:11:08] 3    |
[01:11:08] 4 LL |     let X(_t) = *s;
[01:11:08] -    |           |     |
[01:11:08] -    |           |     cannot move out of borrowed content
[01:11:08] -    |           |     cannot move out of borrowed content
[01:11:08] -    |           |     help: consider removing the `*`: `s`
[01:11:08] +    |           --    ^^ cannot move out of borrowed content
[01:11:08] 9    |           data moved here
[01:11:08] 10    |
[01:11:08] 10    |
[01:11:08] 11 note: move occurs because `_t` has type `Y`, which does not implement the `Copy` trait
[01:11:08] 18   --> $DIR/simple.rs:42:30
[01:11:08] 19    |
[01:11:08] 19    |
[01:11:08] 20 LL |     if let Either::One(_t) = *r { }
[01:11:08] -    |                        |     |
[01:11:08] -    |                        |     cannot move out of borrowed content
[01:11:08] -    |                        |     cannot move out of borrowed content
[01:11:08] -    |                        |     help: consider removing the `*`: `r`
[01:11:08] +    |                        --    ^^ cannot move out of borrowed content
[01:11:08] 25    |                        data moved here
[01:11:08] 26    |
[01:11:08] 26    |
[01:11:08] 27 note: move occurs because `_t` has type `X`, which does not implement the `Copy` trait
[01:11:08] 34   --> $DIR/simple.rs:46:33
[01:11:08] 35    |
[01:11:08] 35    |
[01:11:08] 36 LL |     while let Either::One(_t) = *r { }
[01:11:08] -    |                           |     |
[01:11:08] -    |                           |     cannot move out of borrowed content
[01:11:08] -    |                           |     cannot move out of borrowed content
[01:11:08] -    |                           |     help: consider removing the `*`: `r`
[01:11:08] +    |                           --    ^^ cannot move out of borrowed content
[01:11:08] 41    |                           data moved here
[01:11:08] 42    |
[01:11:08] 42    |
[01:11:08] 43 note: move occurs because `_t` has type `X`, which does not implement the `Copy` trait
[01:11:08] 50   --> $DIR/simple.rs:50:11
[01:11:08] 51    |
[01:11:08] 52 LL |     match *r {
[01:11:08] -    |           ^^
[01:11:08] -    |           ^^
[01:11:08] -    |           |
[01:11:08] -    |           cannot move out of borrowed content
[01:11:08] -    |           help: consider removing the `*`: `r`
[01:11:08] +    |           ^^ cannot move out of borrowed content
[01:11:08] 57 ...
[01:11:08] 58 LL |         Either::One(_t)
[01:11:08] 59    |                     -- data moved here
[01:11:08] 68   --> $DIR/simple.rs:57:11
[01:11:08] 69    |
[01:11:08] 70 LL |     match *r {
[01:11:08] -    |           ^^
[01:11:08] -    |           ^^
[01:11:08] -    |           |
[01:11:08] -    |           cannot move out of borrowed content
[01:11:08] -    |           help: consider removing the `*`: `r`
[01:11:08] +    |           ^^ cannot move out of borrowed content
[01:11:08] 75 ...
[01:11:08] 76 LL |         Either::One(_t) => (),
[01:11:08] 77    |                     -- data moved here
[01:11:08] 86   --> $DIR/simple.rs:66:17
[01:11:08] 87    |
[01:11:08] 87    |
[01:11:08] 88 LL |     let X(_t) = *sm;
[01:11:08] -    |           |     |
[01:11:08] -    |           |     cannot move out of borrowed content
[01:11:08] -    |           |     cannot move out of borrowed content
[01:11:08] -    |           |     help: consider removing the `*`: `sm`
[01:11:08] +    |           --    ^^^ cannot move out of borrowed content
[01:11:08] 93    |           data moved here
[01:11:08] 94    |
[01:11:08] 94    |
[01:11:08] 95 note: move occurs because `_t` has type `Y`, which does not implement the `Copy` trait
[01:11:08] 102   --> $DIR/simple.rs:70:30
[01:11:08] 103    |
[01:11:08] 103    |
[01:11:08] 104 LL |     if let Either::One(_t) = *rm { }
[01:11:08] -    |                        |     |
[01:11:08] -    |                        |     cannot move out of borrowed content
[01:11:08] -    |                        |     cannot move out of borrowed content
[01:11:08] -    |                        |     help: consider removing the `*`: `rm`
[01:11:08] +    |                        --    ^^^ cannot move out of borrowed content
[01:11:08] 109    |                        data moved here
[01:11:08] 110    |
[01:11:08] 110    |
[01:11:08] 111 note: move occurs because `_t` has type `X`, which does not implement the `Copy` trait
[01:11:08] 118   --> $DIR/simple.rs:74:33
[01:11:08] 119    |
[01:11:08] 119    |
[01:11:08] 120 LL |     while let Either::One(_t) = *rm { }
[01:11:08] -    |                           |     |
[01:11:08] -    |                           |     cannot move out of borrowed content
[01:11:08] -    |                           |     cannot move out of borrowed content
[01:11:08] -    |                           |     help: consider removing the `*`: `rm`
[01:11:08] +    |                           --    ^^^ cannot move out of borrowed content
[01:11:08] 125    |                           data moved here
[01:11:08] 126    |
[01:11:08] 126    |
[01:11:08] 127 note: move occurs because `_t` has type `X`, which does not implement the `Copy` trait
[01:11:08] 134   --> $DIR/simple.rs:78:11
[01:11:08] 135    |
[01:11:08] 136 LL |     match *rm {
[01:11:08] -    |           ^^^
[01:11:08] -    |           ^^^
[01:11:08] -    |           |
[01:11:08] -    |           cannot move out of borrowed content
[01:11:08] -    |           help: consider removing the `*`: `rm`
[01:11:08] +    |           ^^^ cannot move out of borrowed content
[01:11:08] 141 ...
[01:11:08] 142 LL |         Either::One(_t)
[01:11:08] 143    |                     -- data moved here
[01:11:08] 152   --> $DIR/simple.rs:85:11
[01:11:08] 153    |
[01:11:08] 154 LL |     match *rm {
[01:11:08] -    |           ^^^
[01:11:08] -    |           ^^^
[01:11:08] -    |           |
[01:11:08] -    |           cannot move out of borrowed content
[01:11:08] -    |           help: consider removing the `*`: `rm`
[01:11:08] +    |           ^^^ cannot move out of borrowed content
[01:11:08] 159 ...
[01:11:08] 160 LL |         Either::One(_t) => (),
[01:11:08] 161    |                     -- data moved here
[01:11:08] 170   --> $DIR/simple.rs:93:11
[01:11:08] 171    |
[01:11:08] 172 LL |     match *rm {
[01:11:08] -    |           ^^^
[01:11:08] -    |           ^^^
[01:11:08] -    |           |
[01:11:08] -    |           cannot move out of borrowed content
[01:11:08] -    |           help: consider removing the `*`: `rm`
[01:11:08] +    |           ^^^ cannot move out of borrowed content
[01:11:08] 177 ...
[01:11:08] 178 LL |         Either::One(_t) => (),
[01:11:08] 179    |                     -- data moved here
[01:11:08] 188   --> $DIR/simple.rs:102:17
[01:11:08] 189    |
[01:11:08] 189    |
[01:11:08] 190 LL |     let X(_t) = vs[0];
[01:11:08] -    |           |     |
[01:11:08] -    |           |     cannot move out of borrowed content
[01:11:08] -    |           |     help: consider borrowing here: `&vs[0]`
[01:11:08] +    |           --    ^^^^^ cannot move out of borrowed content
[01:11:08] +    |           --    ^^^^^ cannot move out of borrowed content
[01:11:08] +    |           |
[01:11:08] 195    |           data moved here
[01:11:08] 196    |
[01:11:08] 197 note: move occurs because `_t` has type `Y`, which does not implement the `Copy` trait
[01:11:08] 204   --> $DIR/simple.rs:106:30
[01:11:08] 205    |
[01:11:08] 205    |
[01:11:08] 206 LL |     if let Either::One(_t) = vr[0] { }
[01:11:08] -    |                        |     |
[01:11:08] -    |                        |     cannot move out of borrowed content
[01:11:08] -    |                        |     help: consider borrowing here: `&vr[0]`
[01:11:08] +    |                        --    ^^^^^ cannot move out of borrowed content
[01:11:08] +    |                        --    ^^^^^ cannot move out of borrowed content
[01:11:08] +    |                        |
[01:11:08] 211    |                        data moved here
[01:11:08] 212    |
[01:11:08] 213 note: move occurs because `_t` has type `X`, which does not implement the `Copy` trait
[01:11:08] 220   --> $DIR/simple.rs:110:33
[01:11:08] 221    |
[01:11:08] 221    |
[01:11:08] 222 LL |     while let Either::One(_t) = vr[0] { }
[01:11:08] -    |                           |     |
[01:11:08] -    |                           |     cannot move out of borrowed content
[01:11:08] -    |                           |     help: consider borrowing here: `&vr[0]`
[01:11:08] +    |                           --    ^^^^^ cannot move out of borrowed content
[01:11:08] +    |                           --    ^^^^^ cannot move out of borrowed content
[01:11:08] +    |                           |
[01:11:08] 227    |                           data moved here
[01:11:08] 228    |
[01:11:08] 229 note: move occurs because `_t` has type `X`, which does not implement the `Copy` trait
[01:11:08] 236   --> $DIR/simple.rs:114:11
[01:11:08] 237    |
[01:11:08] 237    |
[01:11:08] 238 LL |     match vr[0] {
[01:11:08] -    |           |
[01:11:08] -    |           cannot move out of borrowed content
[01:11:08] -    |           help: consider borrowing here: `&vr[0]`
[01:11:08] +    |           ^^^^^ cannot move out of borrowed content
[01:11:08] +    |           ^^^^^ cannot move out of borrowed content
[01:11:08] 243 ...
[01:11:08] 244 LL |         Either::One(_t)
[01:11:08] 245    |                     -- data moved here
[01:11:08] 254   --> $DIR/simple.rs:121:11
[01:11:08] 255    |
[01:11:08] 255    |
[01:11:08] 256 LL |     match vr[0] {
[01:11:08] -    |           |
[01:11:08] -    |           cannot move out of borrowed content
[01:11:08] -    |           help: consider borrowing here: `&vr[0]`
[01:11:08] +    |           ^^^^^ cannot move out of borrowed content
[01:11:08] +    |           ^^^^^ cannot move out of borrowed content
[01:11:08] 261 ...
[01:11:08] 262 LL |         Either::One(_t) => (),
[01:11:08] 263    |                     -- data moved here
[01:11:08] 272   --> $DIR/simple.rs:130:17
[01:11:08] 273    |
[01:11:08] 273    |
[01:11:08] 274 LL |     let X(_t) = vsm[0];
[01:11:08] -    |           |     |
[01:11:08] -    |           |     cannot move out of borrowed content
[01:11:08] -    |           |     cannot move out of borrowed content
[01:11:08] -    |           |     help: consider borrowing here: `&vsm[0]`
[01:11:08] +    |           --    ^^^^^^ cannot move out of borrowed content
[01:11:08] 279    |           data moved here
[01:11:08] 280    |
[01:11:08] 280    |
[01:11:08] 281 note: move occurs because `_t` has type `Y`, which does not implement the `Copy` trait
[01:11:08] 288   --> $DIR/simple.rs:134:30
[01:11:08] 289    |
[01:11:08] 289    |
[01:11:08] 290 LL |     if let Either::One(_t) = vrm[0] { }
[01:11:08] -    |                        |     |
[01:11:08] -    |                        |     cannot move out of borrowed content
[01:11:08] -    |                        |     cannot move out of borrowed content
[01:11:08] -    |                        |     help: consider borrowing here: `&vrm[0]`
[01:11:08] +    |                        --    ^^^^^^ cannot move out of borrowed content
[01:11:08] 295    |                        data moved here
[01:11:08] 296    |
[01:11:08] 296    |
[01:11:08] 297 note: move occurs because `_t` has type `X`, which does not implement the `Copy` trait
[01:11:08] 304   --> $DIR/simple.rs:138:33
[01:11:08] 305    |
[01:11:08] 305    |
[01:11:08] 306 LL |     while let Either::One(_t) = vrm[0] { }
[01:11:08] -    |                           |     |
[01:11:08] -    |                           |     cannot move out of borrowed content
[01:11:08] -    |                           |     cannot move out of borrowed content
[01:11:08] -    |                           |     help: consider borrowing here: `&vrm[0]`
[01:11:08] +    |                           --    ^^^^^^ cannot move out of borrowed content
[01:11:08] 311    |                           data moved here
[01:11:08] 312    |
[01:11:08] 312    |
[01:11:08] 313 note: move occurs because `_t` has type `X`, which does not implement the `Copy` trait
[01:11:08] 320   --> $DIR/simple.rs:142:11
[01:11:08] 321    |
[01:11:08] 321    |
[01:11:08] 322 LL |     match vrm[0] {
[01:11:08] -    |           |
[01:11:08] -    |           cannot move out of borrowed content
[01:11:08] -    |           cannot move out of borrowed content
[01:11:08] -    |           help: consider borrowing here: `&vrm[0]`
[01:11:08] +    |           ^^^^^^ cannot move out of borrowed content
[01:11:08] 327 ...
[01:11:08] 328 LL |         Either::One(_t)
[01:11:08] 329    |                     -- data moved here
[01:11:08] 338   --> $DIR/simple.rs:149:11
[01:11:08] 339    |
[01:11:08] 339    |
[01:11:08] 340 LL |     match vrm[0] {
[01:11:08] -    |           |
[01:11:08] -    |           cannot move out of borrowed content
[01:11:08] -    |           cannot move out of borrowed content
[01:11:08] -    |           help: consider borrowing here: `&vrm[0]`
[01:11:08] +    |           ^^^^^^ cannot move out of borrowed content
[01:11:08] 345 ...
[01:11:08] 346 LL |         Either::One(_t) => (),
[01:11:08] 347    |                     -- data moved here
[01:11:08] 356   --> $DIR/simple.rs:157:11
[01:11:08] 357    |
[01:11:08] 357    |
[01:11:08] 358 LL |     match vrm[0] {
[01:11:08] -    |           |
[01:11:08] -    |           cannot move out of borrowed content
[01:11:08] -    |           cannot move out of borrowed content
[01:11:08] -    |           help: consider borrowing here: `&vrm[0]`
[01:11:08] +    |           ^^^^^^ cannot move out of borrowed content
[01:11:08] 363 ...
[01:11:08] 364 LL |         Either::One(_t) => (),
[01:11:08] 365    |                     -- data moved here
[01:11:08] 
[01:11:08] The actual stderr differed from the expected stderr.
[01:11:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/dont-suggest-ref/simple/simple.stderr
[01:11:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/dont-suggest-ref/simple/simple.stderr
[01:11:08] To update references, rerun the tests and pass the `--bless` flag
[01:11:08] To only update this specific test, also pass `--test-args suggestions/dont-suggest-ref/simple.rs`
[01:11:08] error: 1 errors occurred comparing output.
[01:11:08] status: exit code: 1
[01:11:08] status: exit code: 1
[01:11:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/dont-suggest-ref/simple" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/dont-suggest-ref/simple/auxiliary" "-A" "unused"
[01:11:08] ------------------------------------------
[01:11:08] 
[01:11:08] ------------------------------------------
[01:11:08] stderr:
[01:11:08] stderr:
[01:11:08] ------------------------------------------
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:38:17
[01:11:08]    |
[01:11:08] LL |     let X(_t) = *s;
[01:11:08]    |           --    ^^ cannot move out of borrowed content
[01:11:08]    |           data moved here
[01:11:08]    |
[01:11:08]    |
[01:11:08] note: move occurs because `_t` has type `Y`, which does not implement the `Copy` trait
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     let X(_t) = *s;
[01:11:08] 
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:42:30
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     if let Either::One(_t) = *r { }
[01:11:08]    |                        --    ^^ cannot move out of borrowed content
[01:11:08]    |                        data moved here
[01:11:08]    |
[01:11:08]    |
[01:11:08] note: move occurs because `_t` has type `X`, which does not implement the `Copy` trait
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     if let Either::One(_t) = *r { }
[01:11:08] 
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:46:33
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     while let Either::One(_t) = *r { }
[01:11:08]    |                           --    ^^ cannot move out of borrowed content
[01:11:08]    |                           data moved here
[01:11:08]    |
[01:11:08]    |
[01:11:08] note: move occurs because `_t` has type `X`, which does not implement the `Copy` trait
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     while let Either::One(_t) = *r { }
[01:11:08] 
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:50:11
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     match *r {
[01:11:08]    |           ^^ cannot move out of borrowed content
[01:11:08] ...
[01:11:08] LL |         Either::One(_t)
[01:11:08]    |                     -- data moved here
[01:11:08]    |
[01:11:08] note: move occurs because `_t` has type `X`, which does not implement the `Copy` trait
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |         Either::One(_t)
[01:11:08] 
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:57:11
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     match *r {
[01:11:08]    |           ^^ cannot move out of borrowed content
[01:11:08] ...
[01:11:08] LL |         Either::One(_t) => (),
[01:11:08]    |                     -- data moved here
[01:11:08]    |
[01:11:08] note: move occurs because `_t` has type `X`, which does not implement the `Copy` trait
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |         Either::One(_t) => (),
[01:11:08] 
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:66:17
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     let X(_t) = *sm;
[01:11:08]    |           --    ^^^ cannot move out of borrowed content
[01:11:08]    |           data moved here
[01:11:08]    |
[01:11:08]    |
[01:11:08] note: move occurs because `_t` has type `Y`, which does not implement the `Copy` trait
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     let X(_t) = *sm;
[01:11:08] 
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:70:30
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     if let Either::One(_t) = *rm { }
[01:11:08]    |                        --    ^^^ cannot move out of borrowed content
[01:11:08]    |                        data moved here
[01:11:08]    |
[01:11:08]    |
[01:11:08] note: move occurs because `_t` has type `X`, which does not implement the `Copy` trait
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     if let Either::One(_t) = *rm { }
[01:11:08] 
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:74:33
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     while let Either::One(_t) = *rm { }
[01:11:08]    |                           --    ^^^ cannot move out of borrowed content
[01:11:08]    |                           data moved here
[01:11:08]    |
[01:11:08]    |
[01:11:08] note: move occurs because `_t` has type `X`, which does not implement the `Copy` trait
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     while let Either::One(_t) = *rm { }
[01:11:08] 
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:78:11
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     match *rm {
[01:11:08]    |           ^^^ cannot move out of borrowed content
[01:11:08] ...
[01:11:08] LL |         Either::One(_t)
[01:11:08]    |                     -- data moved here
[01:11:08]    |
[01:11:08] note: move occurs because `_t` has type `X`, which does not implement the `Copy` trait
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |         Either::One(_t)
[01:11:08] 
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:85:11
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     match *rm {
[01:11:08]    |           ^^^ cannot move out of borrowed content
[01:11:08] ...
[01:11:08] LL |         Either::One(_t) => (),
[01:11:08]    |                     -- data moved here
[01:11:08]    |
[01:11:08] note: move occurs because `_t` has type `X`, which does not implement the `Copy` trait
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |         Either::One(_t) => (),
[01:11:08] 
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:93:11
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     match *rm {
[01:11:08]    |           ^^^ cannot move out of borrowed content
[01:11:08] ...
[01:11:08] LL |         Either::One(_t) => (),
[01:11:08]    |                     -- data moved here
[01:11:08]    |
[01:11:08] note: move occurs because `_t` has type `X`, which does not implement the `Copy` trait
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |         Either::One(_t) => (),
[01:11:08] 
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:102:17
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     let X(_t) = vs[0];
[01:11:08]    |           --    ^^^^^ cannot move out of borrowed content
[01:11:08]    |           data moved here
[01:11:08]    |
[01:11:08]    |
[01:11:08] note: move occurs because `_t` has type `Y`, which does not implement the `Copy` trait
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     let X(_t) = vs[0];
[01:11:08] 
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:106:30
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     if let Either::One(_t) = vr[0] { }
[01:11:08]    |                        --    ^^^^^ cannot move out of borrowed content
[01:11:08]    |                        data moved here
[01:11:08]    |
[01:11:08]    |
[01:11:08] note: move occurs because `_t` has type `X`, which does not implement the `Copy` trait
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     if let Either::One(_t) = vr[0] { }
[01:11:08] 
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:110:33
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     while let Either::One(_t) = vr[0] { }
[01:11:08]    |                           --    ^^^^^ cannot move out of borrowed content
[01:11:08]    |                           data moved here
[01:11:08]    |
[01:11:08]    |
[01:11:08] note: move occurs because `_t` has type `X`, which does not implement the `Copy` trait
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     while let Either::One(_t) = vr[0] { }
[01:11:08] 
[01:11:08] error[E0507]: cannot move out of borrowed content
[01:11:08]   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:114:11
[01:11:08]    |
[01:11:08]    |
[01:11:08] LL |     match vr[0] {
[01:11:08]    |           ^^^^^ cannot move out of borrowed content
[01:11:08] ...
---
[01:11:08] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:11:08] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:11:08] 
[01:11:08] 
[01:11:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:11:08] 
[01:11:08] 
[01:11:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:11:08] Build completed unsuccessfully in 0:04:44
[01:11:08] Build completed unsuccessfully in 0:04:44
[01:11:08] make: *** [check] Error 1
[01:11:08] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:3368dbc8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu May 30 00:13:13 UTC 2019

plain
travis_time:end:109c13ee:start=1558845392245305101,finish=1558845394474211229,duration=2228906128
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_run-pass
Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:16:30] 
[01:16:30] running 2922 tests
[01:16:42] .....F.............................................................................................. 100/2922
[01:16:55] .........F.F......................F......................................F...i...................... 200/2922
[01:17:04] .................................................................................................... 300/2922
[01:17:15] ....................F.............................F........F.............................F...F..F... 400/2922
[01:17:25] .................................................................................................... 500/2922
[01:17:37] .....F.............................................................................................. 600/2922
[01:17:52] ....F.................F............F.F.F.FF..F..FF.................................................. 700/2922
[01:18:04] .................................F.......F.......................................................... 800/2922
[01:18:13] ...........F....................................................................F...............FFF. 900/2922
[01:18:27] .F.FFFF......F........................F............................................................. 1000/2922
[01:18:39] ........F.....F....F.....F.F...........F.............F..F........F....FF..FF.......F..............F. 1100/2922
[01:18:49] ...................F....F.......F..F......F...F....................................F...........F..F. 1200/2922
[01:18:59] ..FF..F.....F....F....F..........F.....F......F..................FF........................F...F..F. 1300/2922
[01:19:12] ..F..........FFF..Fii..F....F..F...F....F.....................F.........................F.FF........ 1400/2922
[01:19:24] .......F..........FF..........F.............F.....F.........F....F........F................F........ 1500/2922
[01:19:34] .....F......F.........F.......F......F.....F..........................Fi.......i...........F........ 1600/2922
[01:19:48] .F..........................FF........F...FF.F...........F.....FFF..................F....F........F. 1700/2922
[01:20:02] ...................F...................................F............................................ 1800/2922
[01:20:13] .............................F....F...................................F..F....F............F........ 1900/2922
[01:20:28] ..i.........................................F....FF.......................i......................... 2000/2922
[01:20:54] ....................FFFFFFFF.......................FFF....................................F........F 2100/2922
[01:21:14] ................................................................Ftest [run-pass] run-pass/mpsc_stress.rs has been running for over 60 seconds
[01:21:18] .................FF.F.F........F... 2200/2922
[01:21:29] FF.........F...F...........F.....F.F................................................................ 2300/2922
[01:21:49] ........ii..............................................................F.......FFF............F.... 2400/2922
[01:22:00] ......................F.........................F.F................................................. 2500/2922
[01:22:34] ........................................................................................F....F..F... 2600/2922
[01:22:44] FFF.....F..F...F..F.....F.F..F......FF.........................FFFFF.............F.....F...FF..F.... 2700/2922
[01:22:55] ............FF.F.................F.FF....FF....F.....F........FF...FF....F.......................... 2800/2922
[01:23:08] ........................................F.......................F...........................F....... 2900/2922
[01:23:12] failures:
[01:23:12] 
[01:23:12] ---- [run-pass] run-pass/alignment-gep-tup-like-1.rs stdout ----
[01:23:12] normalized stderr:
[01:23:12] normalized stderr:
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL | fn f<A:Clone + 'static>(a: A, b: u16) -> Box<Invokable<A>+'static> {
[01:23:12]    |                                              ^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Invokable<A>+'static`
[01:23:12]    |
[01:23:12]    = note: #[warn(bare_trait_objects)] on by default
[01:23:12] 
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL |     } as (Box<Invokable<A>+'static>)
[01:23:12]    |               ^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Invokable<A>+'static`
[01:23:12] 
[01:23:12] 
[01:23:12] 
[01:23:12] The actual stderr differed from the expected stderr.
[01:23:12] The actual stderr differed from the expected stderr.
[01:23:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/alignment-gep-tup-like-1/alignment-gep-tup-like-1.stderr
[01:23:12] To update references, rerun the tests and pass the `--bless` flag
[01:23:12] To only update this specific test, also pass `--test-args alignment-gep-tup-like-1.rs`
[01:23:12] error: 1 errors occurred comparing output.
[01:23:12] status: exit code: 0
[01:23:12] status: exit code: 0
[01:23:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/alignment-gep-tup-like-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/alignment-gep-tup-like-1/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/alignment-gep-tup-like-1/auxiliary"
[01:23:12] ------------------------------------------
[01:23:12] 
[01:23:12] ------------------------------------------
[01:23:12] stderr:
[01:23:12] stderr:
[01:23:12] ------------------------------------------
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL | fn f<A:Clone + 'static>(a: A, b: u16) -> Box<Invokable<A>+'static> {
[01:23:12]    |                                              ^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Invokable<A>+'static`
[01:23:12]    |
[01:23:12]    = note: #[warn(bare_trait_objects)] on by default
[01:23:12] 
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL |     } as (Box<Invokable<A>+'static>)
[01:23:12]    |               ^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Invokable<A>+'static`
[01:23:12] 
[01:23:12] ------------------------------------------
[01:23:12] 
[01:23:12] 
[01:23:12] 
[01:23:12] ---- [run-pass] run-pass/associated-types/associated-types-doubleendediterator-object.rs stdout ----
[01:23:12] normalized stderr:
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL | fn pairwise_sub(mut t: Box<DoubleEndedIterator<Item=isize>>) -> isize {
[01:23:12]    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn DoubleEndedIterator<Item=isize>`
[01:23:12]    |
[01:23:12]    = note: #[warn(bare_trait_objects)] on by default
[01:23:12] 
[01:23:12] 
[01:23:12] 
[01:23:12] The actual stderr differed from the expected stderr.
[01:23:12] The actual stderr differed from the expected stderr.
[01:23:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-doubleendediterator-object/associated-types-doubleendediterator-object.stderr
[01:23:12] To update references, rerun the tests and pass the `--bless` flag
[01:23:12] To only update this specific test, also pass `--test-args associated-types/associated-types-doubleendediterator-object.rs`
[01:23:12] error: 1 errors occurred comparing output.
[01:23:12] status: exit code: 0
[01:23:12] status: exit code: 0
[01:23:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/associated-types/associated-types-doubleendediterator-object.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-doubleendediterator-object/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-doubleendediterator-object/auxiliary"
[01:23:12] ------------------------------------------
[01:23:12] 
[01:23:12] ------------------------------------------
[01:23:12] stderr:
[01:23:12] stderr:
[01:23:12] ------------------------------------------
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL | fn pairwise_sub(mut t: Box<DoubleEndedIterator<Item=isize>>) -> isize {
[01:23:12]    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn DoubleEndedIterator<Item=isize>`
[01:23:12]    |
[01:23:12]    = note: #[warn(bare_trait_objects)] on by default
[01:23:12] 
[01:23:12] ------------------------------------------
[01:23:12] 
[01:23:12] 
[01:23:12] 
[01:23:12] ---- [run-pass] run-pass/associated-types/associated-types-eq-obj.rs stdout ----
[01:23:12] normalized stderr:
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]   --> $DIR/associated-types-eq-obj.rs:18:12
[01:23:12]    |
[01:23:12] LL | fn baz(x: &Foo<A=Bar>) -> Bar {
[01:23:12]    |            ^^^^^^^^^^ help: use `dyn`: `dyn Foo<A=Bar>`
[01:23:12]    |
[01:23:12]    = note: #[warn(bare_trait_objects)] on by default
[01:23:12] 
[01:23:12] 
[01:23:12] 
[01:23:12] The actual stderr differed from the expected stderr.
[01:23:12] The actual stderr differed from the expected stderr.
[01:23:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-eq-obj/associated-types-eq-obj.stderr
[01:23:12] To update references, rerun the tests and pass the `--bless` flag
[01:23:12] To only update this specific test, also pass `--test-args associated-types/associated-types-eq-obj.rs`
[01:23:12] error: 1 errors occurred comparing output.
[01:23:12] status: exit code: 0
[01:23:12] status: exit code: 0
[01:23:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/associated-types/associated-types-eq-obj.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-eq-obj/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-eq-obj/auxiliary"
[01:23:12] ------------------------------------------
[01:23:12] 
[01:23:12] ------------------------------------------
[01:23:12] stderr:
[01:23:12] stderr:
[01:23:12] ------------------------------------------
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]   --> /checkout/src/test/run-pass/associated-types/associated-types-eq-obj.rs:18:12
[01:23:12]    |
[01:23:12] LL | fn baz(x: &Foo<A=Bar>) -> Bar {
[01:23:12]    |            ^^^^^^^^^^ help: use `dyn`: `dyn Foo<A=Bar>`
[01:23:12]    |
[01:23:12]    = note: #[warn(bare_trait_objects)] on by default
[01:23:12] 
[01:23:12] ------------------------------------------
[01:23:12] 
[01:23:12] 
[01:23:12] 
[01:23:12] ---- [run-pass] run-pass/associated-types/associated-types-projection-in-object-type.rs stdout ----
[01:23:12] normalized stderr:
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL |     fn subscribe(&mut self, _: Box<Subscriber<Input=Self::Output> + 'a>);
[01:23:12]    |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Subscriber<Input=Self::Output> + 'a`
[01:23:12]    |
[01:23:12]    = note: #[warn(bare_trait_objects)] on by default
[01:23:12] 
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL |     sub: Box<Subscriber<Input=u64> + 'a>
[01:23:12]    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Subscriber<Input=u64> + 'a`
[01:23:12] 
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL |     fn subscribe(&mut self, t : Box<Subscriber<Input=u64> + 'a>) {
[01:23:12]    |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Subscriber<Input=u64> + 'a`
[01:23:12] 
[01:23:12] 
[01:23:12] 
[01:23:12] The actual stderr differed from the expected stderr.
[01:23:12] The actual stderr differed from the expected stderr.
[01:23:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-projection-in-object-type/associated-types-projection-in-object-type.stderr
[01:23:12] To update references, rerun the tests and pass the `--bless` flag
[01:23:12] To only update this specific test, also pass `--test-args associated-types/associated-types-projection-in-object-type.rs`
[01:23:12] error: 1 errors occurred comparing output.
[01:23:12] status: exit code: 0
[01:23:12] status: exit code: 0
[01:23:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/associated-types/associated-types-projection-in-object-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-projection-in-object-type/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-types/associated-types-projection-in-object-type/auxiliary"
[01:23:12] ------------------------------------------
[01:23:12] 
[01:23:12] ------------------------------------------
[01:23:12] stderr:
[01:23:12] stderr:
[01:23:12] ------------------------------------------
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL |     fn subscribe(&mut self, _: Box<Subscriber<Input=Self::Output> + 'a>);
[01:23:12]    |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Subscriber<Input=Self::Output> + 'a`
[01:23:12]    |
[01:23:12]    = note: #[warn(bare_trait_objects)] on by default
[01:23:12] 
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL |     sub: Box<Subscriber<Input=u64> + 'a>
[01:23:12]    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Subscriber<Input=u64> + 'a`
[01:23:12] 
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL |     fn subscribe(&mut self, t : Box<Subscriber<Input=u64> + 'a>) {
[01:23:12]    |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Subscriber<Input=u64> + 'a`
[01:23:12] 
[01:23:12] ------------------------------------------
[01:23:12] 
[01:23:12] 
[01:23:12] 
[01:23:12] ---- [run-pass] run-pass/autoref-autoderef/autoderef-method-on-trait.rs stdout ----
[01:23:12] normalized stderr:
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL |     let x: Box<_> = box (box 3usize as Box<double>);
[01:23:12]    |                                            ^^^^^^ help: use `dyn`: `dyn double`
[01:23:12]    |
[01:23:12]    = note: #[warn(bare_trait_objects)] on by default
[01:23:12] 
[01:23:12] 
[01:23:12] 
[01:23:12] The actual stderr differed from the expected stderr.
[01:23:12] The actual stderr differed from the expected stderr.
[01:23:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/autoref-autoderef/autoderef-method-on-trait/autoderef-method-on-trait.stderr
[01:23:12] To update references, rerun the tests and pass the `--bless` flag
[01:23:12] To only update this specific test, also pass `--test-args autoref-autoderef/autoderef-method-on-trait.rs`
[01:23:12] error: 1 errors occurred comparing output.
[01:23:12] status: exit code: 0
[01:23:12] status: exit code: 0
[01:23:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/autoref-autoderef/autoderef-method-on-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/autoref-autoderef/autoderef-method-on-trait/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/autoref-autoderef/autoderef-method-on-trait/auxiliary"
[01:23:12] ------------------------------------------
[01:23:12] 
[01:23:12] ------------------------------------------
[01:23:12] stderr:
[01:23:12] stderr:
[01:23:12] ------------------------------------------
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL |     let x: Box<_> = box (box 3usize as Box<double>);
[01:23:12]    |                                            ^^^^^^ help: use `dyn`: `dyn double`
[01:23:12]    |
[01:23:12]    = note: #[warn(bare_trait_objects)] on by default
[01:23:12] 
[01:23:12] ------------------------------------------
[01:23:12] 
[01:23:12] 
[01:23:12] 
[01:23:12] ---- [run-pass] run-pass/borrowck/borrowck-trait-lifetime.rs stdout ----
[01:23:12] normalized stderr:
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL |     fn f<'a, V: T>(v: &'a V) -> &'a T {
[01:23:12]    |                                     ^ help: use `dyn`: `dyn T`
[01:23:12]    |
[01:23:12]    = note: #[warn(bare_trait_objects)] on by default
[01:23:12] 
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL |         v as &'a T
[01:23:12]    |                  ^ help: use `dyn`: `dyn T`
[01:23:12] 
[01:23:12] 
[01:23:12] 
[01:23:12] The actual stderr differed from the expected stderr.
[01:23:12] The actual stderr differed from the expected stderr.
[01:23:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/borrowck/borrowck-trait-lifetime/borrowck-trait-lifetime.stderr
[01:23:12] To update references, rerun the tests and pass the `--bless` flag
[01:23:12] To only update this specific test, also pass `--test-args borrowck/borrowck-trait-lifetime.rs`
[01:23:12] error: 1 errors occurred comparing output.
[01:23:12] status: exit code: 0
[01:23:12] status: exit code: 0
[01:23:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/borrowck/borrowck-trait-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/borrowck/borrowck-trait-lifetime/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/borrowck/borrowck-trait-lifetime/auxiliary"
[01:23:12] ------------------------------------------
[01:23:12] 
[01:23:12] ------------------------------------------
[01:23:12] stderr:
[01:23:12] stderr:
[01:23:12] ------------------------------------------
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL |     fn f<'a, V: T>(v: &'a V) -> &'a T {
[01:23:12]    |                                     ^ help: use `dyn`: `dyn T`
[01:23:12]    |
[01:23:12]    = note: #[warn(bare_trait_objects)] on by default
[01:23:12] 
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL |         v as &'a T
[01:23:12]    |                  ^ help: use `dyn`: `dyn T`
[01:23:12] 
[01:23:12] ------------------------------------------
[01:23:12] 
[01:23:12] 
[01:23:12] 
[01:23:12] ---- [run-pass] run-pass/cast-rfc0401-vtable-kinds.rs stdout ----
[01:23:12] normalized stderr:
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL | unsafe fn round_trip_and_call<'a>(t: *const (Foo<u32>+'a)) -> u32 {
[01:23:12]    |                                              ^^^^^^^^^^^ help: use `dyn`: `dyn Foo<u32>+'a`
[01:23:12]    |
[01:23:12]    = note: #[warn(bare_trait_objects)] on by default
[01:23:12] 
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL |     let foo_e : *const Foo<u16> = t as *const _;
[01:23:12]    |                        ^^^^^^^^ help: use `dyn`: `dyn Foo<u16>`
[01:23:12] 
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL |     let r_1 = foo_e as *mut Foo<u32>;
[01:23:12]    |                             ^^^^^^^^ help: use `dyn`: `dyn Foo<u32>`
[01:23:12] 
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL |     let y : &Foo<u32> = &x;
[01:23:12]    |              ^^^^^^^^ help: use `dyn`: `dyn Foo<u32>`
[01:23:12] 
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL |     let fl = unsafe { round_trip_and_call(y as *const Foo<u32>) };
[01:23:12]    |                                                       ^^^^^^^^ help: use `dyn`: `dyn Foo<u32>`
[01:23:12] 
[01:23:12] 
[01:23:12] 
[01:23:12] The actual stderr differed from the expected stderr.
[01:23:12] The actual stderr differed from the expected stderr.
[01:23:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/cast-rfc0401-vtable-kinds/cast-rfc0401-vtable-kinds.stderr
[01:23:12] To update references, rerun the tests and pass the `--bless` flag
[01:23:12] To only update this specific test, also pass `--test-args cast-rfc0401-vtable-kinds.rs`
[01:23:12] error: 1 errors occurred comparing output.
[01:23:12] status: exit code: 0
[01:23:12] status: exit code: 0
[01:23:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/cast-rfc0401-vtable-kinds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/cast-rfc0401-vtable-kinds/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/cast-rfc0401-vtable-kinds/auxiliary"
[01:23:12] ------------------------------------------
[01:23:12] 
[01:23:12] ------------------------------------------
[01:23:12] stderr:
[01:23:12] stderr:
[01:23:12] ------------------------------------------
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL | unsafe fn round_trip_and_call<'a>(t: *const (Foo<u32>+'a)) -> u32 {
[01:23:12]    |                                              ^^^^^^^^^^^ help: use `dyn`: `dyn Foo<u32>+'a`
[01:23:12]    |
[01:23:12]    = note: #[warn(bare_trait_objects)] on by default
[01:23:12] 
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL |     let foo_e : *const Foo<u16> = t as *const _;
[01:23:12]    |                        ^^^^^^^^ help: use `dyn`: `dyn Foo<u16>`
[01:23:12] 
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL |     let r_1 = foo_e as *mut Foo<u32>;
[01:23:12]    |                             ^^^^^^^^ help: use `dyn`: `dyn Foo<u32>`
[01:23:12] 
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL |     let y : &Foo<u32> = &x;
[01:23:12]    |              ^^^^^^^^ help: use `dyn`: `dyn Foo<u32>`
[01:23:12] 
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL |     let fl = unsafe { round_trip_and_call(y as *const Foo<u32>) };
[01:23:12]    |                                                       ^^^^^^^^ help: use `dyn`: `dyn Foo<u32>`
[01:23:12] 
[01:23:12] ------------------------------------------
[01:23:12] 
[01:23:12] 
[01:23:12] 
[01:23:12] ---- [run-pass] run-pass/cast-rfc0401.rs stdout ----
[01:23:12] normalized stderr:
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL |     assert_eq!((itr as &mut Iterator<Item=u32>).next(), Some(137));
[01:23:12]    |                             ^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Iterator<Item=u32>`
[01:23:12]    |
[01:23:12]    = note: #[warn(bare_trait_objects)] on by default
[01:23:12] 
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL |     assert_eq!((itr as &mut Iterator<Item=u32>).next(), None);
[01:23:12]    |                             ^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Iterator<Item=u32>`
[01:23:12] 
[01:23:12] 
[01:23:12] 
[01:23:12] The actual stderr differed from the expected stderr.
[01:23:12] The actual stderr differed from the expected stderr.
[01:23:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/cast-rfc0401/cast-rfc0401.stderr
[01:23:12] To update references, rerun the tests and pass the `--bless` flag
[01:23:12] To only update this specific test, also pass `--test-args cast-rfc0401.rs`
[01:23:12] error: 1 errors occurred comparing output.
[01:23:12] status: exit code: 0
[01:23:12] status: exit code: 0
[01:23:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/cast-rfc0401.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/cast-rfc0401/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/cast-rfc0401/auxiliary"
[01:23:12] ------------------------------------------
[01:23:12] 
[01:23:12] ------------------------------------------
[01:23:12] stderr:
[01:23:12] stderr:
[01:23:12] ------------------------------------------
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL |     assert_eq!((itr as &mut Iterator<Item=u32>).next(), Some(137));
[01:23:12]    |                             ^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Iterator<Item=u32>`
[01:23:12]    |
[01:23:12]    = note: #[warn(bare_trait_objects)] on by default
[01:23:12] 
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL |     assert_eq!((itr as &mut Iterator<Item=u32>).next(), None);
[01:23:12]    |                             ^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Iterator<Item=u32>`
[01:23:12] 
[01:23:12] ------------------------------------------
[01:23:12] 
[01:23:12] 
[01:23:12] 
[01:23:12] ---- [run-pass] run-pass/close-over-big-then-small-data.rs stdout ----
[01:23:12] normalized stderr:
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL | fn f<A:Clone + 'static>(a: A, b: u16) -> Box<Invokable<A>+'static> {
[01:23:12]    |                                              ^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Invokable<A>+'static`
[01:23:12]    |
[01:23:12]    = note: #[warn(bare_trait_objects)] on by default
[01:23:12] 
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL |     } as (Box<Invokable<A>+'static>)
[01:23:12]    |               ^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Invokable<A>+'static`
[01:23:12] 
[01:23:12] 
[01:23:12] 
[01:23:12] The actual stderr differed from the expected stderr.
[01:23:12] The actual stderr differed from the expected stderr.
[01:23:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/close-over-big-then-small-data/close-over-big-then-small-data.stderr
[01:23:12] To update references, rerun the tests and pass the `--bless` flag
[01:23:12] To only update this specific test, also pass `--test-args close-over-big-then-small-data.rs`
[01:23:12] error: 1 errors occurred comparing output.
[01:23:12] status: exit code: 0
[01:23:12] status: exit code: 0
[01:23:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/close-over-big-then-small-data.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/close-over-big-then-small-data/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/close-over-big-then-small-data/auxiliary"
[01:23:12] ------------------------------------------
[01:23:12] 
[01:23:12] ------------------------------------------
[01:23:12] stderr:
[01:23:12] stderr:
[01:23:12] ------------------------------------------
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
[01:23:12]    |
[01:23:12] LL | fn f<A:Clone + 'static>(a: A, b: u16) -> Box<Invokable<A>+'static> {
[01:23:12]    |                                              ^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Invokable<A>+'static`
[01:23:12]    |
[01:23:12]    = note: #[warn(bare_trait_objects)] on by default
[01:23:12] 
[01:23:12] warning: trait objects without an explicit `dyn` are deprecated
[01:23:12]    |
---
[01:23:12] test result: FAILED. 2705 passed; 208 failed; 9 ignored; 0 measured; 0 filtered out
[01:23:12] 
[01:23:12] 
[01:23:12] 
[01:23:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:23:12] 
[01:23:12] 
[01:23:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:23:12] Build completed unsuccessfully in 0:11:43

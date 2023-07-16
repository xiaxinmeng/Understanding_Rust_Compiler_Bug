plain
travis_time:end:08841213:start=1559299557140579932,finish=1559299557903114989,duration=762535057
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
[00:53:14] 
[00:53:14] running 5605 tests
[00:53:17] ..................................................F...............F................................. 100/5605
[00:53:27] .................................................................................................... 300/5605
[00:53:30] .................................................................................................... 400/5605
[00:53:33] ...................................................................................................i 500/5605
[00:53:36] .................................................................................................... 600/5605
[00:53:36] .................................................................................................... 600/5605
[00:53:40] .................................................................................................... 700/5605
[00:53:45] .................................................................................................... 800/5605
[00:53:49] .......................................................................................i...........i 900/5605
[00:53:53] .................................................................................................... 1000/5605
[00:53:57] ................iiiii............................................................................... 1100/5605
[00:53:59] .................................................................................................... 1200/5605
[00:54:02] .......F...........F................................................................................ 1300/5605
[00:54:04] ............................................................................................F....... 1400/5605
[00:54:10] .................................................................................................... 1600/5605
[00:54:13] ................................................i................................................... 1700/5605
[00:54:16] .................................................................................................... 1800/5605
[00:54:20] .................................................................................................... 1900/5605
[00:54:20] .................................................................................................... 1900/5605
[00:54:23] .................................................................................................... 2000/5605
[00:54:26] .....................................................................................i.............. 2100/5605
[00:54:30] .......................................................................................F............ 2200/5605
[00:54:34] ..................................................................................F................. 2300/5605
[00:54:42] .................................................................................................... 2500/5605
[00:54:46] .................................................................................................... 2600/5605
[00:54:49] .................................................................................................... 2700/5605
[00:54:53] .................................................................................................... 2800/5605
---
[00:56:13] .................................................................................................... 4800/5605
[00:56:19] .................................................................................................... 4900/5605
[00:56:22] .................................................................................................... 5000/5605
[00:56:26] .................................................................................................... 5100/5605
[00:56:30] .....F..........................................................................................F... 5200/5605
[00:56:36] .................................................................................................... 5400/5605
[00:56:39] .................................................................................................... 5500/5605
[00:56:39] .................................................................................................... 5500/5605
[00:56:42] ...........................................i........................F............................... 5600/5605
[00:56:42] failures:
[00:56:42] 
[00:56:42] ---- [ui] ui/associated-type/associated-type-projection-from-multiple-supertraits.rs stdout ----
[00:56:42] diff of stderr:
[00:56:42] diff of stderr:
[00:56:42] 
[00:56:42] 25 error[E0191]: the value of the associated type `Color` (from the trait `Vehicle`) must be specified
[00:56:42] 27    |
[00:56:42] - LL |     type Color;
[00:56:42] - LL |     type Color;
[00:56:42] -    |     ----------- `Color` defined here
[00:56:42] - ...
[00:56:42] 31 LL | fn dent_object<COLOR>(c: dyn BoxCar<Color=COLOR>) {
[00:56:42] 32    |                          ^^^^^^^^^^^^^^^^^^^^^^^ associated type `Color` must be specified
[00:56:42] 
[00:56:42] 
[00:56:42] The actual stderr differed from the expected stderr.
[00:56:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-from-multiple-supertraits/associated-type-projection-from-multiple-supertraits.stderr
[00:56:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-from-multiple-supertraits/associated-type-projection-from-multiple-supertraits.stderr
[00:56:42] To update references, rerun the tests and pass the `--bless` flag
[00:56:42] To only update this specific test, also pass `--test-args associated-type/associated-type-projection-from-multiple-supertraits.rs`
[00:56:42] error: 1 errors occurred comparing output.
[00:56:42] status: exit code: 1
[00:56:42] status: exit code: 1
[00:56:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type/associated-type-projection-from-multiple-supertraits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-from-multiple-supertraits" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type/associated-type-projection-from-multiple-supertraits/auxiliary" "-A" "unused"
[00:56:42] ------------------------------------------
[00:56:42] 
[00:56:42] ------------------------------------------
[00:56:42] stderr:
[00:56:42] stderr:
[00:56:42] ------------------------------------------
[00:56:42] error[E0221]: ambiguous associated type `Color` in bounds of `C`
[00:56:42]   --> /checkout/src/test/ui/associated-type/associated-type-projection-from-multiple-supertraits.rs:19:32
[00:56:42]    |
[00:56:42] LL |     type Color;
[00:56:42]    |     ----------- ambiguous `Color` from `Vehicle`
[00:56:42] LL |     type Color;
[00:56:42] LL |     type Color;
[00:56:42]    |     ----------- ambiguous `Color` from `Box`
[00:56:42] ...
[00:56:42] LL | fn dent<C:BoxCar>(c: C, color: C::Color) {
[00:56:42] 
[00:56:42] error[E0221]: ambiguous associated type `Color` in bounds of `BoxCar`
[00:56:42]   --> /checkout/src/test/ui/associated-type/associated-type-projection-from-multiple-supertraits.rs:23:37
[00:56:42]    |
[00:56:42]    |
[00:56:42] LL |     type Color;
[00:56:42]    |     ----------- ambiguous `Color` from `Vehicle`
[00:56:42] LL |     type Color;
[00:56:42] LL |     type Color;
[00:56:42]    |     ----------- ambiguous `Color` from `Box`
[00:56:42] ...
[00:56:42] LL | fn dent_object<COLOR>(c: dyn BoxCar<Color=COLOR>) {
[00:56:42] 
[00:56:42] 
[00:56:42] error[E0191]: the value of the associated type `Color` (from the trait `Vehicle`) must be specified
[00:56:42]    |
[00:56:42]    |
[00:56:42] LL | fn dent_object<COLOR>(c: dyn BoxCar<Color=COLOR>) {
[00:56:42]    |                          ^^^^^^^^^^^^^^^^^^^^^^^ associated type `Color` must be specified
[00:56:42] error[E0221]: ambiguous associated type `Color` in bounds of `C`
[00:56:42]   --> /checkout/src/test/ui/associated-type/associated-type-projection-from-multiple-supertraits.rs:28:29
[00:56:42]    |
[00:56:42] LL |     type Color;
[00:56:42] LL |     type Color;
[00:56:42]    |     ----------- ambiguous `Color` from `Vehicle`
[00:56:42] LL |     type Color;
[00:56:42] LL |     type Color;
[00:56:42]    |     ----------- ambiguous `Color` from `Box`
[00:56:42] ...
[00:56:42] LL | fn paint<C:BoxCar>(c: C, d: C::Color) {
[00:56:42] 
[00:56:42] error: aborting due to 4 previous errors
[00:56:42] 
[00:56:42] Some errors have detailed explanations: E0191, E0221.
---
[00:56:42] 
[00:56:42] ---- [ui] ui/associated-types/associated-types-incomplete-object.rs stdout ----
[00:56:42] diff of stderr:
[00:56:42] 
[00:56:42] 1 error[E0191]: the value of the associated type `B` (from the trait `Foo`) must be specified
[00:56:42] 3    |
[00:56:42] - LL |     type B;
[00:56:42] -    |     ------- `B` defined here
[00:56:42] - ...
[00:56:42] - ...
[00:56:42] 7 LL |     let b = &42isize as &dyn Foo<A=usize>;
[00:56:42] 8    |                          ^^^^^^^^^^^^^^^^ associated type `B` must be specified
[00:56:42] 
[00:56:42] 
[00:56:42] 10 error[E0191]: the value of the associated type `A` (from the trait `Foo`) must be specified
[00:56:42] 12    |
[00:56:42] - LL |     type A;
[00:56:42] - LL |     type A;
[00:56:42] -    |     ------- `A` defined here
[00:56:42] - ...
[00:56:42] 16 LL |     let c = &42isize as &dyn Foo<B=char>;
[00:56:42] 17    |                          ^^^^^^^^^^^^^^^ associated type `A` must be specified
[00:56:42] 
[00:56:42] 
[00:56:42] 19 error[E0191]: the value of the associated types `A` (from the trait `Foo`), `B` (from the trait `Foo`) must be specified
[00:56:42] 21    |
[00:56:42] - LL |     type A;
[00:56:42] - LL |     type A;
[00:56:42] -    |     ------- `A` defined here
[00:56:42] - LL |     type B;
[00:56:42] -    |     ------- `B` defined here
[00:56:42] - ...
[00:56:42] 27 LL |     let d = &42isize as &dyn Foo;
[00:56:42] 29    |                          |
[00:56:42] 
[00:56:42] 
[00:56:42] The actual stderr differed from the expected stderr.
[00:56:42] The actual stderr differed from the expected stderr.
[00:56:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-incomplete-object/associated-types-incomplete-object.stderr
[00:56:42] To update references, rerun the tests and pass the `--bless` flag
[00:56:42] To only update this specific test, also pass `--test-args associated-types/associated-types-incomplete-object.rs`
[00:56:42] error: 1 errors occurred comparing output.
[00:56:42] status: exit code: 1
[00:56:42] status: exit code: 1
[00:56:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-incomplete-object.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-incomplete-object" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-incomplete-object/auxiliary" "-A" "unused"
[00:56:42] ------------------------------------------
[00:56:42] 
[00:56:42] ------------------------------------------
[00:56:42] stderr:
[00:56:42] stderr:
[00:56:42] ------------------------------------------
[00:56:42] error[E0191]: the value of the associated type `B` (from the trait `Foo`) must be specified
[00:56:42]    |
[00:56:42]    |
[00:56:42] LL |     let b = &42isize as &dyn Foo<A=usize>;
[00:56:42]    |                          ^^^^^^^^^^^^^^^^ associated type `B` must be specified
[00:56:42] 
[00:56:42] error[E0191]: the value of the associated type `A` (from the trait `Foo`) must be specified
[00:56:42]    |
[00:56:42]    |
[00:56:42] LL |     let c = &42isize as &dyn Foo<B=char>;
[00:56:42]    |                          ^^^^^^^^^^^^^^^ associated type `A` must be specified
[00:56:42] 
[00:56:42] error[E0191]: the value of the associated types `A` (from the trait `Foo`), `B` (from the trait `Foo`) must be specified
[00:56:42]    |
[00:56:42]    |
[00:56:42] LL |     let d = &42isize as &dyn Foo;
[00:56:42]    |                          |
[00:56:42]    |                          associated type `A` must be specified
[00:56:42]    |                          associated type `B` must be specified
[00:56:42] 
---
[00:56:42] 
[00:56:42] ---- [ui] ui/error-codes/E0191.rs stdout ----
[00:56:42] diff of stderr:
[00:56:42] 
[00:56:42] 1 error[E0191]: the value of the associated type `Bar` (from the trait `Trait`) must be specified
[00:56:42] 2   --> $DIR/E0191.rs:5:12
[00:56:42] - LL |     type Bar;
[00:56:42] - LL |     type Bar;
[00:56:42] -    |     --------- `Bar` defined here
[00:56:42] - ...
[00:56:42] 7 LL | type Foo = dyn Trait;
[00:56:42] 8    |            ^^^^^^^^^ associated type `Bar` must be specified
[00:56:42] 
[00:56:42] 
[00:56:42] The actual stderr differed from the expected stderr.
[00:56:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0191/E0191.stderr
[00:56:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0191/E0191.stderr
[00:56:42] To update references, rerun the tests and pass the `--bless` flag
[00:56:42] To only update this specific test, also pass `--test-args error-codes/E0191.rs`
[00:56:42] error: 1 errors occurred comparing output.
[00:56:42] status: exit code: 1
[00:56:42] status: exit code: 1
[00:56:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0191.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0191" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0191/auxiliary" "-A" "unused"
[00:56:42] ------------------------------------------
[00:56:42] 
[00:56:42] ------------------------------------------
[00:56:42] stderr:
[00:56:42] stderr:
[00:56:42] ------------------------------------------
[00:56:42] error[E0191]: the value of the associated type `Bar` (from the trait `Trait`) must be specified
[00:56:42]    |
[00:56:42]    |
[00:56:42] LL | type Foo = dyn Trait; //~ ERROR E0191
[00:56:42]    |            ^^^^^^^^^ associated type `Bar` must be specified
[00:56:42] error: aborting due to previous error
[00:56:42] 
[00:56:42] For more information about this error, try `rustc --explain E0191`.
[00:56:42] 
[00:56:42] 
[00:56:42] ------------------------------------------
[00:56:42] 
[00:56:42] 
[00:56:42] ---- [ui] ui/error-codes/E0220.rs stdout ----
[00:56:42] diff of stderr:
[00:56:42] 
[00:56:42] 7 error[E0191]: the value of the associated type `Bar` (from the trait `Trait`) must be specified
[00:56:42] 8   --> $DIR/E0220.rs:5:12
[00:56:42] - LL |     type Bar;
[00:56:42] - LL |     type Bar;
[00:56:42] -    |     --------- `Bar` defined here
[00:56:42] - ...
[00:56:42] 13 LL | type Foo = dyn Trait<F=i32>;
[00:56:42] 14    |            ^^^^^^^^^^^^^^^^ associated type `Bar` must be specified
[00:56:42] 
[00:56:42] 
[00:56:42] The actual stderr differed from the expected stderr.
[00:56:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0220/E0220.stderr
[00:56:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0220/E0220.stderr
[00:56:42] To update references, rerun the tests and pass the `--bless` flag
[00:56:42] To only update this specific test, also pass `--test-args error-codes/E0220.rs`
[00:56:42] error: 1 errors occurred comparing output.
[00:56:42] status: exit code: 1
[00:56:42] status: exit code: 1
[00:56:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0220.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0220" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0220/auxiliary" "-A" "unused"
[00:56:42] ------------------------------------------
[00:56:42] 
[00:56:42] ------------------------------------------
[00:56:42] stderr:
[00:56:42] stderr:
[00:56:42] ------------------------------------------
[00:56:42] error[E0220]: associated type `F` not found for `Trait`
[00:56:42]    |
[00:56:42]    |
[00:56:42] LL | type Foo = dyn Trait<F=i32>; //~ ERROR E0220
[00:56:42]    |                      ^^^^^ associated type `F` not found
[00:56:42] 
[00:56:42] error[E0191]: the value of the associated type `Bar` (from the trait `Trait`) must be specified
[00:56:42]    |
[00:56:42]    |
[00:56:42] LL | type Foo = dyn Trait<F=i32>; //~ ERROR E0220
[00:56:42]    |            ^^^^^^^^^^^^^^^^ associated type `Bar` must be specified
[00:56:42] error: aborting due to 2 previous errors
[00:56:42] 
[00:56:42] Some errors have detailed explanations: E0191, E0220.
[00:56:42] For more information about an error, try `rustc --explain E0191`.
[00:56:42] For more information about an error, try `rustc --explain E0191`.
[00:56:42] 
[00:56:42] ------------------------------------------
[00:56:42] 
[00:56:42] 
[00:56:42] ---- [ui] ui/existential-type/issue-60371.rs stdout ----
[00:56:42] diff of stderr:
[00:56:42] 
[00:56:42] 17              <&() as Bug>
[00:56:42] 18    = note: the return type of a function must have a statically known size
[00:56:42] - error: could not find defining uses
[00:56:42] -   --> $DIR/issue-60371.rs:8:5
[00:56:42] -    |
[00:56:42] -    |
[00:56:42] - LL |     existential type Item: Bug;
[00:56:42] - 
[00:56:42] - error: aborting due to 3 previous errors
[00:56:42] + error: aborting due to 2 previous errors
[00:56:42] 27 
[00:56:42] 27 
[00:56:42] 28 Some errors have detailed explanations: E0277, E0658.
[00:56:42] 29 For more information about an error, try `rustc --explain E0277`.
[00:56:42] 
[00:56:42] 
[00:56:42] The actual stderr differed from the expected stderr.
[00:56:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential-type/issue-60371/issue-60371.stderr
[00:56:42] To update references, rerun the tests and pass the `--bless` flag
[00:56:42] To only update this specific test, also pass `--test-args existential-type/issue-60371.rs`
[00:56:42] error: 1 errors occurred comparing output.
[00:56:42] status: exit code: 1
[00:56:42] status: exit code: 1
[00:56:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/existential-type/issue-60371.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential-type/issue-60371" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential-type/issue-60371/auxiliary" "-A" "unused"
[00:56:42] ------------------------------------------
[00:56:42] 
[00:56:42] ------------------------------------------
[00:56:42] stderr:
[00:56:42] stderr:
[00:56:42] ------------------------------------------
[00:56:42] error[E0658]: existential types are unstable
[00:56:42]   --> /checkout/src/test/ui/existential-type/issue-60371.rs:8:5
[00:56:42]    |
[00:56:42] LL |     existential type Item: Bug; //~ ERROR existential types are unstable
[00:56:42]    |
[00:56:42]    = note: for more information, see https://github.com/rust-lang/rust/issues/34511
[00:56:42]    = help: add #![feature(existential_type)] to the crate attributes to enable
[00:56:42] 
[00:56:42] 
[00:56:42] error[E0277]: the trait bound `(): Bug` is not satisfied
[00:56:42]    |
[00:56:42]    |
[00:56:42] LL |     existential type Item: Bug; //~ ERROR existential types are unstable
[00:56:42]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Bug` is not implemented for `()`
[00:56:42]    = help: the following implementations were found:
[00:56:42]    = help: the following implementations were found:
[00:56:42]              <&() as Bug>
[00:56:42]    = note: the return type of a function must have a statically known size
[00:56:42] error: aborting due to 2 previous errors
[00:56:42] 
[00:56:42] Some errors have detailed explanations: E0277, E0658.
[00:56:42] For more information about an error, try `rustc --explain E0277`.
[00:56:42] For more information about an error, try `rustc --explain E0277`.
[00:56:42] 
[00:56:42] ------------------------------------------
[00:56:42] 
[00:56:42] 
[00:56:42] ---- [ui] ui/issues/issue-19482.rs stdout ----
[00:56:42] diff of stderr:
[00:56:42] 
[00:56:42] 1 error[E0191]: the value of the associated type `A` (from the trait `Foo`) must be specified
[00:56:42] 3    |
[00:56:42] - LL |     type A;
[00:56:42] - LL |     type A;
[00:56:42] -    |     ------- `A` defined here
[00:56:42] - ...
[00:56:42] 7 LL | fn bar(x: &dyn Foo) {}
[00:56:42] 8    |            ^^^^^^^ associated type `A` must be specified
[00:56:42] 
[00:56:42] 
[00:56:42] The actual stderr differed from the expected stderr.
[00:56:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-19482/issue-19482.stderr
[00:56:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-19482/issue-19482.stderr
[00:56:42] To update references, rerun the tests and pass the `--bless` flag
[00:56:42] To only update this specific test, also pass `--test-args issues/issue-19482.rs`
[00:56:42] error: 1 errors occurred comparing output.
[00:56:42] status: exit code: 1
[00:56:42] status: exit code: 1
[00:56:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-19482.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-19482" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-19482/auxiliary" "-A" "unused"
[00:56:42] ------------------------------------------
[00:56:42] 
[00:56:42] ------------------------------------------
[00:56:42] stderr:
[00:56:42] stderr:
[00:56:42] ------------------------------------------
[00:56:42] error[E0191]: the value of the associated type `A` (from the trait `Foo`) must be specified
[00:56:42]    |
[00:56:42] LL | fn bar(x: &dyn Foo) {}
[00:56:42]    |            ^^^^^^^ associated type `A` must be specified
[00:56:42] 
---
[00:56:42] 
[00:56:42] ---- [ui] ui/issues/issue-22434.rs stdout ----
[00:56:42] diff of stderr:
[00:56:42] 
[00:56:42] 1 error[E0191]: the value of the associated type `A` (from the trait `Foo`) must be specified
[00:56:42] 3    |
[00:56:42] - LL |     type A;
[00:56:42] - LL |     type A;
[00:56:42] -    |     ------- `A` defined here
[00:56:42] - ...
[00:56:42] 7 LL | type I<'a> = &'a (dyn Foo + 'a);
[00:56:42] 8    |                   ^^^^^^^^^^^^ associated type `A` must be specified
[00:56:42] 
[00:56:42] 
[00:56:42] The actual stderr differed from the expected stderr.
[00:56:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22434/issue-22434.stderr
[00:56:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22434/issue-22434.stderr
[00:56:42] To update references, rerun the tests and pass the `--bless` flag
[00:56:42] To only update this specific test, also pass `--test-args issues/issue-22434.rs`
[00:56:42] error: 1 errors occurred comparing output.
[00:56:42] status: exit code: 1
[00:56:42] status: exit code: 1
[00:56:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-22434.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22434" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22434/auxiliary" "-A" "unused"
[00:56:42] ------------------------------------------
[00:56:42] 
[00:56:42] ------------------------------------------
[00:56:42] stderr:
[00:56:42] stderr:
[00:56:42] ------------------------------------------
[00:56:42] error[E0191]: the value of the associated type `A` (from the trait `Foo`) must be specified
[00:56:42]    |
[00:56:42]    |
[00:56:42] LL | type I<'a> = &'a (dyn Foo + 'a);
[00:56:42]    |                   ^^^^^^^^^^^^ associated type `A` must be specified
[00:56:42] error: aborting due to previous error
[00:56:42] 
[00:56:42] For more information about this error, try `rustc --explain E0191`.
[00:56:42] 
[00:56:42] 
[00:56:42] ------------------------------------------
[00:56:42] 
[00:56:42] 
[00:56:42] ---- [ui] ui/suggestions/use-type-argument-instead-of-assoc-type.rs stdout ----
[00:56:42] diff of stderr:
[00:56:42] 
[00:56:42] 9 error[E0191]: the value of the associated types `A` (from the trait `T`), `C` (from the trait `T`) must be specified
[00:56:42] 11    |
[00:56:42] - LL |     type A;
[00:56:42] - LL |     type A;
[00:56:42] -    |     ------- `A` defined here
[00:56:42] - LL |     type B;
[00:56:42] - LL |     type C;
[00:56:42] -    |     ------- `C` defined here
[00:56:42] - ...
[00:56:42] 18 LL |     i: Box<dyn T<usize, usize, usize, usize, B=usize>>,
[00:56:42] 20    |            |
[00:56:42] 
[00:56:42] 
[00:56:42] The actual stderr differed from the expected stderr.
[00:56:42] The actual stderr differed from the expected stderr.
[00:56:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/use-type-argument-instead-of-assoc-type/use-type-argument-instead-of-assoc-type.stderr
[00:56:42] To update references, rerun the tests and pass the `--bless` flag
[00:56:42] To only update this specific test, also pass `--test-args suggestions/use-type-argument-instead-of-assoc-type.rs`
[00:56:42] error: 1 errors occurred comparing output.
[00:56:42] status: exit code: 1
[00:56:42] status: exit code: 1
[00:56:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/use-type-argument-instead-of-assoc-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/use-type-argument-instead-of-assoc-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/use-type-argument-instead-of-assoc-type/auxiliary" "-A" "unused"
[00:56:42] ------------------------------------------
[00:56:42] 
[00:56:42] ------------------------------------------
[00:56:42] stderr:
[00:56:42] stderr:
[00:56:42] ------------------------------------------
[00:56:42] error[E0107]: wrong number of type arguments: expected 2, found 4
[00:56:42]   --> /checkout/src/test/ui/suggestions/use-type-argument-instead-of-assoc-type.rs:7:32
[00:56:42]    |
[00:56:42] LL |     i: Box<dyn T<usize, usize, usize, usize, B=usize>>,
[00:56:42]    |                                |
[00:56:42]    |                                unexpected type argument
[00:56:42] 
[00:56:42] 
[00:56:42] error[E0191]: the value of the associated types `A` (from the trait `T`), `C` (from the trait `T`) must be specified
[00:56:42]    |
[00:56:42]    |
[00:56:42] LL |     i: Box<dyn T<usize, usize, usize, usize, B=usize>>,
[00:56:42]    |            |
[00:56:42]    |            associated type `A` must be specified
[00:56:42]    |            associated type `C` must be specified
[00:56:42] help: if you meant to specify the associated types, write
[00:56:42] help: if you meant to specify the associated types, write
[00:56:42]    |
[00:56:42] LL |     i: Box<dyn T<usize, usize, A = usize, C = usize, B=usize>>,
[00:56:42] 
[00:56:42] error: aborting due to 2 previous errors
[00:56:42] 
[00:56:42] Some errors have detailed explanations: E0107, E0191.
---
[00:56:42] 
[00:56:42] ---- [ui] ui/traits/trait-object-with-self-in-projection-output-bad.rs stdout ----
[00:56:42] diff of stderr:
[00:56:42] 
[00:56:42] 1 error[E0191]: the value of the associated type `Output` (from the trait `Base`) must be specified
[00:56:42] 3    |
[00:56:42] - LL |     type Output;
[00:56:42] - LL |     type Output;
[00:56:42] -    |     ------------ `Output` defined here
[00:56:42] - ...
[00:56:42] 7 LL |     let _x: Box<dyn Helper<Target=i32>> = Box::new(2u32);
[00:56:42] 8    |                 ^^^^^^^^^^^^^^^^^^^^^^ associated type `Output` must be specified
[00:56:42] 
[00:56:42] 
[00:56:42] 10 error[E0191]: the value of the associated type `Output` (from the trait `Base`) must be specified
[00:56:42] 12    |
[00:56:42] - LL |     type Output;
[00:56:42] - LL |     type Output;
[00:56:42] -    |     ------------ `Output` defined here
[00:56:42] - ...
[00:56:42] 16 LL |     let _y: Box<dyn NormalizableHelper<Target=i32>> = Box::new(2u32);
[00:56:42] 17    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ associated type `Output` must be specified
[00:56:42] 
[00:56:42] 
[00:56:42] The actual stderr differed from the expected stderr.
[00:56:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-with-self-in-projection-output-bad/trait-object-with-self-in-projection-output-bad.stderr
[00:56:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-with-self-in-projection-output-bad/trait-object-with-self-in-projection-output-bad.stderr
[00:56:42] To update references, rerun the tests and pass the `--bless` flag
[00:56:42] To only update this specific test, also pass `--test-args traits/trait-object-with-self-in-projection-output-bad.rs`
[00:56:42] error: 1 errors occurred comparing output.
[00:56:42] status: exit code: 1
[00:56:42] status: exit code: 1
[00:56:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-object-with-self-in-projection-output-bad.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-with-self-in-projection-output-bad" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-with-self-in-projection-output-bad/auxiliary" "-A" "unused"
[00:56:42] ------------------------------------------
[00:56:42] 
[00:56:42] ------------------------------------------
[00:56:42] stderr:
[00:56:42] stderr:
[00:56:42] ------------------------------------------
[00:56:42] error[E0191]: the value of the associated type `Output` (from the trait `Base`) must be specified
[00:56:42]    |
[00:56:42]    |
[00:56:42] LL |     let _x: Box<dyn Helper<Target=i32>> = Box::new(2u32);
[00:56:42]    |                 ^^^^^^^^^^^^^^^^^^^^^^ associated type `Output` must be specified
[00:56:42] 
[00:56:42] error[E0191]: the value of the associated type `Output` (from the trait `Base`) must be specified
[00:56:42]    |
[00:56:42]    |
[00:56:42] LL |     let _y: Box<dyn NormalizableHelper<Target=i32>> = Box::new(2u32);
[00:56:42]    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ associated type `Output` must be specified
[00:56:42] error: aborting due to 2 previous errors
[00:56:42] 
[00:56:42] For more information about this error, try `rustc --explain E0191`.
[00:56:42] 
[00:56:42] 
[00:56:42] ------------------------------------------
[00:56:42] 
[00:56:42] 
[00:56:42] ---- [ui] ui/wf/wf-packed-on-proj-of-type-as-unimpl-trait.rs stdout ----
[00:56:42] 
[00:56:42] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:42] status: exit code: 101
[00:56:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/wf/wf-packed-on-proj-of-type-as-unimpl-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-packed-on-proj-of-type-as-unimpl-trait" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-packed-on-proj-of-type-as-unimpl-trait/auxiliary" "-A" "unused"
[00:56:42] ------------------------------------------
[00:56:42] 
[00:56:42] ------------------------------------------
[00:56:42] stderr:
[00:56:42] stderr:
[00:56:42] ------------------------------------------
[00:56:42] thread 'rustc' panicked at 'assertion failed: !ty.needs_infer()', src/librustc/ty/util.rs:1001:5
[00:56:42] 
[00:56:42] error: internal compiler error: unexpected panic
[00:56:42] 
[00:56:42] note: the compiler unexpectedly panicked. this is a bug.
[00:56:42] note: the compiler unexpectedly panicked. this is a bug.
[00:56:42] 
[00:56:42] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:56:42] 
[00:56:42] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[00:56:42] 
[00:56:42] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
[00:56:42] 
[00:56:42] ------------------------------------------
[00:56:42] 
[00:56:42] 
---
[00:56:42] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[00:56:42] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:56:42] 
[00:56:42] 
[00:56:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:42] 
[00:56:42] 
[00:56:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:42] Build completed unsuccessfully in 0:52:03
---
travis_time:end:00f5921a:start=1559302972010279353,finish=1559302972017082231,duration=6802878
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:26296601
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:28acab73
$ dmesg | grep -i kill

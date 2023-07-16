plain
.........................................................iii........................................ 12600/12662
..............................................................
failures:

---- [ui] ui/rfc-2632-const-trait-impl/staged-api.rs#unstable stdout ----


1 error: `<Foo as staged_api::MyTrait>::func` is not yet stable as a const fn
-   --> $DIR/staged-api.rs:33:5
3    |
4 LL |     Foo::func();
5    |     ^^^^^^^^^^^


7    = help: add `#![feature(foo)]` to the crate attributes to enable
8 
9 error: `<Foo as staged_api::MyTrait>::func` is not yet stable as a const fn
-   --> $DIR/staged-api.rs:41:5
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
11    |
12 LL |     Foo::func();
13    |     ^^^^^^^^^^^
13    |     ^^^^^^^^^^^


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/staged-api.unstable/staged-api.unstable.stderr
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/staged-api.rs`


error in revision `unstable`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/staged-api.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "unstable" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/staged-api.unstable" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/staged-api.unstable/auxiliary"
stdout: none
--- stderr -------------------------------
error: `<Foo as staged_api::MyTrait>::func` is not yet stable as a const fn
   |
LL |     Foo::func();
   |     ^^^^^^^^^^^
   |
   |
   = help: add `#![feature(foo)]` to the crate attributes to enable

error: `<Foo as staged_api::MyTrait>::func` is not yet stable as a const fn
   |
LL |     Foo::func();
   |     ^^^^^^^^^^^
   |
   |
   = help: add `#![feature(foo)]` to the crate attributes to enable
error: aborting due to 2 previous errors
------------------------------------------



plain
....................................................iii................................. 13816/13890
..........................................................................
failures:

---- [ui] src/test/ui/typeck/hang-in-overflow.rs stdout ----

6    |
6    |
7    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`hang_in_overflow`)
8    = note: required for `...` to implement `...`
-    = note: the full type name has been written to '$TEST_BUILD_DIR/typeck/hang-in-overflow/hang-in-overflow.long-type-hash.txt'
10    = note: 127 redundant requirements hidden
11    = note: required for `...` to implement `...`
12    = note: the full type name has been written to '$TEST_BUILD_DIR/typeck/hang-in-overflow/hang-in-overflow.long-type-hash.txt'

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/hang-in-overflow/hang-in-overflow.stderr
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
To only update this specific test, also pass `--test-args typeck/hang-in-overflow.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/hang-in-overflow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/hang-in-overflow" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/hang-in-overflow/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0275]: overflow evaluating the requirement `&mut [&mut [&[&[&_; _]; _]; _]; _]: PartialEq<&_>`
  --> /checkout/src/test/ui/typeck/hang-in-overflow.rs:9:5
   |
LL |     foo::<_>();
   |
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`hang_in_overflow`)
   = note: required for `[&mut [&mut [&[&[&_; _]; _]; _]; _]; _]` to implement `PartialEq<[&_; _]>`
   = note: 127 redundant requirements hidden
   = note: required for `Vec<[[[Vec<[...; 1], ...>; 1]; 1]; 1]>` to implement `PartialEq<Vec<[[[&[[[[[[[[&mut [[[[&[[[&[&mut [[[[&[[[[&mut [[[&[&[[[&mut [&mut [[[&[&[[&[&mut [&mut [[[&[&[[[&mut [&mut [[&[&[&[[&mut [&mut [&mut [[&[&[&[&[&mut [&mut [&mut [[&[&[&[[&mut [&mut [&mut [&[&[&[&[&mut [&mut [&mut [&mut [&[&[&_; _]; _]; _]; _]; _]]; _]; _]; _]]; _]; _]; _]; _]; _]; _]; _]; _]; _]; _]; _]]; _]; _]; _]; 1]; _]; _]]; 1]; _]; _]]; 1]; _]; _]; _]; 1]; _]; _]; _]; 1]; _]; _]]; 1]; _]; _]; 1]; 1]; _]]; 1]; 1]; _]]; 1]; 1]; _]; _]; 1]; 1]; _]; _]; 1]; 1]; _]]; 1]; 1]; _]; 1]; 1]; 1]]; 1]; 1]; 1]; _]; 1]; 1]; 1]]; 1]; 1]; 1], _>>`
   = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/hang-in-overflow/hang-in-overflow.long-type-3530725882784848861.txt'
note: required by a bound in `foo`
  --> /checkout/src/test/ui/typeck/hang-in-overflow.rs:15:28
   |
LL | fn foo<B>()
LL | where
LL | where
LL |     Vec<[[[B; 1]; 1]; 1]>: PartialEq<B>,
   |                            ^^^^^^^^^^^^ required by this bound in `foo`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0275`.
------------------------------------------

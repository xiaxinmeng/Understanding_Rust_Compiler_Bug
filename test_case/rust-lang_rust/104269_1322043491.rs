`
ailures:

---- [ui] src/test/ui/typeck/hang-in-overflow.rs stdout ----
diff of stderr:

8	   = note: required for `...` to implement `...`
9	   = note: 127 redundant requirements hidden
10	   = note: required for `...` to implement `...`
+	   = note: the full type name has been written to '$TEST_BUILD_DIR/typeck/hang-in-overflow/hang-in-overflow.long-type-13273308321832961787.txt'
11	note: required by a bound in `foo`
12	  --> $DIR/hang-in-overflow.rs:14:28
13	   |


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/hang-in-overflow/hang-in-overflow.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args typeck/hang-in-overflow.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/hang-in-overflow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/hang-in-overflow" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/hang-in-overflow/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0275]: overflow evaluating the requirement `&&[&&mut [&&[&[&_]]; _]]: PartialEq<&_>`
  --> /checkout/src/test/ui/typeck/hang-in-overflow.rs:8:5
   |
LL |     foo::<_>();
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |     ^^^^^^^^
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`hang_in_overflow`)
   = note: required for `[&&[&&mut [&&[&[&_]]; _]]]` to implement `PartialEq<[&_]>`
   = note: 127 redundant requirements hidden
   = note: required for `Vec<[[[&[...; _]; 1]; 1]; 1]>` to implement `PartialEq<&[&mut [&mut [&mut [&&mut [&[&[&[&&[&[&[&[&[&[&[&[&&mut [&[&[[&&[&[&[&mut [&&[&[&[&mut [&&[&[&[&&mut [&&[&[&&mut [&&[&&[&[&&[&&[&&[&[&[&&[&&[&[&&mut [&[&&[&[&&[&&mut [&&[&[&&[&&mut [&&[&[&_]]; _]]]]]]]]; _]]]]]; _]]]]]]]]]]]]; _]]]; _]]]]]]]]; _]]]]]]]; _]]]]]]]]]]]]; _]>`
   = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/hang-in-overflow/hang-in-overflow.long-type-13273308321832961787.txt'
note: required by a bound in `foo`
  --> /checkout/src/test/ui/typeck/hang-in-overflow.rs:14:28
   |
LL | fn foo<B>()
   |    --- required by a bound in this
LL | where
LL |     Vec<[[[B; 1]; 1]; 1]>: PartialEq<B>,
   |                            ^^^^^^^^^^^^ required by this bound in `foo`

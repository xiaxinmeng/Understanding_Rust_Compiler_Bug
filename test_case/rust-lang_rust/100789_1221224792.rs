plain
....................i................................................................... 1496/13400
........................................................................................ 1584/13400
........................................................................................ 1672/13400
..........................................................i......ii..................... 1760/13400
...........................................................F.F.......................... 1848/13400
.................................i...................................................... 2024/13400
........................................................................................ 2112/13400
........................................................................................ 2200/13400
........................................................................................ 2288/13400
---

---- [ui] src/test/ui/coherence/coherence-negative-outlives-lifetimes.rs#stock stdout ----
diff of stderr:

1 error[E0119]: conflicting implementations of trait `MyTrait<'_>` for type `&_`
-   --> $DIR/coherence-negative-outlives-lifetimes.rs:11:1
+   --> $DIR/coherence-negative-outlives-lifetimes.rs:14:1
3    |
4 LL | impl<'a, T: MyPredicate<'a>> MyTrait<'a> for T {}
5    | ---------------------------------------------- first implementation here

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-negative-outlives-lifetimes.stock/coherence-negative-outlives-lifetimes.stock.stderr
To only update this specific test, also pass `--test-args coherence/coherence-negative-outlives-lifetimes.rs`


error in revision `stock`: 1 errors occurred comparing output.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence-negative-outlives-lifetimes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "stock" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-negative-outlives-lifetimes.stock" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-negative-outlives-lifetimes.stock/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0119]: conflicting implementations of trait `MyTrait<'_>` for type `&_`
   |
   |
LL | impl<'a, T: MyPredicate<'a>> MyTrait<'a> for T {}
   | ---------------------------------------------- first implementation here
LL | impl<'a, T> MyTrait<'a> for &'a T {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&_`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0119`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/coherence/coherence-negative-outlives-lifetimes.rs#with_negative_coherence stdout ----
diff of stderr:

1 error[E0119]: conflicting implementations of trait `MyTrait<'_>` for type `&_`
-   --> $DIR/coherence-negative-outlives-lifetimes.rs:11:1
+   --> $DIR/coherence-negative-outlives-lifetimes.rs:14:1
3    |
4 LL | impl<'a, T: MyPredicate<'a>> MyTrait<'a> for T {}
5    | ---------------------------------------------- first implementation here

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-negative-outlives-lifetimes.with_negative_coherence/coherence-negative-outlives-lifetimes.with_negative_coherence.stderr
To only update this specific test, also pass `--test-args coherence/coherence-negative-outlives-lifetimes.rs`


error in revision `with_negative_coherence`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence-negative-outlives-lifetimes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "with_negative_coherence" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-negative-outlives-lifetimes.with_negative_coherence" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-negative-outlives-lifetimes.with_negative_coherence/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0119]: conflicting implementations of trait `MyTrait<'_>` for type `&_`
   |
   |
LL | impl<'a, T: MyPredicate<'a>> MyTrait<'a> for T {}
   | ---------------------------------------------- first implementation here
LL | impl<'a, T> MyTrait<'a> for &'a T {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&_`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0119`.
------------------------------------------

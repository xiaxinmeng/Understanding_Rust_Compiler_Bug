plain
........................................................................................ 4664/13760
...............i........................................................................ 4752/13760
........................................................................................ 4840/13760
........................................................................................ 4928/13760
...........................................F.......F.................................... 5016/13760
........................................................................................ 5192/13760
........................................................................................ 5280/13760
...................................................................i.................... 5368/13760
.................................................i...................................... 5456/13760
---
failures:

---- [ui] src/test/ui/impl-trait/in-trait/generics-mismatch.rs stdout ----
normalized stderr:
error[E0049]: method `bar` has 1 type parameter but its trait declaration has 0 type parameters
   |
LL |     fn bar(&self) -> impl Sized;
   |           - expected 0 type parameters
...
...
LL |     fn bar<T>(&self) {}
   |            ^ found 1 type parameter
error: aborting due to previous error

For more information about this error, try `rustc --explain E0049`.

---
To only update this specific test, also pass `--test-args impl-trait/in-trait/generics-mismatch.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/in-trait/generics-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/in-trait/generics-mismatch" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/in-trait/generics-mismatch/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
stdout: none
--- stderr -------------------------------
error[E0049]: method `bar` has 1 type parameter but its trait declaration has 0 type parameters
   |
LL |     fn bar(&self) -> impl Sized;
   |           - expected 0 type parameters
...
...
LL |     fn bar<T>(&self) {}
   |            ^ found 1 type parameter
error: aborting due to previous error

For more information about this error, try `rustc --explain E0049`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/impl-trait/in-trait/specialization-substs-remap.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/in-trait/specialization-substs-remap.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/in-trait/specialization-substs-remap" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/in-trait/specialization-substs-remap/auxiliary"
stdout: none
stderr: none


failures:
    [ui] src/test/ui/impl-trait/in-trait/generics-mismatch.rs

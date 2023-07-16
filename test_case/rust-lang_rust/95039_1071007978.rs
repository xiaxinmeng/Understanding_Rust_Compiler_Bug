plain

- error[E0119]: conflicting implementations of trait `C` for type `u32`
-   --> $DIR/coherence-overlap-negate-alias-strict.rs:15:1
-    |
- LL | impl<T: AB> C for T {}
-    | ------------------- first implementation here
- LL | impl C for u32 {}
-    | ^^^^^^^^^^^^^^ conflicting implementation for `u32`
- error: aborting due to previous error
- 
- For more information about this error, try `rustc --explain E0119`.
- 
---
To only update this specific test, also pass `--test-args coherence/coherence-overlap-negate-alias-strict.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence-overlap-negate-alias-strict.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-overlap-negate-alias-strict" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-overlap-negate-alias-strict/auxiliary"
stdout: none
stderr: none


failures:
    [ui] ui/coherence/coherence-overlap-negate-alias-strict.rs

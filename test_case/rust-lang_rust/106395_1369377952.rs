plain
---- [ui] src/test/ui/const-generics/defaults/wfness.rs stdout ----
diff of stderr:

12    |
13    = help: the trait `Trait<3>` is implemented for `()`
14 
+ error[E0277]: the trait bound `(): Traitor<u32, 2>` is not satisfied
+   --> $DIR/wfness.rs:14:9
+    |
+ LL |     (): Traitor<T, N>;
+    |         ^^^^^^^^^^^^^ the trait `Traitor<u32, 2>` is not implemented for `()`
+ 
15 error[E0277]: the trait bound `(): Trait<1>` is not satisfied
16   --> $DIR/wfness.rs:18:13


28 LL |     (): Trait<N>;
29    |         ^^^^^^^^ required by this bound in `WhereClause`
- error: aborting due to 3 previous errors
+ error: aborting due to 4 previous errors
32 
33 Some errors have detailed explanations: E0080, E0277.
---
To only update this specific test, also pass `--test-args const-generics/defaults/wfness.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/defaults/wfness.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/wfness" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/wfness/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/defaults/wfness.rs:1:33
   |
   |
LL | struct Ooopsies<const N: u8 = { u8::MAX + 1 }>;
   |                                 ^^^^^^^^^^^ attempt to compute `u8::MAX + 1_u8`, which would overflow

error[E0277]: the trait bound `(): Trait<2>` is not satisfied
   |
   |
LL |     (): Trait<N>;
   |         ^^^^^^^^ the trait `Trait<2>` is not implemented for `()`
   |
   = help: the trait `Trait<3>` is implemented for `()`

error[E0277]: the trait bound `(): Traitor<u32, 2>` is not satisfied
   |
   |
LL |     (): Traitor<T, N>;
   |         ^^^^^^^^^^^^^ the trait `Traitor<u32, 2>` is not implemented for `()`

error[E0277]: the trait bound `(): Trait<1>` is not satisfied
   |
   |
LL | fn foo() -> DependentDefaultWfness {
   |             ^^^^^^^^^^^^^^^^^^^^^^ the trait `Trait<1>` is not implemented for `()`
   |
   = help: the trait `Trait<3>` is implemented for `()`
note: required by a bound in `WhereClause`
   |
   |
LL | struct WhereClause<const N: u8 = 2>
LL | where
LL | where
LL |     (): Trait<N>;
   |         ^^^^^^^^ required by this bound in `WhereClause`
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0080, E0277.
For more information about an error, try `rustc --explain E0080`.
For more information about an error, try `rustc --explain E0080`.
------------------------------------------


---- [ui] src/test/ui/defaults-well-formedness.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/defaults-well-formedness.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/defaults-well-formedness/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/defaults-well-formedness/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `TwoParams<u32, i32>: Marker` is not satisfied
   |
   |
LL | struct BogusTogether<T = u32, U = i32>(T, U) where TwoParams<T, U>: Marker;
   |                                                                     ^^^^^^ the trait `Marker` is not implemented for `TwoParams<u32, i32>`
   |
   = help: the trait `Marker` is implemented for `TwoParams<i32, i32>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------

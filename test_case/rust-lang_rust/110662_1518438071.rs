plain

---- [ui] tests/ui/transmutability/region-infer.rs stdout ----
diff of stderr:

- error[E0277]: `()` cannot be safely transmuted into `W<'_>` in the defining scope of `Context`
-   --> $DIR/region-infer.rs:20:5
- LL |     test();
- LL |     test();
-    |     ^^^^ `W<'_>` does not have a well-specified layout
- note: required by a bound in `test`
- note: required by a bound in `test`
+ error[E0283]: type annotations needed: cannot satisfy `W<'a>: BikeshedIntrinsicFrom<(), Context, Assume { alignment: true, lifetimes: true, safety: true, validity: true }>`
9    |
9    |
- LL |   fn test<'a>()
- LL |   where
- LL |   where
13 LL |       W<'a>: BikeshedIntrinsicFrom<
15 LL | |             (),

16 LL | |             Context,
16 LL | |             Context,
17 LL | |             { Assume { alignment: true, lifetimes: true, safety: true, validity: true } },
18 LL | |         >,
-    | |_________^ required by this bound in `test`
+    |
+    |
+    = note: cannot satisfy `W<'a>: BikeshedIntrinsicFrom<(), Context, Assume { alignment: true, lifetimes: true, safety: true, validity: true }>`
21 error: aborting due to previous error
22 

- For more information about this error, try `rustc --explain E0277`.
---
To only update this specific test, also pass `--test-args transmutability/region-infer.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/transmutability/region-infer.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmutability/region-infer" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmutability/region-infer/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0283]: type annotations needed: cannot satisfy `W<'a>: BikeshedIntrinsicFrom<(), Context, Assume { alignment: true, lifetimes: true, safety: true, validity: true }>`
  --> fake-test-src-base/transmutability/region-infer.rs:11:12
   |
LL |       W<'a>: BikeshedIntrinsicFrom<
LL | |             (),
LL | |             Context,
LL | |             Context,
LL | |             { Assume { alignment: true, lifetimes: true, safety: true, validity: true } },
LL | |         >,
   |
   |
   = note: cannot satisfy `W<'a>: BikeshedIntrinsicFrom<(), Context, Assume { alignment: true, lifetimes: true, safety: true, validity: true }>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0283`.
------------------------------------------

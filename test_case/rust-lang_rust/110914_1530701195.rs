plain
---- [ui] tests/ui/suggestions/missing-assoc-fn-applicable-suggestions.rs stdout ----
diff of stderr:

6    |
7    = help: implement the missing item: `type Type = /* Type */;`
8    = help: implement the missing item: `fn bar<T>(_: T) -> Self { todo!() }`
-    = help: implement the missing item: `fn baz<T>(_: T) -> Self where T: TraitB, <T as TraitB>::Item: Copy { todo!() }`
+    = help: implement the missing item: `fn baz<T>(_: T) -> Self where <T as TraitB>::Item: Copy, T: TraitB { todo!() }`
10    = help: implement the missing item: `const A: usize = 42;`
12 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-assoc-fn-applicable-suggestions/missing-assoc-fn-applicable-suggestions.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/missing-assoc-fn-applicable-suggestions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/suggestions/missing-assoc-fn-applicable-suggestions.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-assoc-fn-applicable-suggestions" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=x86_64-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-assoc-fn-applicable-suggestions/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0046]: not all trait items implemented, missing: `Type`, `bar`, `baz`, `A`
  --> fake-test-src-base/suggestions/missing-assoc-fn-applicable-suggestions.rs:7:1
   |
LL | impl TraitA<()> for S {
   | ^^^^^^^^^^^^^^^^^^^^^ missing `Type`, `bar`, `baz`, `A` in implementation
   |
   = help: implement the missing item: `type Type = /* Type */;`
   = help: implement the missing item: `fn bar<T>(_: T) -> Self { todo!() }`
   = help: implement the missing item: `fn baz<T>(_: T) -> Self where <T as TraitB>::Item: Copy, T: TraitB { todo!() }`
   = help: implement the missing item: `const A: usize = 42;`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0046`.
------------------------------------------

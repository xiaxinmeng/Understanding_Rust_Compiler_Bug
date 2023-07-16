plain
---- [ui] tests/ui/traits/new-solver/structural-resolve-field.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/traits/new-solver/structural-resolve-field.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/structural-resolve-field" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/structural-resolve-field/auxiliary" "-Ztrait-solver=next"
stdout: none
--- stderr -------------------------------
error[E0599]: no function or associated item named `default` found for array `[Foo; 1]` in the current scope
  --> fake-test-src-base/traits/new-solver/structural-resolve-field.rs:10:30
   |
LL |     let mut xs = <[Foo; 1]>::default();
   |                              ^^^^^^^ function or associated item not found in `[Foo; 1]`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
------------------------------------------

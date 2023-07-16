plain
........

failures:

---- [ui] tests/ui/const_prop/unsized-local-ice.rs stdout ----
error: test compilation failed although it shouldn't!
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/const_prop/unsized-local-ice.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const_prop/unsized-local-ice" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const_prop/unsized-local-ice/auxiliary"
stdout: none
error[E0601]: `main` function not found in crate `unsized_local_ice`
  --> fake-test-src-base/const_prop/unsized-local-ice.rs:7:2
   |
LL | }
LL | }
   |  ^ consider adding a `main` function to `fake-test-src-base/const_prop/unsized-local-ice.rs`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0601`.
------------------------------------------

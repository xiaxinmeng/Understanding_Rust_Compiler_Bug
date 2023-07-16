plain
---- [ui] tests/ui/native-library-link-flags/mix-bundle-and-whole-archive.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/native-library-link-flags/mix-bundle-and-whole-archive.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/native-library-link-flags/mix-bundle-and-whole-archive" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/native-library-link-flags/mix-bundle-and-whole-archive/auxiliary" "-l" "static:+bundle,+whole-archive=rust_test_helpers" "--crate-type" "rlib" "--print" "link-args"
stdout: none
--- stderr -------------------------------
error: could not find native static library `rust_test_helpers`, perhaps an -L flag is missing?
error: aborting due to previous error
------------------------------------------



---- [ui] tests/ui/native-library-link-flags/mix-bundle-and-whole-archive-link-attr.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/native-library-link-flags/mix-bundle-and-whole-archive-link-attr.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/native-library-link-flags/mix-bundle-and-whole-archive-link-attr" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/native-library-link-flags/mix-bundle-and-whole-archive-link-attr/auxiliary" "--crate-type" "rlib"
stdout: none
--- stderr -------------------------------
error: could not find native static library `rust_test_helpers`, perhaps an -L flag is missing?
error: aborting due to previous error
------------------------------------------



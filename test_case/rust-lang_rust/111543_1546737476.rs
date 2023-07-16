plain
failures:

---- [ui] tests/ui/process/process-panic-after-fork.rs stdout ----
normalized stderr:
warning: calls to `std::str::from_utf8` with a invalid literal -> always return an error
   |
   |
LL |     one(&|| { std::str::from_utf8(b"\xff").unwrap(); });
   |                                   |
   |                                   |
   |                                   the literal was valid UTF-8 up to the 0 bytes
   |
   = note: `#[warn(invalid_from_utf8)]` on by default
warning: 1 warning emitted



---
To only update this specific test, also pass `--test-args process/process-panic-after-fork.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/process/process-panic-after-fork.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/process-panic-after-fork/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/process-panic-after-fork/auxiliary"
stdout: none
--- stderr -------------------------------
warning: calls to `std::str::from_utf8` with a invalid literal -> always return an error
  --> fake-test-src-base/process/process-panic-after-fork.rs:184:15
   |
LL |     one(&|| { std::str::from_utf8(b"\xff").unwrap(); });
   |                                   |
   |                                   |
   |                                   the literal was valid UTF-8 up to the 0 bytes
   |
   = note: `#[warn(invalid_from_utf8)]` on by default
warning: 1 warning emitted
------------------------------------------



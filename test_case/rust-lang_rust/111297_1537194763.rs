plain
diff of stderr:

18              Cow<'a, str>
19            and 42 others
20 note: required by a bound in `DiagnosticBuilder::<'a, G>::set_arg`
-   --> $COMPILER_DIR/rustc_errors/src/diagnostic_builder.rs:747:5
+   --> $COMPILER_DIR/rustc_errors/src/diagnostic_builder.rs:748:5
22    = note: this error originates in the macro `forward` (in Nightly builds, run with -Z macro-backtrace for more info)
23 
24 error[E0277]: the trait bound `NotIntoDiagnosticArg: IntoDiagnosticArg` is not satisfied

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/diagnostic-derive-doc-comment-field/diagnostic-derive-doc-comment-field.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args session-diagnostic/diagnostic-derive-doc-comment-field.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
Build completed unsuccessfully in 0:11:46
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui-fulldeps/session-diagnostic/diagnostic-derive-doc-comment-field.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui-fulldeps=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/diagnostic-derive-doc-comment-field" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-diagnostic/diagnostic-derive-doc-comment-field/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `NotIntoDiagnosticArg: IntoDiagnosticArg` is not satisfied
  --> fake-test-src-base/session-diagnostic/diagnostic-derive-doc-comment-field.rs:35:10
LL | #[derive(Diagnostic)]
   |          ---------- required by a bound introduced by this call
...
...
LL |     arg: NotIntoDiagnosticArg,
   |          ^^^^^^^^^^^^^^^^^^^^ the trait `IntoDiagnosticArg` is not implemented for `NotIntoDiagnosticArg`
   |
   = help: the following other types implement trait `IntoDiagnosticArg`:
             &'a T
             &'a std::path::Path
             &'a str
             &rustc_target::spec::TargetTriple
             Box<(dyn std::error::Error + 'static)>
             CString
             CguReuse
             Cow<'a, str>
           and 42 others
note: required by a bound in `DiagnosticBuilder::<'a, G>::set_arg`
  --> /rustc/FAKE_PREFIX/compiler/rustc_errors/src/diagnostic_builder.rs:748:5
   = note: this error originates in the macro `forward` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `NotIntoDiagnosticArg: IntoDiagnosticArg` is not satisfied
  --> fake-test-src-base/session-diagnostic/diagnostic-derive-doc-comment-field.rs:45:10
LL | #[derive(Subdiagnostic)]
   |          ------------- required by a bound introduced by this call
...
...
LL |     arg: NotIntoDiagnosticArg,
   |          ^^^^^^^^^^^^^^^^^^^^ the trait `IntoDiagnosticArg` is not implemented for `NotIntoDiagnosticArg`
   |
   = help: the following other types implement trait `IntoDiagnosticArg`:
             &'a T
             &'a std::path::Path
             &'a str
             &rustc_target::spec::TargetTriple
             Box<(dyn std::error::Error + 'static)>
             CString
             CguReuse
             Cow<'a, str>
note: required by a bound in `Diagnostic::set_arg`
  --> /rustc/FAKE_PREFIX/compiler/rustc_errors/src/diagnostic.rs:964:5

error: aborting due to 2 previous errors

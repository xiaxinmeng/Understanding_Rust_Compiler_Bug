\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0411.rs","byte_start":484,"byte_end":488,"line_start":12,"line_end":12,"column_start":6,"column_end":10,"is_primary":true,"text":[{"text":"    <Self>::foo; //~ ERROR E0411","highlight_start":6,"highlight_end":10}],"label":"`Self` is only available in impls and traits","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0411]: cannot find type `Self` in this scope\n  --> /checkout/src/test/ui/error-codes/E0411.rs:12:6\n   |\nLL |     <Self>::foo; //~ ERROR E0411\n   |      ^^^^ `Self` is only available in impls and traits\n\n"}
[00:49:44] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:49:44] {"message":"For more information about this error, try `rustc --explain E0411`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0411`.\n"}
[00:49:44] ------------------------------------------
[00:49:44] 
[00:49:44] thread '[ui] ui/error-codes/E0411.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3166:9
[00:49:44] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:49:44] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:49:44] 
[00:49:44] ---- [ui] ui/feature-gates/feature-gate-panic-implementation.rs stdout ----
[00:49:44] diff of stderr:
[00:49:44] 
[00:49:44] 1 error[E0658]: #[panic_implementation] is an unstable feature (see issue #44489)
[00:49:44] -   --> $DIR/feature-gate-panic-implementation.rs:18:1
[00:49:44] +   --> $DIR/feature-gate-panic-implementation.rs:16:1
[00:49:44] 3    |
[00:49:44] 4 LL | #[panic_implementation\nAn unstable feature was used.\n\nErroneous code example:\n\n
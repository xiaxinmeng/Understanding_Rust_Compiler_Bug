\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2126-crate-paths/crate-path-non-absolute.rs","byte_start":610,"byte_end":615,"line_start":18,"line_end":18,"column_start":20,"column_end":25,"is_primary":true,"text":[{"text":"        let s1 = ::crate::S; //~ ERROR failed to resolve","highlight_start":20,"highlight_end":25}],"label":"global paths cannot start with `crate`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0433]: failed to resolve. global paths cannot start with `crate`\n  --> /checkout/src/test/ui/rfc-2126-crate-paths/crate-path-non-absolute.rs:18:20\n   |\nLL |         let s1 = ::crate::S; //~ ERROR failed to resolve\n   |                    ^^^^^ global paths cannot start with `crate`\n\n"}
[00:47:59] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:47:59] {"message":"For more information about this error, try `rustc --explain E0433`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0433`.\n"}
[00:47:59] ------------------------------------------
[00:47:59] 
[00:47:59] thread '[ui] ui/rfc-2126-crate-paths/crate-path-non-absolute.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3166:9
[00:47:59] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:59] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:59] 
[00:47:59] ---- [ui] ui/rfc-2126-crate-paths/keyword-crate-as-identifier.rs stdout ----
[00:47:59] diff of stderr:
[00:47:59] 
[00:47:59] - error[E0433]: failed to resolve. `crate` in paths can only be used in start position
[00:47:59] + error[E0532]: expected unit struct/variant or constant, found module `crate`
[00:47:59] 2   --> $DIR/keyword-crate-as-identifier.rs:14:9
[00:47:59] 3    |
[00:47:59] - LL |     let crate = 0; //~ ERROR failed to resolve. `crate` in paths can only be used in start position
[00:47:59] -    |         ^^^^^ `crate` in paths can only be used in start position
[00:47:59] + LL |     let crate = 0;
[00:47:59] 6 
[00:47:59] 7 error: aborting due to previous error
[00:47:59] 8 
[00:47:59] 
[00:47:59] 
[00:47:59] - For more information about this error, try `rustc --explain E0433`.
[00:47:59] + For more information about this error, try `rustc --explain E0532`.
[00:47:59] 10 
[00:47:59] 
[00:47:59] 
[00:47:59] The actual stderr differed from the expected stderr.
[00:47:59] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2126-crate-paths/keyword-crate-as-identifier/keyword-crate-as-identifier.stderr
[00:47:59] To update references, rerun the tests and pass the `--bless` flag
[00:47:59] To only update this specific test, also pass `--test-args rfc-2126-crate-paths/keyword-crate-as-identifier.rs`
[00:47:59] error: 1 errors occurred comparing output.
[00:47:59] status: exit code: 1
[00:47:59] status: exit code: 1
[00:47:59] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2126-crate-paths/keyword-crate-as-identifier.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2126-crate-paths/keyword-crate-as-identifier/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2126-crate-paths/keyword-crate-as-identifier/auxiliary" "-A" "unused"
[00:47:59] ------------------------------------------
[00:47:59] 
[00:47:59] ------------------------------------------
[00:47:59] stderr:
[00:47:59] stderr:
[00:47:59] ------------------------------------------
[00:47:59] {"message":"expected unit struct/variant or constant, found module `crate`","code":{"code":"E0532","explanation":"\nPattern arm did not match expected kind.\n\nErroneous code example:\n\n
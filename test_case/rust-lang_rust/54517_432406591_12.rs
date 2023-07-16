\n\nIf the item you are importing is not defined in some super-module of the\ncurrent module, then it must also be declared as public (e.g., `pub fn`).\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/ambiguous-builtin-attrs-test.rs","byte_start":308,"byte_end":319,"line_start":19,"line_end":19,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    NonExistent; //~ ERROR cannot find value `NonExistent` in this scope","highlight_start":5,"highlight_end":16}],"label":"not found in this scope","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0425]: cannot find value `NonExistent` in this scope\n  --> /checkout/src/test/ui-fulldeps/proc-macro/ambiguous-builtin-attrs-test.rs:19:5\n   |\nLL |     NonExistent; //~ ERROR cannot find value `NonExistent` in this scope\n   |     ^^^^^^^^^^^ not found in this scope\n\n"}
[01:04:51] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[01:04:51] {"message":"Some errors occurred: E0425, E0428.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0425, E0428.\n"}
[01:04:51] {"message":"For more information about an error, try `rustc --explain E0425`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0425`.\n"}
[01:04:51] ------------------------------------------
[01:04:51] 
[01:04:51] thread '[ui] ui-fulldeps/proc-macro/ambiguous-builtin-attrs-test.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:04:51] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:04:51] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:04:51] 
[01:04:51] ---- [ui] ui-fulldeps/proc-macro/ambiguous-builtin-attrs.rs stdout ----
[01:04:51] diff of stderr:
[01:04:51] 
[01:04:51] + error[E0428]: the name `Test` is defined multiple times
[01:04:51] +   --> $DIR/ambiguous-builtin-attrs.rs:17:1
[01:04:51] +    |
[01:04:51] + LL | #[test] // OK, shadowed
[01:04:51] +    | ------- previous definition of the type `Test` here
[01:04:51] + ...
[01:04:51] + LL | #[bench] // OK, shadowed
[01:04:51] +    | ^^^^^^^^ `Test` redefined here
[01:04:51] +    |
[01:04:51] +    = note: `Test` must be defined only once in the type namespace of this module
[01:04:51] + 
[01:04:51] 1 error[E0659]: `repr` is ambiguous
[01:04:51] 2   --> $DIR/ambiguous-builtin-attrs.rs:9:3
[01:04:51] 
[01:04:51] 88    |    ^^^^^^^
[01:04:51] 88    |    ^^^^^^^
[01:04:51] 89    = note: consider adding an explicit import of `feature` to disambiguate
[01:04:51] 90 
[01:04:51] + error[E0425]: cannot find value `Bench` in this scope
[01:04:51] +   --> $DIR/ambiguous-builtin-attrs.rs:29:5
[01:04:51] +    |
[01:04:51] + LL |     Bench;
[01:04:51] + 
[01:04:51] 91 error[E0425]: cannot find value `NonExistent` in this scope
[01:04:51] 92   --> $DIR/ambiguous-builtin-attrs.rs:30:5
[01:04:51] 93    |
[01:04:51] 93    |
[01:04:51] 
[01:04:51] 94 LL |     NonExistent; //~ ERROR cannot find value `NonExistent` in this scope
[01:04:51] 96 
[01:04:51] - error: aborting due to 6 previous errors
[01:04:51] + error: aborting due to 8 previous errors
[01:04:51] 98 
[01:04:51] 98 
[01:04:51] - Some errors occurred: E0425, E0659.
[01:04:51] + Some errors occurred: E0425, E0428, E0659.
[01:04:51] 100 For more information about an error, try `rustc --explain E0425`.
[01:04:51] 101 
[01:04:51] 
[01:04:51] 
[01:04:51] The actual stderr differed from the expected stderr.
[01:04:51] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/ambiguous-builtin-attrs/ambiguous-builtin-attrs.stderr
[01:04:51] To update references, rerun the tests and pass the `--bless` flag
[01:04:51] To only update this specific test, also pass `--test-args proc-macro/ambiguous-builtin-attrs.rs`
[01:04:51] error: 1 errors occurred comparing output.
[01:04:51] status: exit code: 1
[01:04:51] status: exit code: 1
[01:04:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/proc-macro/ambiguous-builtin-attrs.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/ambiguous-builtin-attrs/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/ambiguous-builtin-attrs/auxiliary" "-A" "unused"
[01:04:51] ------------------------------------------
[01:04:51] 
[01:04:51] ------------------------------------------
[01:04:51] stderr:
[01:04:51] stderr:
[01:04:51] ------------------------------------------
[01:04:51] {"message":"the name `Test` is defined multiple times","code":{"code":"E0428","explanation":"\nA type or module has been defined more than once.\n\nErroneous code example:\n\n
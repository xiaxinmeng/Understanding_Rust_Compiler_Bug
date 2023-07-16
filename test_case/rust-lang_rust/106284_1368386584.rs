plain
error: /checkout/src/test/ui/augmented-assignments.rs:20: expected suggestion not found: mut y

error: 0 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/checkout/src/test/ui/augmented-assignments.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/augmented-assignments" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/augmented-assignments/auxiliary"
    Error {
        line_num: 20,
        kind: Some(
            Suggestion,
            Suggestion,
        ),
        msg: "mut y",
]

thread '[ui] src/test/ui/augmented-assignments.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1413:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

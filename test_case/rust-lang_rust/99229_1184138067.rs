plain
---- [ui] src/test/ui/asm/aarch64/srcloc.rs stdout ----

error: /checkout/src/test/ui/asm/aarch64/srcloc.rs:125: unexpected error: '125:14: 125:14: unrecognized instruction mnemonic'

error: /checkout/src/test/ui/asm/aarch64/srcloc.rs:125: expected error not found: invalid instruction mnemonic 'invalid_instruction'
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/aarch64/srcloc.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/srcloc" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-Ccodegen-units=1" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/srcloc/auxiliary"
    Error {
        line_num: 125,
        kind: Some(
            Error,
---
        line_num: 125,
        kind: Some(
            Error,
        ),
        msg: "invalid instruction mnemonic 'invalid_instruction'",
]

thread '[ui] src/test/ui/asm/aarch64/srcloc.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

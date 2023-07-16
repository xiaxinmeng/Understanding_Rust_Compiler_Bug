plain
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/null_pointer_deref.rs" "-L" "/tmp/compiletestSKklR3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestSKklR3/null_pointer_deref.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestSKklR3/null_pointer_deref.stage-id.aux"
    Error {
        line_num: 3,
        kind: Some(
            Error,
---
error: tests/compile-fail/null_pointer_write.rs:3: expected error not found: null pointer is not a valid pointer for this operation

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/null_pointer_write.rs" "-L" "/tmp/compiletestSKklR3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestSKklR3/null_pointer_write.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestSKklR3/null_pointer_write.stage-id.aux"
    Error {
        line_num: 3,
        kind: Some(
            Error,
---
.......... (60/61)
          (61/61)


/checkout/src/test/rustdoc-gui/search-tab-selection-if-current-is-empty.goml search-tab-selection-if-current-is-empty... FAILED
[ERROR] (line 6) TimeoutError: waiting for selector "#titles" failed: timeout 30000ms exceeded: for command `wait-for: "#titles"`
[ERROR] (line 7) "#titles > button:nth-of-type(1)" not found: for command `assert-attribute: ("#titles > button:nth-of-type(1)", {"class": "selected"})`
Build completed unsuccessfully in 0:00:48

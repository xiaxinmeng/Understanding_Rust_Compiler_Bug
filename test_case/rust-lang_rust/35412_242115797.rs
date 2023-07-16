
error: /build/src/test/compile-fail/E0220.rs:15: unexpected "error": '15:12: 15:24: the value of the associated type `Bar` (from the trait `Trait`) must be specified [E0191]'
error: /build/src/test/compile-fail/E0220.rs:15: unexpected "note": 'missing associated type `Bar` value'
error: /build/src/test/compile-fail/E0220.rs:16: expected error not found: E0191

error: 2 unexpected errors found, 1 expected errors not found

error: /build/src/test/compile-fail/unboxed-closure-sugar-wrong-trait.rs:15: unexpected "note": 'associated type `Output` not found'

error: 1 unexpected errors found, 0 expected errors not found

failures:
    [compile-fail] compile-fail/E0220.rs
    [compile-fail] compile-fail/unboxed-closure-sugar-wrong-trait.rs

test result: FAILED. 2443 passed; 2 failed; 12 ignored; 0 measured

/build/mk/tests.mk:769: recipe for target 'tmp/check-stage2-T-x86_64-unknown-linux-gnu-H-x86_64-unknown-linux-gnu-cfail.ok' failed

make: *** [tmp/check-stage2-T-x86_64-unknown-linux-gnu-H-x86_64-unknown-linux-gnu-cfail.ok] Error 101

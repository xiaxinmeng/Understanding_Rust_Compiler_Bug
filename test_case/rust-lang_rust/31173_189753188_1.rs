
expected.rs:3:5: 3:33 error: mismatched types:
 expected `()`,
    found `core::iter::Filter<core::ops::Range<_>, [closure@expected.rs:3:20: 3:32
 expected:_]>` (expected (),
    found struct `core::iter::Filter`) [E0308]
expected.rs:3     (0..10).filter(|i| expected)
                  ^~~~~~~~~~~~~~~~~~~~~~~~~~~~
expected.rs:3:5: 3:33 help: run `rustc --explain E0308` to see a detailed explanation
error: aborting due to previous error

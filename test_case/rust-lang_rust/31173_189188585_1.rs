
found.rs:3:5: 3:32 error: mismatched types:
 expected `()`,
    found `core::iter::Filter<core::ops::Range<_>, [closure@found.rs:3:20: 3:31 unfound:_]>`
(expected (),
    found struct `core::iter::Filter`) [E0308]
found.rs:3     (0..10).filter(|i| unfound)
               ^~~~~~~~~~~~~~~~~~~~~~~~~~~
found.rs:3:5: 3:32 help: run `rustc --explain E0308` to see a detailed explanation
error: aborting due to previous error

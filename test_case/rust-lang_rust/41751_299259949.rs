
[00:05:47] error: unused import: `early_warn`
[00:05:47]     --> /checkout/src/librustc/session/config.rs:1664:32
[00:05:47]      |
[00:05:47] 1664 |     use session::{early_error, early_warn};
[00:05:47]      |                                ^^^^^^^^^^
[00:05:47]      |
[00:05:47]      = note: #[deny(unused_imports)] implied by #[deny(warnings)]
[00:05:47] note: lint level defined here
[00:05:47]     --> /checkout/src/librustc/lib.rs:24:9
[00:05:47]      |
[00:05:47] 24   | #![deny(warnings)]
[00:05:47]      |         ^^^^^^^^
[00:05:47] 
[00:05:48] error: aborting due to previous error

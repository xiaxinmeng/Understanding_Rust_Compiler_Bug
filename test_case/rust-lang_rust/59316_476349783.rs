
error: unknown lint: `internal`
  --> src/librustc/lib.rs:32:31
   |
32 | #![cfg_attr(not(stage0), deny(internal))]
   |                               ^^^^^^^^
   |
   = note: `-D unknown-lints` implied by `-D warnings`

error: aborting due to previous error

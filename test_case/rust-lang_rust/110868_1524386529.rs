plain
[RUSTC-TIMING] object test:false 5.127
error[E0277]: the trait bound `process_inner::ExitStatus: core::default::Default` is not satisfied
    --> library/std/src/process.rs:1519:23
     |
1517 | #[derive(PartialEq, Eq, Clone, Copy, Debug, Default)]
1518 | #[stable(feature = "process", since = "1.0.0")]
1518 | #[stable(feature = "process", since = "1.0.0")]
1519 | pub struct ExitStatus(imp::ExitStatus);
     |                       ^^^^^^^^^^^^^^^ the trait `core::default::Default` is not implemented for `process_inner::ExitStatus`
    ::: /rustc/ee2a104407b73b9887cbe8de9c85ecdad393f094/library/core/src/default.rs:185:1
     |
185  | pub macro Default($item:item) {
     | ----------------- in this expansion of `#[derive(Default)]`
     | ----------------- in this expansion of `#[derive(Default)]`
     |
help: consider annotating `process_inner::ExitStatus` with `#[derive(Default)]`
    -->  library/std/src/sys/unix/process/process_fuchsia.rs:239:1
239  + #[derive(Default)]
239  + #[derive(Default)]
240  |         f.debug_struct("Child")

For more information about this error, try `rustc --explain E0277`.
[RUSTC-TIMING] std test:false 1.776
error: could not compile `std` due to previous error

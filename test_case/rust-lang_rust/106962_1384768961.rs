
error[E0432]
 --> src/lib.rs:1:24
  |
1 | use std::sync::{Mutex, AtomicU32};
  |                        ^^^^^^^^^ no `AtomicU32` in `sync`
  |
  = note: consider importing one of these items instead:
          pin_utils::core_reexport::sync::atomic::AtomicU32
          std::sync::atomic::AtomicU32
          core::sync::atomic::AtomicU32

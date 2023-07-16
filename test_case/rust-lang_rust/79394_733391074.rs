
error: unreachable pattern
   --> compiler/rustc_traits/src/chalk/lowering.rs:490:13
    |
490 |             chalk_ir::LifetimeData::Phantom(_, _) => unimplemented!(),
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `-D unreachable-patterns` implied by `-D warnings`

error: aborting due to previous error

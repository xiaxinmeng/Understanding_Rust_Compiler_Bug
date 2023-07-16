
error[E0283]: type annotations required: cannot resolve `dyn emitter::Emitter + rustc_data_structures::sync::Send: rustc_data_structures::sync::Send`
   --> src/librustc_errors/lib.rs:391:13
    |
391 |             e,
    |             ^
    |
    = note: required for the cast to the object type `dyn emitter::Emitter + rustc_data_structures::sync::Send`

error: aborting due to previous error

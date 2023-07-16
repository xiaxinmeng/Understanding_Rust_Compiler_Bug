shell
error[E0425]: cannot find value `FFI_TRAMPOLINE_SIZE` in this scope
   --> libffi-sys-rs/src/lib.rs:165:25
    |
165 |     pub tramp: [c_char; FFI_TRAMPOLINE_SIZE],
    |                         ^^^^^^^^^^^^^^^^^^^ not found in this scope
    |
note: these constants exist but are inaccessible
   --> libffi-sys-rs/src/arch.rs:34:9

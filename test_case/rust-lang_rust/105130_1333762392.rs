
error[E0121]: the placeholder `_` is not allowed within types on item signatures for static variables
  --> src/main.rs:34:24
   |
34 | static mut CTRLPARAMS: Array<_, CtrlParams> = Array::<CtrlParams>::with_max_entries(1, 0);
   |                        ^^^^^^^^^^^^^^^^^^^^
   |                        |
   |                        not allowed in type signatures
   |                        help: replace with the correct type: `Array<CtrlParams>`

For more information about this error, try `rustc --explain E0121`.
error: could not compile `ebpftm-ebpf` due to previous error

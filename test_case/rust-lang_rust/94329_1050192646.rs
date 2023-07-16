`
 error[E0381]: use of possibly-uninitialized variable: `adt`
   --> src/tools/clippy/clippy_utils/src/ty.rs:580:55
    |
580 |         && let &[krate, .., name] = &*cx.get_def_path(adt.did)
    |                                                       ^^^^^^^ use of possibly-uninitialized `adt.did`

error[E0381]: use of possibly-uninitialized variable: `krate`
   --> src/tools/clippy/clippy_utils/src/ty.rs:581:51
    |
581 |         && let sym::libc | sym::core | sym::std = krate
    |                                                   ^^^^^ use of possibly-uninitialized `krate`

error[E0381]: borrow of possibly-uninitialized variable: `name`
   --> src/tools/clippy/clippy_utils/src/ty.rs:582:12
    |
582 |         && name.as_str() == "c_void"
    |            ^^^^^^^^^^^^^ use of possibly-uninitialized `name`

    Checking regex v1.5.4
For more information about this error, try `rustc --explain E0381`.
error: could not compile `clippy_utils` due to 4 previous errors

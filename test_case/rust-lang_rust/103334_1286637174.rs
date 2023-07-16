plain
    |
365 | #[unstable(feature = "dbg_macro")]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: duplicate diagnostic item found: `dbg_macro`.
    |
366 | macro_rules! pdbg {
    | ^^^^^^^^^^^^^^^^^
    |
---
383 | |     };
384 | | }
    | |_^

error[E0711]: feature `dbg_macro` is declared unstable, but was previously declared stable
    |
365 | #[unstable(feature = "dbg_macro")]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


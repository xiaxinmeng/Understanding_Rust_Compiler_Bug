plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0308]: mismatched types
   --> compiler/rustc_middle/src/middle/cstore.rs:214:38
    |
214 |         .filter(|cnum| !tcx.dep_kind(cnum).macros_only())
    |                                      |
    |                                      |
    |                                      expected struct `rustc_span::def_id::CrateNum`, found `&rustc_span::def_id::CrateNum`
    |                                      help: consider dereferencing the borrow: `*cnum`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_middle`

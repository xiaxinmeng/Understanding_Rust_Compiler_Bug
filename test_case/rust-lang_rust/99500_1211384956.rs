plain
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0308]: `if` and `else` have incompatible types
    --> compiler/rustc_codegen_ssa/src/back/link.rs:1598:9
     |
1595 |       } else if !(sess.target.os == "fuchsia" && flavor == LinkerFlavor::Gcc) {
     |  ____________-
1596 | |         &opts.pre_link_objects
1597 | |     } else {
1598 | |         &[]
1598 | |         &[]
     | |         ^^^ expected struct `BTreeMap`, found array of 0 elements
1599 | |     };
     | |_____- `if` and `else` have incompatible types
     |
     = note: expected reference `&BTreeMap<LinkOutputKind, Vec<Cow<'_, str>>>`
                found reference `&[_; 0]`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_codegen_ssa` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_codegen_ssa` due to previous error

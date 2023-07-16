plain
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
error: unused variable: `path`
   --> compiler/rustc_privacy/src/lib.rs:683:36
    |
683 |             hir::ItemKind::Use(ref path, ..) => {
    |                                    ^^^^ help: if this is intentional, prefix it with an underscore: `_path`
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustc_privacy`


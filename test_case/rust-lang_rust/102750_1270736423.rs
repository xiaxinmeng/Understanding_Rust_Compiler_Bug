plain
error[E0308]: mismatched types
     |
    ::: compiler/rustc_hir/src/hir.rs:3543:5
     |
3543 |     static_assert_size!(TraitItem<'_>, 88);

error[E0308]: mismatched types
     |
    ::: compiler/rustc_hir/src/hir.rs:3544:5
    ::: compiler/rustc_hir/src/hir.rs:3544:5
     |
3544 |     static_assert_size!(TraitItemKind<'_>, 48);

For more information about this error, try `rustc --explain E0308`.
[RUSTC-TIMING] rustc_hir test:false 1.323
error: could not compile `rustc_hir` due to 2 previous errors

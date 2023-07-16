
rustc: x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/libsyntax
error[E0507]: cannot move out of borrowed content
   --> src/libsyntax/fold.rs:610:27
    |
610 |             token::NtItem(fld.fold_item(item)
    |                           ^^^^^^^^^^^^^^^^^^^ cannot move out of borrowed content

error[E0507]: cannot move out of borrowed content
   --> src/libsyntax/fold.rs:617:43
    |
617 |             token::NtStmt(stmt.map(|stmt| fld.fold_stmt(stmt)
    |                                           ^^^^^^^^^^^^^^^^^^^ cannot move out of borrowed content

error[E0507]: cannot move out of borrowed content
   --> src/libsyntax/fold.rs:632:45
    |
632 |             token::NtImplItem(arm.map(|arm| fld.fold_impl_item(arm)
    |                                             ^^^^^^^^^^^^^^^^^^^^^^^ cannot move out of borrowed content

error[E0507]: cannot move out of borrowed content
   --> src/libsyntax/fold.rs:635:46
    |
635 |             token::NtTraitItem(arm.map(|arm| fld.fold_trait_item(arm)
    |                                              ^^^^^^^^^^^^^^^^^^^^^^^^ cannot move out of borrowed content

error[E0507]: cannot move out of borrowed content
   --> src/libsyntax/test.rs:183:22
    |
183 |         let folded = fold::noop_fold_item(i, self).expect_one("noop did something");
    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot move out of borrowed content

error: aborting due to 5 previous errors


 error[E0308]: mismatched types
   --> C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src/test\run-pass-fulldeps\auxiliary\procedural_mbe_matching.rs:39:22
    |
 39 |             match (&*map[&str_to_ident("matched").name], &*map[&str_to_ident("pat").name]) {
    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `syntax::ast::Ident`, found struct `syntax::ast::Name`
    |
    = note: expected type `&syntax::ast::Ident`
    = note:    found type `&syntax::ast::Name`

 error[E0308]: mismatched types
   --> C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src/test\run-pass-fulldeps\auxiliary\procedural_mbe_matching.rs:39:60
    |
 39 |             match (&*map[&str_to_ident("matched").name], &*map[&str_to_ident("pat").name]) {
    |                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `syntax::ast::Ident`, found struct `syntax::ast::Name`
    |
    = note: expected type `&syntax::ast::Ident`
    = note:    found type `&syntax::ast::Name`

 error: aborting due to 2 previous errors

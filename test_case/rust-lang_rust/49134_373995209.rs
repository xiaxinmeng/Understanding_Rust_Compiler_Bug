
error[E0308]: mismatched types
  --> clippy_lints/src/unsafe_removed_from_name.rs:46:29
   |
46 |         UseTreeKind::Simple(Some(new_name)) => {
   |                             ^^^^^^^^^^^^^^ expected struct `syntax::ast::Ident`, found enum `std::option::Option`
   |
   = note: expected type `syntax::ast::Ident`
              found type `std::option::Option<_>`
error[E0308]: mismatched types
  --> clippy_lints/src/unsafe_removed_from_name.rs:55:29
   |
55 |         UseTreeKind::Simple(None) |
   |                             ^^^^ expected struct `syntax::ast::Ident`, found enum `std::option::Option`
   |
   = note: expected type `syntax::ast::Ident`
              found type `std::option::Option<_>`
error: aborting due to 2 previous errors

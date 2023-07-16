
error: proc macro panicked
  --> src/lib.rs:9:5
   |
9  | /     quote! {
10 | |         $()*
11 | |     }
   | |_____^
   |
   = help: message: `$` must be followed by an ident or `$` in `quote!`

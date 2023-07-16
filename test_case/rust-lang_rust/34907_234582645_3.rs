
error: mismatched method receiver [--explain E0211]
  --> issue-26194.rs:14:5
   |>
14 |>     fn f(self: *mut S) -> String { self.0 }
   |>     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `S`, found *-ptr
note: expected type `S`
note:    found type `*mut S`

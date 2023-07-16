
error[E0308]: mismatched types
  --> src/test/ui/deref-suggestion.rs:12:20
   |
12 |     ($x:expr) => { &$x }
   |                    ^^^ expected u32, found &{integer}
...
32 |     foo3(borrow!(0));
   |          ---------- in this macro invocation
   |
   = note: expected type `u32`
              found type `&{integer}`
   = help: try with `0`

error: aborting due to 5 previous errors

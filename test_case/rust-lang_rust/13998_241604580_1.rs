
error: mismatched types [--explain E0308]
  --> src/main.rs:16:14
16 |>     register(run);
   |>              ^^^ expected concrete lifetime, found bound lifetime parameter
note: expected type `fn(&mut RunLoop<'_>)`
note:    found type `fn(&'a mut RunLoop<'a>) {run}`
note: expected concrete lifetime is lifetime ReSkolemized(0, BrAnon(0))

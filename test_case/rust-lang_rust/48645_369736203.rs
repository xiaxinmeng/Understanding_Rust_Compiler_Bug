
error[E0597]: borrowed value does not live long enough
  --> src/test/ui/nll/borrowed-universal-error.rs:22:12
   |
22 |     gimme(&(v,))
   |      ----- ^^^^ temporary is created here, and then borrowed
   |      |
   |      reference is returned here
24 | }
   | - temporary will be freed here, when this block exits

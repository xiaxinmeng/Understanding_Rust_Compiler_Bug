
error[E0308]: mismatched types
  --> src/main.rs:16:38
   |
16 |     let a: Wrapper<u32> = weird_call(25);
   |                                      ^^ expected `bool`, found integer

error[E0308]: mismatched types
  --> src/main.rs:16:27
   |
16 |     let a: Wrapper<u32> = weird_call(25);
   |            ------------   ^^^^^^^^^^^^^^ expected `u32`, found `bool`
   |            |
   |            expected due to this
   |
   = note: expected struct `Wrapper<u32>`
              found struct `Wrapper<bool>`

error: aborting due to 2 previous errors

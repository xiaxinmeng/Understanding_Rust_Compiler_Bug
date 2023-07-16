
error[E0053]: method `a` has an incompatible type for trait
 --> src/c.rs:7:5
  |
7 |     fn a(a: a::A) { }
  |     ^^^^^^^^^^^^^^^^^ expected struct `a::A`, found a different struct `a::A`
  |
  = note: expected type `fn(a::A)` (struct `a::A`)
             found type `fn(a::A)` (struct `a::A`)
note: Perhaps two different versions of crate `a` are being used?
 --> src/c.rs:7:5
  |
7 |     fn a(a: a::A) { }
  |     ^^^^^^^^^^^^^^^^^

error[E0053]: method `sub` has an incompatible type for trait
 --> src/c.rs:8:5
  |
8 |     fn sub(sub: a::Sub) { }
  |     ^^^^^^^^^^^^^^^^^^^^^^^ expected struct `a::sub::Sub`, found struct `a::Sub`
  |
  = note: expected type `fn(a::sub::Sub)`
             found type `fn(a::Sub)`

error: aborting due to 2 previous errors

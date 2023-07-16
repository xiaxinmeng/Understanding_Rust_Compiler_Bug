
rustc 1.16.0-nightly (83c2d9523 2017-01-24)
error[E0080]: constant evaluation error
  --> <anon>:12:20
   |
12 |     Small { _cake: BlackForest }
   |                    ^^^^^^^^^^^ unimplemented constant expression: enum variants
   |
note: for pattern here
  --> <anon>:19:12
   |
19 |     if let QUX = cake {
   |            ^^^

error: aborting due to previous error


error: expected identifier, found `"howdy"`
  --> examples/test.rs:11:9
   |
10 |     Thing {
   |     ----- while parsing this struct
11 |         "howdy".to_string(),
   |         ^^^^^^^ expected identifier

error: expected identifier, found `"yo"`
  --> examples/test.rs:18:9
   |
17 |     Thing {
   |     ----- while parsing this struct
18 |         "yo".to_string(),
   |         ^^^^ expected identifier

error[E0063]: missing fields `x`, `y` in initializer of `Thing`
  --> examples/test.rs:10:5
   |
10 |     Thing {
   |     ^^^^^ missing `x`, `y`

error[E0063]: missing fields `x`, `y` in initializer of `Thing`
  --> examples/test.rs:17:5
   |
17 |     Thing {
   |     ^^^^^ missing `x`, `y`

error: aborting due to 4 previous errors

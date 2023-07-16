
error[E0381]: use of possibly uninitialized variable: `a.x`
  --> borrowck-uninit-field-access.rs:34:13
   |
34 |     let _ = a.x + 1; //[ast]~ ERROR use of possibly uninitialized variable: `a.x`
   |             ^^^ use of possibly uninitialized `a.x`

error[E0382]: use of moved value: `line1.origin.x`
  --> borrowck-uninit-field-access.rs:39:13
   |
38 |     let _moved = line1.origin;
   |                  ------------ value moved here
39 |     let _ = line1.origin.x + 1; //[ast]~ ERROR use of moved value: `line1.origin.x`
   |             ^^^^^^^^^^^^^^ value used here after move
   |
   = note: move occurs because `line1.origin` has type `Point`, which does not implement the `Copy` trait

error[E0382]: use of moved value: `line2`
  --> borrowck-uninit-field-access.rs:44:5
   |
43 |     let _moved = (line2.origin, line2.middle);
   |                                 ------------ value moved here
44 |     line2.consume(); //[ast]~ ERROR use of partially moved value: `line2` [E0382]
   |     ^^^^^ value used here after move
   |
   = note: move occurs because `line2.middle` has type `Point`, which does not implement the `Copy` trait

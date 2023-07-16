
error[E0381]: use of possibly uninitialized variable: `i`
  --> src/test/compile-fail/borrowck/borrowck-and-init.rs:17:20
   |
17 |     println!("{}", i); //~ ERROR use of possibly uninitialized variable: `i`
   |                    ^ use of possibly uninitialized `i`

error[E0381]: use of possibly uninitialized variable: `_` (Mir)
  --> src/test/compile-fail/borrowck/borrowck-and-init.rs:16:44
   |
16 |     println!("{}", false && { i = 5; true });
   |                                            ^

error[E0381]: use of possibly uninitialized variable: `i` (Mir)
  --> src/test/compile-fail/borrowck/borrowck-and-init.rs:17:20
   |
17 |     println!("{}", i); //~ ERROR use of possibly uninitialized variable: `i`
   |                    ^

error[E0381]: use of possibly uninitialized variable: `i` (Mir)
  --> src/test/compile-fail/borrowck/borrowck-and-init.rs:18:2
   |
18 | }
   |  ^

error: aborting due to 4 previous errors

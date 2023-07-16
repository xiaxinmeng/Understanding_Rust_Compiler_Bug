
/tmp $ rustc hi.rs
hi.rs:5:6: 5:7 error: use of undeclared type name `X`
hi.rs:5 impl X {}
             ^
hi.rs:6:6: 6:7 error: use of undeclared type name `Y`
hi.rs:6 impl Y {}
             ^
hi.rs:7:6: 7:9 error: use of undeclared type name `foo`
hi.rs:7 impl foo {}
             ^~~
error: aborting due to 3 previous errors

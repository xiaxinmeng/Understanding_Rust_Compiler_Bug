
$ rustc test.rs
test.rs:6:5: 6:18 error: user-defined types or type parameters cannot shadow the
 primitive types [E0317]
test.rs:6     type u8 = ();
              ^~~~~~~~~~~~~
error: aborting due to previous error


error[E0505]: cannot move out of `nc` because it is borrowed
  --> src/main.rs:13:10
   |
12 |     let _foo = foo(&nc);
   |                    --- borrow of `nc` occurs here
13 |     drop(nc); // do something with nc
   |          ^^ move out of `nc` occurs here
14 | }
   | - borrow might be used here, when `_foo` is dropped and runs the destructor for type `impl Marker`

For more information about this error, try `rustc --explain E0505`.
error: could not compile `lf2` due to previous error

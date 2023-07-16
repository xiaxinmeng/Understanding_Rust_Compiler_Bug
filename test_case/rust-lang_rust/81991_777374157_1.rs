
error[E0308]: `match` arms have incompatible types
  --> test.rs:10:18
   |
3  |   async fn async_dummy() {}
   |                          - the `Output` of this `async fn`'s found opaque type
...
6  |       let _ = match true {
   |  _____________-
7  | |         true => {
8  | |             async_dummy();
   | |             -------------- this is found to be of type `()`
9  | |         }
10 | |         false => async_dummy(),
   | |                  ^^^^^^^^^^^^^ expected `()`, found opaque type
11 | |     };
   | |_____- `match` arms have incompatible types
   |
   = note:     expected type `()`
           found opaque type `impl Future`
help: consider `await`ing on the `Future`
   |
10 |         false => async_dummy().await,
   |                               ^^^^^^
help: consider removing this semicolon and boxing the expressions
   |
8  |             Box::new(async_dummy())
9  |         }
10 |         false => Box::new(async_dummy()),
   |

error: aborting due to previous error

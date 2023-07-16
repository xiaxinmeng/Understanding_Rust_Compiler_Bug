
error[E0271]: type mismatch resolving `<std::result::Result<std::result::Result<(), std::result::Result<std::result::Result<(), std::result::Result<std::result::Result<(), std::option::Option<{integer}>>, ()>>, ()>>, ()> as Future>::Error == Foo`
  --> src/main.rs:16:5
   |
16 | /     Box::new(
17 | |         Ok::<_, ()>(
18 | |             Err::<(), _>(
19 | |                 Ok::<_, ()>(
...  |
28 | |         )
29 | |     )
   | |_____^ expected (), found struct `Foo`
   |
   = note: expected type `()`
              found type `Foo`
   = note: required for the cast to the object type `dyn Future<Error = Foo>`

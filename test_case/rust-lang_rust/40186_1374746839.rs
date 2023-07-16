
error[[E0271]](https://doc.rust-lang.org/nightly/error-index.html#E0271): type mismatch resolving `<Result<Result<(), Result<Result<(), Result<Result<(), Option<{integer}>>, ()>>, ()>>, ()> as Future>::Error == Foo`
  --> src/main.rs:16:5
   |
16 | /     Box::new(
17 | |         Ok::<_, ()>(
18 | |             Err::<(), _>(
19 | |                 Ok::<_, ()>(
...  |
28 | |         )
29 | |     )
   | |_____^ type mismatch resolving `<Result<Result<(), Result<Result<(), Result<Result<(), Option<{integer}>>, ()>>, ()>>, ()> as Future>::Error == Foo`
   |
note: expected this to be `Foo`
  --> src/main.rs:6:18
   |
6  |     type Error = E;
   |                  ^
   = note: required for the cast from `Result<Result<(), Result<Result<(), Result<Result<(), Option<{integer}>>, ()>>, ()>>, ()>` to the object type `dyn Future<Error = Foo>`


error: implementation of `Iterator` is not general enough
  --> src/main.rs:8:5
   |
8  | /     async move {
9  | |         let flat = some_vec.iter()
10 | |             .map(|_| std::iter::empty::<()>())
11 | |             .flatten();
12 | |         std::mem::drop(flat);
13 | |         dummy().await;
14 | |     }
   | |_____^ implementation of `Iterator` is not general enough
   |
   = note: `Iterator` would have to be implemented for the type `std::slice::Iter<'0, ()>`, for any lifetime `'0`...
   = note: ...but `Iterator` is actually implemented for the type `std::slice::Iter<'1, ()>`, for some specific lifetime `'1`

error: implementation of `FnOnce` is not general enough
  --> src/main.rs:8:5
   |
8  | /     async move {
9  | |         let flat = some_vec.iter()
10 | |             .map(|_| std::iter::empty::<()>())
11 | |             .flatten();
12 | |         std::mem::drop(flat);
13 | |         dummy().await;
14 | |     }
   | |_____^ implementation of `FnOnce` is not general enough
   |
   = note: closure with signature `fn(&'0 ()) -> std::iter::Empty<()>` must implement `FnOnce<(&(),)>`, for any lifetime `'0`...
   = note: ...but it actually implements `FnOnce<(&(),)>`


error: implementation of `Stream` is not general enough
  --> src/main.rs:90:22
   |
90 |     let map = source.map(|x: &_| x); // Remove to break.
   |                      ^^^
   |
   = note: `Stream` would have to be implemented for the type `&'0 mut Map<Repeat, [closure@src/main.rs:90:26: 90:35]>`, for any lifetime `'0`
   = note: but `Stream` is actually implemented for the type `&'1 mut Map<Repeat, [closure@src/main.rs:90:26: 90:35]>`, for some specific lifetime `'1`

error: aborting due to previous error

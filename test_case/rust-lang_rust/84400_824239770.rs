
error: implementation of `FnOnce` is not general enough
 --> src/main.rs:7:5
  |
7 |     serialize_wrap(serialize, &Test);
  |     ^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough
  |
  = note: `fn(&'2 Test) {serialize::<&'2 Test>}` must implement `FnOnce<(&'1 Test,)>`, for any lifetime `'1`...
  = note: ...but it actually implements `FnOnce<(&'2 Test,)>`, for some specific lifetime `'2`

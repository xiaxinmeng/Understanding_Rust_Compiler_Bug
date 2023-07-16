
error: implementation of `FnOnce` is not general enough
  --> src/main.rs:11:13
   |
11 |     let _ = is_send(make_future_2());
   |             ^^^^^^^ implementation of `FnOnce` is not general enough
   |
   = note: closure with signature `fn(Pin<Box<(dyn futures::Stream<Item = ()> + std::marker::Send + '0)>>) -> futures::stream::Empty<()>` must implement `FnOnce<(Pin<Box<(dyn futures::Stream<Item = ()> + std::marker::Send + '1)>>,)>`, for any two lifetimes `'0` and `'1`...
   = note: ...but it actually implements `FnOnce<(Pin<Box<(dyn futures::Stream<Item = ()> + std::marker::Send + RePlaceholder(Placeholder { universe: U6, name: BrAnon(7) }))>>,)>`

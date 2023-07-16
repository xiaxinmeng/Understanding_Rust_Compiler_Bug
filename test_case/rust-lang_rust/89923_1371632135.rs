
error[E0308]: mismatched types
  --> f.rs:16:5
   |
16 |     async_magic(push_zero).await;
   |     ^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected trait `for<'a> <for<'a> fn(&'a mut Vec<u8>) -> impl Future<Output = ()> {push_zero} as FnOnce<(&'a mut Vec<u8>,)>>`
              found trait `for<'a> <for<'a> fn(&'a mut Vec<u8>) -> impl Future<Output = ()> {push_zero} as FnOnce<(&'a mut Vec<u8>,)>>`
note: the lifetime requirement is introduced here
  --> f.rs:5:32
   |
5  |     F: FnOnce(&mut Vec<u8>) -> R,
   |                                ^

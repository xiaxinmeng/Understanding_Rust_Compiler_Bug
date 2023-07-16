rust
error[E0491]: in type `&'b &'a ()`, reference has a longer lifetime than the data it references
   --> src/main.rs:51:5
    |
 51 |     type Output = PhantomData<&'b &'a ()>;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
 note: the pointer is valid for the lifetime `'b` as defined on the impl at 50:6
   --> src/main.rs:50:6
    |
 50 | impl<'b, 'a> MyFn<&'b (), &'a ()> for Fun {
    |      ^^
 note: but the referenced data is only valid for the lifetime `'a` as defined on the impl at 50:10
   --> src/main.rs:50:10
    |
 50 | impl<'b, 'a> MyFn<&'b (), &'a ()> for Fun {
    |          ^^

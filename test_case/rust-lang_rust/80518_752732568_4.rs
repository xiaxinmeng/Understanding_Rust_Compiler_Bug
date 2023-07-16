
error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
  --> src\lib.rs:38:30
   |
38 |         data.into_iter().map(move |b| *b.borrow() ^ next_key(self.key, &mut self.index))
   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: first, the lifetime cannot outlive the lifetime `'b` as defined on the method body at 32:18...
  --> src\lib.rs:32:18
   |
32 |     pub fn munge<'b, Data>(&'b mut self, data: Data) -> impl Iterator<Item=u8> + 'a + 'b
   |                  ^^
note: ...so that the types are compatible
  --> src\lib.rs:38:30
   |
38 |         data.into_iter().map(move |b| *b.borrow() ^ next_key(self.key, &mut self.index))
   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: expected `&mut Xorcism<'a>`
              found `&'b mut Xorcism<'a>`
note: but, the lifetime must be valid for the lifetime `'a` as defined on the impl at 18:6...
  --> src\lib.rs:18:6
   |
18 | impl<'a> Xorcism<'a> {
   |      ^^
note: ...so that return value is valid for the call
  --> src\lib.rs:32:57
   |
32 |     pub fn munge<'b, Data>(&'b mut self, data: Data) -> impl Iterator<Item=u8> + 'a + 'b
   |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

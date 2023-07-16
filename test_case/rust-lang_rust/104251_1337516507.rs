
error[E0599]: the associated item `BAR` exists for struct `Foo<[u8]>`, but its trait bounds were not satisfied
 --> src/main.rs:8:30
  |
1 | struct Foo<T: ?Sized>(T);
  | --------------------- associated item `BAR` not found for this struct
...
8 |     let _: () = Foo::<[u8]>::BAR;
  |                              ^^^ associated item cannot be called on `Foo<[u8]>` due to unsatisfied trait bounds
  |
  = note: the following trait bounds were not satisfied:
          `[u8]: Sized`


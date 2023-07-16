
error[E0308]: mismatched types
  --> src/main.rs:19:5
   |
19 | /     async {
20 | |         let _x = Struct::<Empty<&'static ()>, _>(PhantomData);
21 | |         async {}.await;
22 | |     }
   | |_____^ one type is more general than the other
   |
   = note: expected reference `&()`
              found reference `&()`

For more information about this error, try `rustc --explain E0308`.

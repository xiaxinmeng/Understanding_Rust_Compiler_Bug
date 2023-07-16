
error: lifetime may not live long enough
 --> src/main.rs:8:23
  |
5 |     a: &'l mut Foo,
  |     - has type `&mut Foo<'1>`
6 |     b: &'l mut Foo,
  |     - has type `&mut Foo<'2>`
7 | ) {
8 |     let _ = if flag { a } else { b };
  |                       ^ requires that `'1` must outlive `'2`

error: lifetime may not live long enough
 --> src/main.rs:8:34
  |
5 |     a: &'l mut Foo,
  |     - has type `&mut Foo<'2>`
6 |     b: &'l mut Foo,
  |     - has type `&mut Foo<'1>`
7 | ) {
8 |     let _ = if flag { a } else { b };
  |                                  ^ requires that `'1` must outlive `'2`

error: aborting due to 2 previous errors

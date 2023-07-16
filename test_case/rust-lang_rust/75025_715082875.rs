
error: lifetime must precede `mut`
 --> src/lib.rs:2:13
  |
2 |     parent: &mut 'a SpanManager,
  |             ^^^^^^^ help: place the lifetime before `mut`: `&'a mut`

error[E0412]: cannot find type `SpanManager` in this scope
 --> src/lib.rs:2:21
  |
2 |     parent: &mut 'a SpanManager,
  |                     ^^^^^^^^^^^ not found in this scope

error: aborting due to 2 previous errors


error[E0308]: mismatched types
 --> src/main.rs:9:12
  |
2 | type A = impl Drop;
  |          --------- the expected opaque type
...
9 |     S { a: Box::new(1) }
  |            ^^^^^^^^^^^ expected opaque type, found struct `Box`
  |
  = note: expected opaque type `impl Drop`
                  found struct `Box<{integer}>`

error: could not find defining uses
 --> src/main.rs:2:10
  |
2 | type A = impl Drop;
  |          ^^^^^^^^^

error: aborting due to 2 previous errors

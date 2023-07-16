
error[E0107]: wrong number of const arguments: expected 0, found 1
 --> src/lib.rs:5:15
  |
5 |         Baz::<FOO>;
  |               ^^^ unexpected const argument

error[E0107]: wrong number of type arguments: expected 1, found 0
 --> src/lib.rs:5:9
  |
5 |         Baz::<FOO>;
  |         ^^^^^^^^^^ expected 1 type argument

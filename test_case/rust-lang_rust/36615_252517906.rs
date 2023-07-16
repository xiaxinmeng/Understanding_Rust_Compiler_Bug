
error[E0107]: wrong number of lifetime parameters: expected 2, found 0
 --> /tmp/test.rs:6:18
  |
6 | type Bar<T, U> = Foo<T, U>;
  |                  ^^^^^^^^^ expected 2 lifetime parameters

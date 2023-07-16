
error[E0271]: type mismatch resolving `<() as weird_ice::HasAssocType>::Inner == <() as weird_ice::HasAssocType>::Inner`
 --> tests/test.rs:4:7
  |
4 |     g(f())(());
  |       ^^^ expected (), found associated type
  |
  = note: expected type `()`
             found type `<() as weird_ice::HasAssocType>::Inner`

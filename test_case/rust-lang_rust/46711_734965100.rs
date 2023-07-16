
error[E0277]: the trait bound `(): futures::Future` is not satisfied
  --> src/main.rs:17:14
   |
17 | fn test() -> MapErr<(), ()> {
   |              ^^^^^^^^^^^^^^ the trait `futures::Future` is not implemented for `()`
   | 
  ::: /home/achin/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-0.1.17/src/future/map_err.rs:8:34
   |
8  | pub struct MapErr<A, F> where A: Future {
   |                                  ------ required by this bound in `futures::MapErr`

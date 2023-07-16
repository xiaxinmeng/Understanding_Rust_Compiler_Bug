
error[E0277]: the trait bound `P: F<()>` is not satisfied
  --> src/main.rs:17:1
   |
17 | / impl<P> M for A<P> where
18 | |     N<P>: M,
19 | |     //P: F<()>,
20 | |     P: F<N<S>>, // This should work just as fine as the commented bound just above
   | |                - help: consider further restricting type parameter `P`: `, P: F<()>`
21 | | {}
   | |__^ the trait `F<()>` is not implemented for `P`

error[E0277]: the trait bound `P: F<()>` is not satisfied
  --> src/main.rs:17:9
   |
17 | impl<P> M for A<P> where
   |         ^ the trait `F<()>` is not implemented for `P`
...
20 |     P: F<N<S>>, // This should work just as fine as the commented bound just above
   |                - help: consider further restricting type parameter `P`: `, P: F<()>`
   |
   = note: required because of the requirements on the impl of `M` for `A<P>`

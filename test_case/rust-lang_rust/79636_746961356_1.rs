
error: generic associated types in trait paths are currently not implemented
  --> src/main.rs:15:41
   |
15 |     MInner: Monad<Unwrapped = A, Wrapped<A> = MOuter::Wrapped<A>>,
   |                                         ^^^

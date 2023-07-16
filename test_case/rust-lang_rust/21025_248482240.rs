
1:
expected: `A<B<C, D>, E>`, found: `B<C, D>`  // You likely forgot to use `Ok` or `Some`,
                                             // discussed in previous comments
2:
expected: `A<B<C, D>, E>`, found: `B<X, Y>`  // You likely forgot to use `Ok` and above
                                             // that you're also providing a wrong type
                                             // regardless
3:
expected: `A<B<C, D>, E>`, found: `A<F, E>`  // You provided the wrong type, already
                                             // handled somewhat by the current
                                             // implementation
4:
expected: `A<B<C, D>, E>`, found: `A<F, J>`  // You provided the wrong type for both
                                             // the success and failure types
5:
expected: `B<C, D>`, found: `A<B<C, D>, E>`  // You probably used `Ok` or `Some` when
                                             // you didn't need to

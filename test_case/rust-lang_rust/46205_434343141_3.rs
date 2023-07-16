
rror[E0119]: conflicting implementations of trait `std::convert::Into<_>` for type `CustomWrapper<_>`:

 --> <source>:3:1

  |

3 | impl<T> Into<T> for CustomWrapper<T> {

  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

  |

  = note: conflicting implementation in crate `core`:

          - impl<T, U> std::convert::Into<U> for T

            where U: std::convert::From<T>;

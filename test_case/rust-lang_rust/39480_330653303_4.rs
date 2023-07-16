
error[E0277]: the trait bound `I: iter::traits::ExactSizeIterator` is not satisfied
    --> src/libcore/iter/iterator.rs:2301:18
     |
2301 |         (**self).rposition(predicate)
     |                  ^^^^^^^^^ the trait `iter::traits::ExactSizeIterator` is not implemented for `I`
     |
     = help: consider adding a `where I: iter::traits::ExactSizeIterator` bound

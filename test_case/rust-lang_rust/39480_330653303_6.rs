rust
error[E0277]: the trait bound `P: ops::function::FnMut<(<I as iter::iterator::Iterator>::Item,)>` is not satisfied
    --> src/libcore/iter/iterator.rs:2315:18
     |
2315 |         (**self).rposition(predicate)
     |                  ^^^^^^^^^ the trait `ops::function::FnMut<(<I as iter::iterator::Iterator>::Item,)>` is not implemented for `P`
     |
     = help: consider adding a `where P: ops::function::FnMut<(<I as iter::iterator::Iterator>::Item,)>` bound


error[E0277]: expected a `ops::function::FnMut<(&<Self as iter::traits::iterator::Iterator>::Item,)>` closure, found `impl ops::function::FnMut<(&<Self as iter::traits::iterator::Iterator>::Item,)>`
    --> src/libcore/iter/traits/iterator.rs:1540:35
     |
1540 |         right.extend(fused.filter(invert(predicate)));
     |                                   ^^^^^^^^^^^^^^^^^ expected an `FnMut<(&<Self as iter::traits::iterator::Iterator>::Item,)>` closure, found `impl ops::function::FnMut<(&<Self as iter::traits::iterator::Iterator>::Item,)>`
     |
     = help: the trait `for<'r> ops::function::FnMut<(&'r <Self as iter::traits::iterator::Iterator>::Item,)>` is not implemented for `impl ops::function::FnMut<(&<Self as iter::traits::iterator::Iterator>::Item,)>`

error[E0271]: type mismatch resolving `for<'r> <impl ops::function::FnMut<(&<Self as iter::traits::iterator::Iterator>::Item,)> as ops::function::FnOnce<(&'r <Self as iter::traits::iterator::Iterator>::Item,)>>::Output == bool`
    --> src/libcore/iter/traits/iterator.rs:1540:28
     |
1540 |         right.extend(fused.filter(invert(predicate)));
     |                            ^^^^^^ expected bound lifetime parameter, found concrete lifetime

error[E0271]: type mismatch resolving `for<'r> <impl ops::function::FnMut<(&<Self as iter::traits::iterator::Iterator>::Item,)> as ops::function::FnOnce<(&'r <Self as iter::traits::iterator::Iterator>::Item,)>>::Output == bool`
    --> src/libcore/iter/traits/iterator.rs:1540:15
     |
1540 |         right.extend(fused.filter(invert(predicate)));
     |               ^^^^^^ expected bound lifetime parameter, found concrete lifetime
     |
     = note: required because of the requirements on the impl of `iter::traits::iterator::Iterator` for `iter::adapters::Filter<iter::adapters::Fuse<Self>, impl ops::function::FnMut<(&<Self as iter::traits::iterator::Iterator>::Item,)>>`

error[E0277]: expected a `ops::function::FnMut<(&<Self as iter::traits::iterator::Iterator>::Item,)>` closure, found `impl ops::function::FnMut<(&<Self as iter::traits::iterator::Iterator>::Item,)>`
    --> src/libcore/iter/traits/iterator.rs:1540:22
     |
1540 |         right.extend(fused.filter(invert(predicate)));
     |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an `FnMut<(&<Self as iter::traits::iterator::Iterator>::Item,)>` closure, found `impl ops::function::FnMut<(&<Self as iter::traits::iterator::Iterator>::Item,)>`
     |
     = help: the trait `for<'r> ops::function::FnMut<(&'r <Self as iter::traits::iterator::Iterator>::Item,)>` is not implemented for `impl ops::function::FnMut<(&<Self as iter::traits::iterator::Iterator>::Item,)>`
     = note: required because of the requirements on the impl of `iter::traits::iterator::Iterator` for `iter::adapters::Filter<iter::adapters::Fuse<Self>, impl ops::function::FnMut<(&<Self as iter::traits::iterator::Iterator>::Item,)>>`

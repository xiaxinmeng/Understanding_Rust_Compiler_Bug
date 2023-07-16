
<anon>:5:5: 7:6 error: the trait bound `[_]: std::marker::Sized` is not satisfied [E0277]
<anon>:5     for x in vec[3..] {
<anon>:6         print!("{} ", x);
<anon>:7     }
<anon>:5:5: 7:6 help: see the detailed explanation for E0277
<anon>:5:5: 7:6 note: `[_]` does not have a constant size known at compile-time
<anon>:5:5: 7:6 note: required by `std::iter::IntoIterator::into_iter`
<anon>:5:5: 7:6 error: the trait bound `[_]: std::iter::Iterator` is not satisfied [E0277]
<anon>:5     for x in vec[3..] {
<anon>:6         print!("{} ", x);
<anon>:7     }
<anon>:5:5: 7:6 help: see the detailed explanation for E0277
<anon>:5:5: 7:6 note: `[_]` is not an iterator; maybe try calling `.iter()` or a similar method
<anon>:5:5: 7:6 note: required by `std::iter::IntoIterator::into_iter`

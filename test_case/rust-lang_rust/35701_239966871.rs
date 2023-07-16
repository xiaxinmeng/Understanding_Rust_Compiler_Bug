
main.rs:15:5: 17:6 error: the trait bound `std::ops::RangeFull: std::iter::Iterator` is not satisfied [E0277]
main.rs:15     for i in .. {
               ^
main.rs:15:5: 17:6 help: run `rustc --explain E0277` to see a detailed explanation
main.rs:15:5: 17:6 note: `std::ops::RangeFull` is not an iterator; maybe try calling `.iter()` or a similar method
main.rs:15:5: 17:6 note: required by `std::iter::IntoIterator::into_iter`

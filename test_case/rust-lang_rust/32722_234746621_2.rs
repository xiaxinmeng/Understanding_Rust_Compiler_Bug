
test.rs:6:1: 8:2 error: the trait bound `<<Self as SessionType>::Dual as SessionType>::Dual: Recv` is not satisfied [E0277]
test.rs:6 trait Recv: SessionType
test.rs:7     where Self::Dual: Send {
test.rs:8 }
test.rs:6:1: 8:2 help: run `rustc --explain E0277` to see a detailed explanation
test.rs:6:1: 8:2 help: consider adding a `where <<Self as SessionType>::Dual as SessionType>::Dual: Recv` bound
test.rs:6:1: 8:2 note: required by `Send`
test.rs:10:1: 12:2 error: the trait bound `<<Self as SessionType>::Dual as SessionType>::Dual: Send` is not satisfied [E0277]
test.rs:10 trait Send: SessionType
test.rs:11     where Self::Dual: Recv {
test.rs:12 }
test.rs:10:1: 12:2 help: run `rustc --explain E0277` to see a detailed explanation
test.rs:10:1: 12:2 help: consider adding a `where <<Self as SessionType>::Dual as SessionType>::Dual: Send` bound
test.rs:10:1: 12:2 note: required by `Recv`


error[E0277]: the trait bound `<<Self as SessionType>::Dual as SessionType>::Dual: Recv` is not satisfied
  --> src/main.rs:5:1
   |
5  | / trait Recv: SessionType
6  | |     where Self::Dual: Send {
   | |                           - help: consider further restricting the associated type: `, <<Self as SessionType>::Dual as SessionType>::Dual: Recv`
7  | | }
   | |_^ the trait `Recv` is not implemented for `<<Self as SessionType>::Dual as SessionType>::Dual`
8  | 
9  | / trait Send: SessionType
10 | |     where Self::Dual: Recv {
11 | | }
   | |_- required by `Send`

error[E0277]: the trait bound `<<Self as SessionType>::Dual as SessionType>::Dual: Send` is not satisfied
  --> src/main.rs:9:1
   |
5  | / trait Recv: SessionType
6  | |     where Self::Dual: Send {
7  | | }
   | |_- required by `Recv`
8  | 
9  | / trait Send: SessionType
10 | |     where Self::Dual: Recv {
   | |                           - help: consider further restricting the associated type: `, <<Self as SessionType>::Dual as SessionType>::Dual: Send`
11 | | }
   | |_^ the trait `Send` is not implemented for `<<Self as SessionType>::Dual as SessionType>::Dual`

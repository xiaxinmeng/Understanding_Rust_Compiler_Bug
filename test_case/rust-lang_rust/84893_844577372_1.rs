
error[E0277]: the trait bound `T: MyHash` is not satisfied
   --> test.rs:16:25
    |
16  | pub struct CustomSet<T>(MyHashSet<T>);
    |                         ^^^^^^^^^^^^ the trait `MyHash` is not implemented for `T`
    |
    = note: required because of the requirements on the impl of `Eq` for `MyHashSet<T>`
    = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider further restricting this bound
    |
15  | #[derive(Eq + MyHash)]
    |             ^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.

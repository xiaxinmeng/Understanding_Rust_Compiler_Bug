
[INFO] [stdout] error[E0277]: the trait bound `Continuation: From<!>` is not satisfied
[INFO] [stdout]    --> src/lib.rs:485:13
[INFO] [stdout]     |
[INFO] [stdout] 485 |         chm.for_each(Filter::all(), |_, _| panic!("Oops..."))
[INFO] [stdout]     |             ^^^^^^^^ the trait `From<!>` is not implemented for `Continuation`
[INFO] [stdout]     |
[INFO] [stdout]     = help: the following implementations were found:
[INFO] [stdout]               <Continuation as From<()>>
[INFO] [stdout]               <Continuation as From<std::result::Result<(), E>>>
[INFO] [stdout]     = note: required because of the requirements on the impl of `Into<Continuation>` for `!`

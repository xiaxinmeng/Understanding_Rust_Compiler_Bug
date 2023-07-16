console
error[E0277]: the trait bound `dyn realstd::io::LocalOutput: io::Write` is not satisfied
   --> library/std/src/panicking.rs:216:15
    |
216 |         write(&mut local);
    |               ^^^^^^^^^^ the trait `io::Write` is not implemented for `dyn realstd::io::LocalOutput`
    |
    = note: required because of the requirements on the impl of `io::Write` for `realstd::boxed::Box<dyn realstd::io::LocalOutput>`
    = note: required for the cast to the object type `dyn io::Write`


error[E0277]: the trait bound `IterationControl: std::convert::From<!>` is not satisfied
   --> src/lib.rs:326:40
    |
326 |         match panic::catch_unwind(|| { TargetSharedLibrary::each(|_| panic!("uh oh")); }) {
    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<!>` is not implemented for `IterationControl`
    |
    = help: the following implementations were found:
              <IterationControl as std::convert::From<()>>
    = note: required because of the requirements on the impl of `std::convert::Into<IterationControl>` for `!`
note: required by `SharedLibrary::each`
   --> src/lib.rs:296:5
    |
296 | /     fn each<F, C>(f: F)
297 | |     where
298 | |         F: FnMut(&Self) -> C,
299 | |         C: Into<IterationControl>;
    | |__________________________________^
error: aborting due to previous error

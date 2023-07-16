
error: internal compiler error: ../src/librustc_typeck/check/method/confirm.rs:365: std::ops::Range<usize> was a subtype of std::ops::Range<I> but now is not?
   --> src/lib.rs:326:31
    |
326 |     for (k, colptr) in (0..n).zip(l_colptr.iter_mut()) {
    |                               ^^^

note: the compiler unexpectedly panicked. this is a bug.

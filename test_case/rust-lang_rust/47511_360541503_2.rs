
error: internal compiler error: librustc_typeck/astconv.rs:1212: anonymous bound region BrAnon(0) in return but not args
  --> src/main.rs:47:26
   |
47 | fn f(arg: Iter<A, B>) -> Iter<A, C> {
   |                          ^^^^^^^^^^

note: the compiler unexpectedly panicked. this is a bug.

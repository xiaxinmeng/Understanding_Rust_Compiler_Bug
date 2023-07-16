
   Compiling criterion v0.2.5
warning: unused import: `std::cmp::Ordering`
 --> src/stv.rs:2:5
  |
2 | use std::cmp::Ordering;
  |     ^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(unused_imports)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 30.22s
     Running target/debug/deps/tallyman-2282da47096c9aaf

running 5 tests
test plurality::tests::plurality_test ... ok
test stv::tests::stv_test ... ok
test condorcet::tests::condorcet_test ... ok
test condorcet::tests::condorcet_wikipedia_test ... ok
test stv::tests::stv_wikipedia_test ... FAILED

failures:

---- stv::tests::stv_wikipedia_test stdout ----
thread 'stv::tests::stv_wikipedia_test' panicked at 'assertion failed: `(left == right)`
  left: `{"Scott": 0, "Brad": 1, "Andrea": 2, "Carter": 3, "Matt": 4, "Jennifer": 5, "Delilah": 6}`,
 right: `{"Scott": 0, "Brad": 1, "Andrea": 2, "Jennifer": 3, "Carter": 4, "Matt": 5, "Delilah": 6}`', src/stv.rs:415:9
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.


failures:
    stv::tests::stv_wikipedia_test

test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

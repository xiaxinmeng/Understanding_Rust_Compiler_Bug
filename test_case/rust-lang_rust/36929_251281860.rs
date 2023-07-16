
failures:

---- Statements_and_expressions_16 stdout ----
    error[E0425]: unresolved name `ten_times`
 --> <anon>:3:1
  |
3 | ten_times(move |j| println!("{}, {}", word, j));
  | ^^^^^^^^^ unresolved name

error: aborting due to previous error(s)

thread 'Statements_and_expressions_16' panicked at 'Box<Any>', src/librustc/session/mod.rs:185
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    Statements_and_expressions_16

test result: FAILED. 94 passed; 1 failed; 18 ignored; 0 measured

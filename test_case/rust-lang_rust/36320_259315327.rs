
running 1 test
test main_0 ... FAILED

failures:

---- main_0 stdout ----
        error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `bar`
 --> <anon>:2:9
  |
2 |     foo bar baz
  |         ^^^

error[E0425]: unresolved name `foo`
 --> <anon>:2:5
  |
2 |     foo bar baz
  |     ^^^ unresolved name

error: aborting due to previous error(s)

thread 'main_0' panicked at 'Box<Any>', ../src/librustc/session/mod.rs:191
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    main_0

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured


failures:

---- Lifetimes_1 stdout ----
    error[E0106]: missing lifetime specifier
 --> <anon>:2:45
  |
2 |     fn data_value(data: &str, key: &str) -> &str {
  |                                             ^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `data` or `key`

error: aborting due to previous error(s)

thread 'Lifetimes_1' panicked at 'Box<Any>', src/librustc/session/mod.rs:183
note: Run with `RUST_BACKTRACE=1` for a backtrace.

---- Lifetimes_0 stdout ----
    error: `i` does not live long enough
  --> <anon>:6:14
   |
6  |         r = &i;         // Store reference of i in r
   |              ^ does not live long enough
7  |     }                   // i goes out of scope and is dropped.
   |     - borrowed value only lives until here
...
10 | }
   | - borrowed value needs to live until here

error[E0384]: re-assignment of immutable variable `r`
 --> <anon>:6:9
  |
3 |     let r = &0;         // Introduce reference: r
  |         - first assignment to `r`
...
6 |         r = &i;         // Store reference of i in r
  |         ^^^^^^ re-assignment of immutable variable

error: aborting due to previous error(s)

thread 'Lifetimes_0' panicked at 'Box<Any>', src/librustc/session/mod.rs:183

---- Lifetimes_2 stdout ----
    error[E0269]: not all control paths return a value
 --> <anon>:2:5
  |
2 |     fn data_value<'a,'b>(data: &'a str, key: &'b str) -> &'a str {
  |     ^

error: aborting due to previous error

thread 'Lifetimes_2' panicked at 'Box<Any>', src/librustc_errors/lib.rs:694
thread 'Lifetimes_2' panicked at 'couldn't compile the test', src/librustdoc/test.rs:283


failures:
    Lifetimes_0
    Lifetimes_1
    Lifetimes_2

test result: FAILED. 10 passed; 3 failed; 10 ignored; 0 measured

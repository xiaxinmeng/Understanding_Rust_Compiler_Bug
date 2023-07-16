
failures:

---- Taking_closures_as_arguments_5 stdout ----
    <anon>:4:2: 4:2 error: this file contains an un-closed delimiter
<anon>:4 }
          ^
<anon>:1:11: 1:12 help: did you mean to close this delimiter?
<anon>:1 fn main() {
                   ^
<anon>:3:21: 3:23 error: expected type, found `32`
<anon>:3     where F: Fn(&'a 32) -> i32 {
                             ^~
<anon>:3:23: 3:24 error: expected one of `+`, `,`, `->`, `::`, or `{`, found `)`
<anon>:3     where F: Fn(&'a 32) -> i32 {
                               ^
error: aborting due to previous error(s)
thread 'Taking_closures_as_arguments_5' panicked at 'Box<Any>', src/librustc/session/mod.rs:167
note: Run with `RUST_BACKTRACE=1` for a backtrace.

---- Taking_closures_as_arguments_6 stdout ----
    <anon>:4:2: 4:2 error: this file contains an un-closed delimiter
<anon>:4 }
          ^
<anon>:1:11: 1:12 help: did you mean to close this delimiter?
<anon>:1 fn main() {
                   ^
<anon>:3:29: 3:31 error: expected type, found `32`
<anon>:3     where F: for<'a> Fn(&'a 32) -> i32 {
                                     ^~
<anon>:3:31: 3:32 error: expected one of `+`, `,`, `->`, `::`, or `{`, found `)`
<anon>:3     where F: for<'a> Fn(&'a 32) -> i32 {
                                       ^
error: aborting due to previous error(s)
thread 'Taking_closures_as_arguments_6' panicked at 'Box<Any>', src/librustc/session/mod.rs:167


failures:
    Taking_closures_as_arguments_5
    Taking_closures_as_arguments_6

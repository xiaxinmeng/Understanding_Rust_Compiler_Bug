
$ rustc --cfg showbug2 a.rs
a.rs:18:10: 18:16 error: wrong number of lifetime parameters: expected 1 but found 0
a.rs:18 impl<'a> B<int> { pub fn foo() -> int { 5 } }
                 ^~~~~~
error: aborting due to previous error

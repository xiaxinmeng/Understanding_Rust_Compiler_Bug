
test.rs:13:15: 13:16 error: loan of mutable local variable as mutable conflicts with prior loan
test.rs:13     b.next1(); b.next1();
                          ^
test.rs:13:4: 13:5 note: prior loan as mutable granted here
test.rs:13     b.next1(); b.next1();
               ^
error: aborting due to previous error

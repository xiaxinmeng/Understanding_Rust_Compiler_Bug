
hello.rs:1:1: 1:27 warning: function is never used: `bye`, #[warn(dead_code)] on by default
hello.rs:1 fn bye() -> ! { panic!() }
           ^~~~~~~~~~~~~~~~~~~~~~~~~~
hello.rs:3:1: 3:29 warning: function is never used: `warns`, #[warn(dead_code)] on by default
hello.rs:3 fn warns() { bye(); 1 + 1; }
           ^~~~~~~~~~~~~~~~~~~~~~~~~~~~
hello.rs:3:21: 3:27 warning: unreachable statement, #[warn(unreachable_code)] on by default
hello.rs:3 fn warns() { bye(); 1 + 1; }
                               ^~~~~~
hello.rs:5:1: 5:34 warning: function is never used: `does_not_warn`, #[warn(dead_code)] on by default
hello.rs:5 fn does_not_warn() { 1 + bye(); }
           ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

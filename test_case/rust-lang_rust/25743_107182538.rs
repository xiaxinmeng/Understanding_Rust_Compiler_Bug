
range.rs:4:9: 4:19 error: mismatched types in range:
 expected char,
    found integral variable [E0211]
range.rs:4         'a' ... 10 => (),
                   ^~~~~~~~~~
range.rs:5:9: 5:19 error: mismatched types in range:
 expected integral variable,
    found char [E0211]
range.rs:5         10 ... 'z' => (),
                   ^~~~~~~~~~
range.rs:6:9: 6:19 error: only char and numeric types are allowed in range patterns
 start type: &'static str
 end type: _ [E0029]
range.rs:6         "what" ... 10 => (),
                   ^~~~~~~~~~
range.rs:6:9: 6:19 help: run `rustc --explain E0029` to see a detailed explanation
range.rs:7:9: 7:26 error: only char and numeric types are allowed in range patterns
 start type: &'static str
 end type: &'static str [E0029]
range.rs:7         "what" ... "well" => (),
                   ^~~~~~~~~~~~~~~~~
range.rs:7:9: 7:26 help: run `rustc --explain E0029` to see a detailed explanation
range.rs:8:16: 8:25 error: only char and numeric types are allowed in range patterns
 start type: _
 end type: &'static str [E0029]
range.rs:8         10 ... "what" => ()
                          ^~~~~~~~~
range.rs:8:16: 8:25 help: run `rustc --explain E0029` to see a detailed explanation
error: aborting due to 5 previous errors

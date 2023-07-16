
test.rs:4:21: 6:6 error: mismatched types: expected `()` but found `<generic integer #1>` (expected () but found integral variable)
test.rs:4     } else if false {
test.rs:5         1
test.rs:6     };
test.rs:2:13: 6:6 error: if and else have incompatible types: expected `<generic integer #0>` but found `()` (expected integral variable but found ())
test.rs:2     let a = if true {
test.rs:3         0 
test.rs:4     } else if false {
test.rs:5         1
test.rs:6     };


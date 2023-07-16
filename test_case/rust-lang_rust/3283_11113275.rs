
/Users/tchevalier/rust2/src/test/run-pass/issue-3283.rs:4:11: 6:5 error: mismatched types: expected `&fn()` but found `@fn()` (expected & closure, found @ closure)
/Users/tchevalier/rust2/src/test/run-pass/issue-3283.rs:4     do blk {
/Users/tchevalier/rust2/src/test/run-pass/issue-3283.rs:5         || { *statep = 1; }
/Users/tchevalier/rust2/src/test/run-pass/issue-3283.rs:6     }

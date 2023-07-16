
---- [run-pass] run-pass/issue-14393.rs ----    
error: compilation failed!
thread 'rustc' panicked at 'assertion failed: rows.iter().all(|r| r.len() == v.len())', src/librustc/middle/check_match.rs:666

---- [run-pass] run-pass/vec-matching.rs ----
error: test run failed!
thread '<main>' panicked at 'assertion failed: `(left == right)` (left: `1`, right: `0`)', /home/eddy/rust/src/test/run-pass/vec-matching.rs:125

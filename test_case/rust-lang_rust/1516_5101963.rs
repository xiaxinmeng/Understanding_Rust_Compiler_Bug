
./src/test/run-pass/issue-1516.rs:1:47: 1:60 error: mismatched types: expected `fn@(str) -> !` but found `fn@(_|_) -> _|_` (return-by-value function found where non-returning function was expected)
./src/test/run-pass/issue-1516.rs:1 fn main() {  let early_error: fn@(str) -> !  = {|msg| fail }; }

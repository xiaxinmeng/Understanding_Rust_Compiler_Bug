
% rustc -Z orbit ../src/test/run-pass/issue-4401.rs
% ./issue-4401 
999999
% ./x86_64-apple-darwin/stage1/bin/rustc -Z orbit ../src/test/run-pass/issue-4401.rs
% ./issue-4401 

thread 'main' has overflowed its stack
fatal runtime error: stack overflow
Abort trap: 6
% 


% cd ../../rust-baseline/objdir-opt/
% git log -1 --oneline
6800538 Auto merge of #26984 - nham:errorck-ignore-long-diag, r=brson
% ./x86_64-apple-darwin/stage2/bin/rustc --version
rustc 1.3.0-dev (680053848 2015-07-13)
% time ./x86_64-apple-darwin/stage2/bin/rustc /tmp/driver_run.rs -o driver_run 

real    0m14.264s
user    0m14.073s
sys 0m0.187s
% time ./driver_run 

real    0m0.279s
user    0m0.262s
sys 0m0.003s
% cd ../../rust-rw-match/objdir-opt/
% git log -1 --oneline
54b4a0f Fix lint.
% ./x86_64-apple-darwin/stage2/bin/rustc --version
rustc 1.3.0-dev (54b4a0f5b 2015-07-16)
% time ./x86_64-apple-darwin/stage2/bin/rustc /tmp/driver_run.rs -o driver_run 

real    0m13.745s
user    0m13.523s
sys 0m0.219s
% time ./driver_run 

real    0m9.235s
user    0m9.219s
sys 0m0.006s
% 

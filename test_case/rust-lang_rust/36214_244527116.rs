
[root@li1424-173 brainmunch]# time ../rust/stackless/build/x86_64-unknown-linux-gnu/stage1/bin/rustc -O -Z time-passes src/main.rs | grep expansion
time: 8.082; rss: 68MB  expansion

real    0m9.118s
user    0m8.993s
sys     0m0.100s
[root@li1424-173 brainmunch]# time ../rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc -o master -O -Z time-passes src/main.rs | grep expansion
time: 8.501; rss: 67MB  expansion

real    0m9.575s
user    0m9.450s
sys     0m0.103s

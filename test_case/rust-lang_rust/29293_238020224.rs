
/home/z $ time RUST_BACKTRACE=0 rustc -vv
!! executing '/home/z/build/1nonpkgs/rust/rust//x86_64-unknown-linux-gnu/stage2/bin//rustc' with args: '-vv'
error: Option 'verbose' given more than once. 

real    0m0.062s
user    0m0.020s
sys 0m0.041s
-----------
z@myzee 2016/08/06 15:04:23 -bash4.3.46 t:3 j:0 d:3 pp:1119 p:4019 ut1603
!13231 19 101  4.7.0-ga157b3a #6 SMP PREEMPT Sat Aug 6 12:11:16 EEST 2016
/home/z
/home/z $ time RUST_BACKTRACE=1 rustc -vv
!! executing '/home/z/build/1nonpkgs/rust/rust//x86_64-unknown-linux-gnu/stage2/bin//rustc' with args: '-vv'
error: Option 'verbose' given more than once. 

real    0m0.454s
user    0m0.341s
sys 0m0.080s

$ rustc -Vv
!! executing '/home/z/build/1nonpkgs/rust/rust//x86_64-unknown-linux-gnu/stage2/bin//rustc' with args: '-Vv'
rustc 1.12.0-dev (2c1612c62 2016-08-01)
binary: rustc
commit-hash: 2c1612c62aa59e40cf1a4bddde943938e0147eec
commit-date: 2016-08-01
host: x86_64-unknown-linux-gnu
release: 1.12.0-dev

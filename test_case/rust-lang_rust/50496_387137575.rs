
$ rustup-toolchain-install-master c1168be5360f17516b233be85ebb193bb4e612bf a2726846f6d6280b752019472b6bd791c0752006
$ pwd
/home/alex/code/rustc-perf
$ git rev-parse HEAD
254d28f6181cd5d20cedcd3fa9ae36df847da958
$ time rustc +a2726846f6d6280b752019472b6bd791c0752006 collector/benchmarks/tuple-stress/src/main.rs
rustc +a2726846f6d6280b752019472b6bd791c0752006   4.58s user 0.72s system 100% cpu 5.264 total
$ time rustc +c1168be5360f17516b233be85ebb193bb4e612bf collector/benchmarks/tuple-stress/src/main.rs
^C
rustc +c1168be5360f17516b233be85ebb193bb4e612bf   109.70s user 0.60s system 99% cpu 1:50.31 total

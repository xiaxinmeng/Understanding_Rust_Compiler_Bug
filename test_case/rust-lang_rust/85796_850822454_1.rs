console
$ rustc -O -Zmir-opt-level=2 a.rs && ./a
3
$ rustc -O -Zmir-opt-level=3 a.rs && ./a
0

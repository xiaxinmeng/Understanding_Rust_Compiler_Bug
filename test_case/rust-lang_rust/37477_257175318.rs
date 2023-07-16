
$ touch test.rs
$ perf record rustc test.rs
$ perf report --stdio --header

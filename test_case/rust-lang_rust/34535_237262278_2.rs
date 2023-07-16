
$ for i in $(seq 1 1000); do echo "fn x$i() { a }" >> t.rs; done
$ rustc -Z time-passes t.rs
time: 0.005; rss: 63MB  parsing
[...]
time: 0.000; rss: 102MB AST validation

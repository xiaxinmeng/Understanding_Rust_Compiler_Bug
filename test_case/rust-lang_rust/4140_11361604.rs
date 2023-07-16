
$ ./x86_64-apple-darwin/stage2/bin/rustc -Z verbose -o /tmp/test1 ./src/test/bench/shootout-nbody.rs
warning: no debug symbols in executable (-arch x86_64)
$ ./x86_64-apple-darwin/stage2/bin/rustc -Z verbose -o /tmp/test2 ./src/test/run-pass/stat.rs
warning: no debug symbols in executable (-arch x86_64)
$ cd /tmp
$ ./test1
-0.169075
-0.169080
$ ./test2
$

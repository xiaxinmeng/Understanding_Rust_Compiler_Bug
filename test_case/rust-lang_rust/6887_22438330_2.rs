
-*- mode: compilation; default-directory: "/tmp/show-el-fail/" -*-
Compilation started at Sat Aug 10 13:35:58

make
rustc -o nonexhaust.bin nonexhaust.rs
nonexhaust.rs:2:4: 5:5 error: non-exhaustive patterns: None not covered
nonexhaust.rs:2     match x {
nonexhaust.rs:3         Some(Some(n)) => n,
nonexhaust.rs:4         Some(None) => 3
nonexhaust.rs:5     }
error: aborting due to previous error
make: *** [nonexhaust.bin] Error 101

Compilation exited abnormally with code 2 at Sat Aug 10 13:35:59



-*- mode: compilation; default-directory: "/tmp/show-el-fail/" -*-
Compilation started at Sat Aug 10 13:52:17

make
rustc -o nonexhaust.bin nonexhaust.rs
nonexhaust.rs:2:10: 2:17 error: expected `;` or `}` after expression but found `problem`
nonexhaust.rs:2     other problem here
                          ^~~~~~~
make: *** [nonexhaust.bin] Error 101

Compilation exited abnormally with code 2 at Sat Aug 10 13:52:17

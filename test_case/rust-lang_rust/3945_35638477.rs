
$ rustc -v
 rustc 0.10-pre (0cc8ba0 2014-02-19 21:56:51 -0800)
 host: x86_64-unknown-linux-gnu
$ rustc --emit=ir mainpage.rs
$ ls
 mainpage.ll  mainpage.rs
$ rustc -C save-temps mainpage.rs 
$ ls
 mainpage  mainpage.bc  mainpage.metadata.o  mainpage.no-opt.bc  mainpage.o  mainpage.rs

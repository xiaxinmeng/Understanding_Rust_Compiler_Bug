
$ mkdir $(echo 'foo\xff')    
$ touch $(echo 'foo\xff')/foo.rs 
$ rustc $(echo 'foo\xff')/foo.rs 
error: couldn't read "foo\u{fffd}/foo.rs": couldn't open path as file (no such file or directory (No such file or directory); path=foo�/foo.rs; mode=open; access=read)
$ rustc -V
rustc 1.0.0-nightly (1d00c545e 2015-01-30 19:56:34 +0000)

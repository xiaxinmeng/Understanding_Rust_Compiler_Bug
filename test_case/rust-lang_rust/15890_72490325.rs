
$ mkdir $(echo -e 'foo\xff')
$ touch $(echo -e 'foo\xff')/foo.rs
$ echo "fn main() {}" > $(echo -e 'foo\xff')/foo.rs
$ rustc $(echo -e 'foo\xff')/foo.rs
error: couldn't read "foo\u{fffd}/foo.rs": couldn't open path as file (no such file or directory (No such file or directory); path=foo�/foo.rs; mode=open; access=read)


$ rustc --crate-type=rlib lib.rs & rustc --crate-type=staticlib lib.rs
[1] 5798
error: failed to remove lib.0.o: couldn't unlink path (no such file or directory (No such file or directory); path=lib.0.o)
error: aborting due to previous error


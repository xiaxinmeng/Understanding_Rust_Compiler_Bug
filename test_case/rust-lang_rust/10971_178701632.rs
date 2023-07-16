
$ touch lib.rs
$ rustc --crate-type=rlib lib.rs & rustc --crate-type=staticlib lib.rs
[1] 17820
error: failed to build archive: No such file or directory
[1]+  Done                    rustc --crate-type=rlib lib.rs

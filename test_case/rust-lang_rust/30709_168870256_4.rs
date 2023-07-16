 text
$ rustc A.rs
$ rustc B.rs -L .
$ rustc C.rs -L .
$ rustc D.rs -L .
$ ./D # runs fine
$ $EDITOR A.rs # uncomment `println!`
$ rustc A.rs
$ rustc C.rs -L .
C.rs:2:1: 2:16 error: found possibly newer version of crate `A` which `B` depends on [E0460]
C.rs:2 extern crate B;
       ^~~~~~~~~~~~~~~
C.rs:2:1: 2:16 note: perhaps this crate needs to be recompiled?
C.rs:2 extern crate B;
       ^~~~~~~~~~~~~~~
C.rs:2:1: 2:16 note: crate `A` path #1: /home/huon/tmp/libA.rlib
C.rs:2:1: 2:16 note: crate `B` path #1: /home/huon/tmp/libB.rlib
error: aborting due to previous error

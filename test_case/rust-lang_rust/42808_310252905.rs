
$ rustc +stable -V
rustc 1.18.0 (03fc9d622 2017-06-06)
$ rustc +stable foo.rs && strip foo && ls -alh foo
-rwxrwxr-x 1 alex alex 316K Jun 21 18:47 foo
$ rustc +beta -V
rustc 1.19.0-beta.2 (a175ee509 2017-06-15)
$ rustc +beta foo.rs && strip foo && ls -alh foo
-rwxrwxr-x 1 alex alex 368K Jun 21 18:47 foo
$ rsutc +nightly -V
rustc 1.20.0-nightly (622e7e648 2017-06-21)
$ rustc +nightly foo.rs && strip foo && ls -alh foo
-rwxrwxr-x 1 alex alex 324K Jun 21 18:47 foo

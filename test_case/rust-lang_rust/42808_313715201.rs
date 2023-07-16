
$ rustc +nightly -V
rustc 1.20.0-nightly (696412de7 2017-07-06)
$ rustc +nightly foo.rs -O && strip -g ./foo && ls -alh foo
-rwxrwxr-x 1 alex alex 483K Jul  7 08:29 foo
$ rustc +nightly-2017-06-01 foo.rs -O && strip -g ./foo && ls -alh foo
-rwxrwxr-x 1 alex alex 476K Jul  7 08:29 foo

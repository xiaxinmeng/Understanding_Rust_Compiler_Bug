
$ rustc +nightly foo.rs -O && strip -g ./foo && ls -alh foo
-rwxrwxr-x 1 alex alex 459K Jul  7 08:30 foo
$ rustc +nightly-2017-06-01 foo.rs -O && strip -g ./foo && ls -alh foo
-rwxrwxr-x 1 alex alex 456K Jul  7 08:30 foo

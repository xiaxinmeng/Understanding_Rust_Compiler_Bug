
$ rustc lib.rs
$ rustc a.rs -L .
error: linking with `gcc` failed: exit code: 1
...
note: a.o:(.text+0x1a): undefined reference to `func1'

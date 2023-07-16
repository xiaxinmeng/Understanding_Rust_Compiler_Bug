console
$ rustc --target x86_64-unknown-linux-musl hello.rs && ./hello
Segmentation fault (core dumped)
$ rustc -C relocation-model=static --target x86_64-unknown-linux-musl hello.rs && ./hello
Hello, World!

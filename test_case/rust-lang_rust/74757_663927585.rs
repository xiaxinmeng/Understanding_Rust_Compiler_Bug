
$ rustc --version; and rustc --target=x86_64-unknown-linux-musl test.rs -C relocation-model=static; and ./test
rustc 1.46.0-beta.2 (6f959902b 2020-07-23)
$ echo $status
0

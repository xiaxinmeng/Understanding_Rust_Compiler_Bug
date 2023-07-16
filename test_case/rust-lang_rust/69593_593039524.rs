bash
$ cargo build --release --bin struct && cp target/release/struct ./struct-stable && cargo +nightly build --release --bin struct && cp target/release/struct ./struct-nightly
$ ls -l
total 5352
-rw-rw-r--. 1 josh josh     150 Feb 29 04:28 Cargo.lock
-rw-rw-r--. 1 josh josh     118 Feb 29 14:07 Cargo.toml
-rw-rw-r--. 1 josh josh   10803 Feb 29 08:10 LICENSE-APACHE.txt
-rw-rw-r--. 1 josh josh   10803 Feb 29 08:10 LICENSE-MIT.txt
-rw-rw-r--. 1 josh josh     121 Feb 29 09:45 README.md
drwxrwxr-x. 3 josh josh    4096 Feb 29 05:34 src
-rwxrwxr-x. 1 josh josh 2781704 Feb 29 18:20 struct-nightly
-rwxrwxr-x. 1 josh josh 2646504 Feb 29 18:20 struct-stable
drwxrwxr-x. 5 josh josh    4096 Feb 29 04:36 target
$ ./struct-stable
1999999999 in 1025 ms
$ ./struct-nightly
1999999999 in 1283 ms

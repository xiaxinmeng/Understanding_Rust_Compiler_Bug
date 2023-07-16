
$ RUST_SRC_REPO=~/projects/rust/rust cargo-bisect-rustc --preserve --start=2021-01-01 --script=./script.sh

$ cat script.sh
#!/bin/sh
rustc --emit mir -Copt-level=3 main.rs
grep "drop\(_3\) -> bb4" main.mir

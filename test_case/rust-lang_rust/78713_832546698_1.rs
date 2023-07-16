
$ RUST_SRC_REPO=~/rust cargo-bisect-rustc --preserve --start=2020-06-01 --end=2020-10-08 --script=./test.sh

$ cat ./test.sh
#!/bin/sh
RUST_BACKTRACE=1 cargo run 2>&1 | grep __libc_start_main

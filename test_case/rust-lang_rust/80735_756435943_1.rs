bash
$ cargo clean
$ strace -ff cargo build 2>&1 | grep crt1

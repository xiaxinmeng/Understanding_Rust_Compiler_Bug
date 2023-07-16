 shell
$ RUST_BACKTRACE=1 rustc - <<< 'fn main() { &*b"foo"; }'
$ echo $?
0

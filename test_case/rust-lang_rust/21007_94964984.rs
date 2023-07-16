 shell
$ RUST_BACKTRACE=1 RUST_LOG=rustc_trans::trans::expr rustc - <<< 'fn main() { println!("{}", 1u8); }'
$ echo $?
0


$ RUST_BACKTRACE=1 rustc - --out-dir=random_directory_that_does_not_exist/ --emit=llvm-ir <<< 'fn main(){}'
LLVM ERROR: IO failure on output stream.

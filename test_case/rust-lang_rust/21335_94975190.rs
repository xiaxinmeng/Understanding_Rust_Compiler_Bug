 shell
$ RUST_BACKTRACE=1 rustc - --out-dir=random_directory_that_does_not_exist/ --emit=llvm-bc <<< 'fn main(){}'
error: could not copy "target/rust_out.0.bc" to "target/rust_out.bc": the source path is not an existing file
error: failed to remove target/rust_out.0.bc: No such file or directory (os error 2)
error: aborting due to 2 previous errors

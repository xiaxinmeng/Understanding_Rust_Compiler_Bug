shell
$ echo 'fn main(){}' > a.rs
$ rustc -Cpanic=abort -Cprefer-dynamic a.rs
error: the linked panic runtime `panic_unwind` is not compiled with this crate's panic strategy `abort`

error: aborting due to previous error

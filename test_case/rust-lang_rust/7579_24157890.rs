
$ mkdir noperm && cd noperm/
$ echo 'fn main() {}' > test.rs
$ chmod -w .
$ rustc test.rs
error: Could not write output: Error opening output file 'test.o'


$ cat test.rs
fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);
}

$ rustc --version --verbose
rustc 1.55.0 (c8dfcfe04 2021-09-06)
binary: rustc
commit-hash: c8dfcfe046a7680554bf4eb612bad840e7631c4b
commit-date: 2021-09-06
host: x86_64-unknown-linux-gnu
release: 1.55.0
LLVM version: 12.0.1
$ rustc -o test -g ./test.rs
$

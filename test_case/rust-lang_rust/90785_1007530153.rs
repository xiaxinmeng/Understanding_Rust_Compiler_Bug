console
$ cat src/lib.rs
#[test]
fn test() {
    use std::io::Write;

    eprintln!("eprintln");
    println!("println");
    std::io::stdout().write_all(b"stdout\n").unwrap();
    std::io::stderr().write_all(b"stderr\n").unwrap();
}
$ cargo test --tests
    Finished test [unoptimized] target(s) in 0.00s
     Running unittests (target/debug/deps/a-7d5a981fc2a1d521)

running 1 test
stdout
stderr
test test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ cargo --version
cargo 1.58.0-nightly (7f08ace4f 2021-11-24)

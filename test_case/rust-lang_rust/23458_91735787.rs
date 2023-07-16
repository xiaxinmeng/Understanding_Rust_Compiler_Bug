
Abhishek@Abhisheks-MacBook-Pro:~/src/rust (master %=)$ ./x86_64-apple-darwin/stage2/bin/rustc crash.rs
crash.rs:5:9: 5:24 error: <inline asm>:1:2: error: too few operands for instruction
        int
        ^

crash.rs:5         asm!("int $3");
                   ^~~~~~~~~~~~~~~
error: aborting due to previous error
Abhishek@Abhisheks-MacBook-Pro:~/src/rust (master %=)$ ./x86_64-apple-darwin/stage2/bin/rustc --version --verbose
rustc 1.0.0-dev (c897ac04e 2015-04-10) (built 2015-04-10)
binary: rustc
commit-hash: c897ac04e2ebda378fd9e38f6ec0878ae3a2baf7
commit-date: 2015-04-10
build-date: 2015-04-10
host: x86_64-apple-darwin
release: 1.0.0-dev
Abhishek@Abhisheks-MacBook-Pro:~/src/rust (master %=)$ cat crash.rs
#![feature(asm)]

fn main() {
    unsafe {
        asm!("int $3");
    }
}

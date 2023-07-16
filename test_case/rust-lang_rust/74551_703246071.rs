
hakua:/tmp% cat hello.rs 
fn main() {
    println!("Hello, world!");
}
hakua:/tmp% /tmp/rust-1.44.1/bin/rustc -C lto hello.rs
Segmentation fault
hakua:/tmp% /tmp/rust-1.45.2/bin/rustc -C lto hello.rs
hakua:/tmp% /tmp/rust-nigthly/bin/rustc -C lto hello.rs
hakua:/tmp% 

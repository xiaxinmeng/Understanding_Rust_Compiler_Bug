
% rm -f /cores/*
% ls /cores/
% cat hello.rs
fn main() {
    println!("Hello World");
}
% ./build/x86_64-apple-darwin/stage1/bin/rustc hello.rs 
% ls /cores/
% ./build/x86_64-apple-darwin/stage1/bin/rustc hello.rs -C debuginfo=2
% ls /cores/
core.97645
% 

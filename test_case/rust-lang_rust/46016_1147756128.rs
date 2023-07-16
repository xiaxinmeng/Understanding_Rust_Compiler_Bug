rust
#![feature(unix_sigpipe)]

#[unix_sigpipe = "sig_dfl"]
fn main() {
    loop {
        println!("hello world");
    }
}

rust
pub fn main() {
    loop {
        std::process::Command::new("test").output().unwrap_or(continue);
//      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// warning: unreachable expression
        println!("done");
    }
}

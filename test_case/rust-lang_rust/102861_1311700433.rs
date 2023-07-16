rust
fn main() {
    eprint!("Hello stderr");
    print!("Hello stdout");
    std::thread::sleep(std::time::Duration::from_secs(100));
}

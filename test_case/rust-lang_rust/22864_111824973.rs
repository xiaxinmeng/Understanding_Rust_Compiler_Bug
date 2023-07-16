
fn main() {
    let f = || || 0;
    std::thread::spawn(f());
}

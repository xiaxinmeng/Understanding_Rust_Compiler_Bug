 rust
fn main() {
    let arr : ~[&str] = std::os::args()[1].split_str("::").collect();
    std::io::println("first " + arr[0]);
    std::io::println("first again " + arr[0]);
}

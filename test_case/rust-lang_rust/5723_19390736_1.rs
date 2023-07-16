 rust
fn main() {
    fn its97<T>(f: &fn(@Reader) -> T) -> T {
        let s = std::str::from_bytes([97]);
        std::io::with_str_reader(s, f)
    }

    fn consumer(r: @std::io::Reader) -> int { r.read_byte() }
    println(fmt!("%d", its97(consumer)));

    let long_lived_reader = its97(|r| r);
    println(fmt!("%d", consumer(long_lived_reader)));
}

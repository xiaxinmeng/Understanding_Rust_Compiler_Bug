rust
use memchr::memchr;

fn main() {
    let haystack = "abcdefghijklmnopqrstuvwxyz".repeat(15);

    for _ in 0..100_000_000 {
        assert_eq!(None, memchr(b'@', haystack.as_bytes()));
    }
}

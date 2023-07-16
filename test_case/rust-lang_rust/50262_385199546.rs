Rust
enum Empty {}

fn zzz() {}

fn main() {
    let first: *const Empty = 1 as *const _;

    zzz();
}

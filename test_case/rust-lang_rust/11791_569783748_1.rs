rust
const TEN: u32 = 10;

fn main() {
    let x: u64 = 20;

    match x {
        a if a == TEN as u64 => { /* ... */ },
        _                    => { /* ... */ },
    }
}

 rust
fn myfn(a: u8) { loop {} }

fn main() {
    let mut q = myfn;
    for i in 0..1000_000_000 {
        q.insert(i, [0u8; 65536]);
    }
}

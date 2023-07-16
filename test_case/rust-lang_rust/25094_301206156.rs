 rust
fn total1(a: &[u32]) -> u32 { // OK
    a.iter().sum()
}

fn total3(a: &[u32]) -> u32 { // Error
    a.iter().sum() + 5u32
}

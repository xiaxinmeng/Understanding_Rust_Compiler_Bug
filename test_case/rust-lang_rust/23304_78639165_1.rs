 rust
#[repr(u32)] enum X { A = 0u32 | 1u32, B }

#[repr(u64)] enum Y { A = X::A as u64, B }

fn main() { }

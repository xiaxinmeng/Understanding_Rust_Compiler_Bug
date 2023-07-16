 rust
macro_rules! build_struct {
    ($s:ident) => {{
        $s { key: 12345 }
    }}
}

struct X { key: u32 }

fn main() {
    let x = build_struct!(X);
}

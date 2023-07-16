rust
fn deref<'x>(v: &'x u32) -> u32 {
//       ^^ lifetime name `'x` only used once
    *v
}

fn main() { }

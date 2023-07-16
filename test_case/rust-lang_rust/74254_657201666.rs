rust
fn default_array<T, const N: usize>() -> [T; N]
where
    [T; N]: Default
{
    Default::default()
}

fn main() {
    let _: [u8; 4] = default_array();
}

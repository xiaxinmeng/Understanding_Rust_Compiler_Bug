 Rust
fn main() {
    const FNV_PRIME: u64 = 1099511628211;
    let mut h = 14695981039346656037;
    h = (h ^ (10 as u64)).wrapping_mul(FNV_PRIME);
    h = (h ^ (50 as u64)).wrapping_mul(FNV_PRIME);
    h = (h ^ (60 as u64)).wrapping_mul(FNV_PRIME);
    println!("{}", h == 3470322685770408467);
}

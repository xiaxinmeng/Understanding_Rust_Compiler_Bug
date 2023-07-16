rust
const CONFIG = ProptestConfig { fork: true, timeout: 1000, ..ProptestConfig::default() };

#[proptest(CONFIG)]
mod tests {
// Ideally we would be able to remove `tests` from here and just write `mod { .. }` instead.

    fn first_test(n: u64) { ... }

    fn second_test(n: u64) { ... }
}

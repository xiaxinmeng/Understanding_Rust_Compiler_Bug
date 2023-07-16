rust
proptest! {
    #![proptest_config(ProptestConfig {
        fork: true,
        timeout: 1000,
        .. ProptestConfig::default()
    })]

    fn first_test(n: u64) { ... }

    fn second_test(n: u64) { ... }
}

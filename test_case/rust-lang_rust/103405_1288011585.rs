rust
fn shoule_match_ok() {
    #[cfg(feature = "full")]
    {
        let a = 1;
        let b = 2;
        if match a {
            1 if b == 1 => true,
            _ => false,
        } && if a > 1 { true } else { false }
        {
            true
        }
    }
}

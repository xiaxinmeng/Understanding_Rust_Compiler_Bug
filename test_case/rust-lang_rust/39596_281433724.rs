
fn sort_key(s: &str) -> (&str, u64) {
    let split = s.bytes().rposition(|b| b < b'0' || b'9' < b).unwrap_or(0);
    // get rid of leading zeroes
    let split = split + s[split..].bytes().position(|b| b != b'0').unwrap_or(0);
    let strkey = &s[..split];
    let intkey = s[split..].parse().unwrap_or(0);
    (strkey, intkey)
}

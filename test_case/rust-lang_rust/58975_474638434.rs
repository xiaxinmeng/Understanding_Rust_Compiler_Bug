rust
fn is_luhn_valid(value: &str) -> bool {
    value
        .chars()
        .rev()
        .map(|c| c.to_digit(10))
        .enumerate()
        .map(|(idx, n)| {
            if idx % 2 != 0 {
                n.map(|v| {
                    let v = v * 2;
                    if v > 9 {
                        v - 9
                    } else {
                        v
                    }
                })
            } else {
                n
            }
        })
        .sum::<Option<u32>>()
        .map(|total| total % 10 == 0)
        .unwrap_or(false)
}

fn main() {
    dbg!(is_luhn_valid("4111111111111111"));
    dbg!(is_luhn_valid("5454545454545454"));
}

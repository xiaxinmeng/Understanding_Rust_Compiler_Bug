rust
fn is_luhn_valid(input: &str) -> bool {
    match luhn_sum(input) {
        Some(sum) => sum % 10 == 0,
        None => false,
    }
}

fn luhn_sum(input: &str) -> Option<u32> {
    let mut sum = 0;

    for (idx, ch) in input.chars().rev().enumerate() {
        let digit = ch.to_digit(10)?;

        if idx % 2 == 0 {
            sum += digit;
        } else if digit * 2 <= 9 {
            sum += digit * 2;
        } else {
            sum += digit * 2 - 9;
        }
    }

    Some(sum)
}

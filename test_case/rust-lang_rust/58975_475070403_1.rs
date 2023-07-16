rust
fn is_luhn_valid(input: &str) -> bool {
    let mut sum = 0;

    for (idx, ch) in input.chars().rev().enumerate() {
        let digit = match ch.to_digit(10) {
            Some(digit) => digit,
            None => return false,
        };

        if idx % 2 == 0 {
            sum += digit;
        } else if digit * 2 <= 9 {
            sum += digit * 2;
        } else {
            sum += digit * 2 - 9;
        }
    }

    sum % 10 == 0
}

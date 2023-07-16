 rust
loop {
    match c {
        // ...
        '0'...'9' | 'a'...'f' if valid_digit(c, radix) => {/* parse digit */},
        'e' | 'f' | 's' | 'd' | 'l' if is_decimal_literal() => {/* parse exponential marker */},
        '0'...'9' | 'a'...'f' => {/* return error, bad digit */},
        _ => break
    }
}

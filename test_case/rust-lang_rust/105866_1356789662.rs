
fn stripe(x: u8, mask: u8) -> Option<u8> {
    if x < 1 || x >= 5 { return None };

    Some(mask)
}

fn next(x: &mut u8, mask: &mut u8) -> Option<u8> {
    if *x >= 6 {
        None
    } else {
        let result = stripe(*x, *mask);

        *x += 1;
        if result.is_some() { *mask <<= 1 };

        Some(result.unwrap_or(0))
    }
}

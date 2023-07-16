rust
#[inline(never)]
fn parse_hex<'a>(input: &'a [u8]) -> Option<usize> {
    let mut s = input;
    let mut result: Option<usize> = None;
    loop {
        if let Some((&c, remainder)) = s.split_first() {
            let d = match c {
                d @ b'0'...b'9' => d - b'0',
                d @ b'a'...b'f' => d + 10 - b'a',
                d @ b'A'...b'F' => d + 10 - b'A',
                _ => break,
            };
            result = Some(match result {
                Some(r) => r.checked_mul(16usize)
                    .and_then(|r| r.checked_add(d as usize))?,
                None => d as usize,
            });
            s = remainder;
        } else {
            break;
        }
    }
    result
}

fn main() {
    for _ in 1..10000000 {
        match parse_hex("85af342b1".as_bytes()) {
            Some(_) => continue,
            _ => break,
        }
    }
}

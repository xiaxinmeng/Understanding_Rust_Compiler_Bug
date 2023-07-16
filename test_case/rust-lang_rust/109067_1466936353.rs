rust
use std::ops::Range;
use std::str::Chars;

#[derive(Debug)]
enum EscapeError {
    UnskippedWhitespaceWarning,
}

fn scan_escape(chars: &mut Chars<'_>) -> Result<char, EscapeError> {
    let res = match chars.next().unwrap() {
        _ => panic!("invalid"),
    };
    // Ok(res)
}

fn unescape_str_or_byte_str<F>(src: &str, callback: &mut F)
where
    F: FnMut(Range<usize>, Result<char, EscapeError>),
{
    let mut chars = src.chars();

    // The `start` and `end` computation here is complicated because
    // `skip_ascii_whitespace` makes us to skip over chars without counting
    // them in the range computation.
    while let Some(c) = chars.next() {
        let start = src.len() - chars.as_str().len() - c.len_utf8();
        let res = match c {
            '\\' => {
                match chars.clone().next() {
                    Some('\n') => {
                        // Rust language specification requires us to skip whitespaces
                        // if unescaped '\' character is followed by '\n'.
                        // For details see [Rust language reference]
                        // (https://doc.rust-lang.org/reference/tokens.html#string-literals).
                        skip_ascii_whitespace(&mut chars, start, callback);
                        continue;
                    }
                    _ => scan_escape(&mut chars),
                }
            }
            _ => Ok(c)
        };
        let end = src.len() - chars.as_str().len();
        callback(start..end, res);
    }

    fn skip_ascii_whitespace<F>(chars: &mut Chars<'_>, start: usize, callback: &mut F)
    where
        F: FnMut(Range<usize>, Result<char, EscapeError>),
    {
        let tail = chars.as_str();
        println!("tail={tail:?}");
        let first_non_space = tail
            .bytes()
            .position(|b| b != b' ' && b != b'\t' && b != b'\n' && b != b'\r')
            .unwrap_or(tail.len());
        println!("first_non_space={first_non_space:?} start={start:?}", );
        if tail[1..first_non_space].contains('\n') {
            // The +1 accounts for the escaping slash.
            // let end = start + first_non_space + 1;
            // callback(start..end, Err(EscapeError::MultipleSkippedLinesWarning));
        }
        let tail = &tail[first_non_space..];
        println!("tail={tail:?}");
        if let Some(c) = tail.chars().nth(0) {
            // For error reporting, we would like the span to contain the character that was not
            // skipped. The +1 is necessary to account for the leading \ that started the escape.
            // println!("{:?}", '£'.is_whitespace());
            println!("first char is {c:?}");
            let end = start + first_non_space + c.len_utf8() + 1;
            println!("end is {end:?}");
            if c.is_whitespace() {
                println!("{c:?} is whitespace, err range is {:?}", start..end);
                callback(start..end, Err(EscapeError::UnskippedWhitespaceWarning));
            }
        }
        *chars = tail.chars();
    }
}

fn main() {
    unescape_str_or_byte_str("\\\n£",  &mut |_range, result| {
        eprintln!("cb={result:?}", );
    });
}

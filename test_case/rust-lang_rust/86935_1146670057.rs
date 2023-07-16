rust
use std::ops::Range;

type Type<T> = T;

fn main() {
    let Type::<<Vec<Range<u8>> as IntoIterator>::Item> {start, end} =
        Type::<<Vec<Range<u8>> as IntoIterator>::Item> {start: 3, end: 5};
    
    assert_eq!(start, 3);
    assert_eq!(end, 5);
}

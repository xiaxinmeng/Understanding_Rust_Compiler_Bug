 rust
struct ByteIndex {
     priv idx: uint
}
impl ByteIndex {
    /* unsafe? */ fn new(idx: uint) -> ByteIndex { idx: idx }
}

fn slice(&'a str, from: ByteIndex, to: ByteIndex) -> &'a str { ... }

fn find_char(&'a str, c: char) -> Option<ByteIndex> { ... }

struct CharRange { 
    ch: char,
    next: ByteIndex
}

fn char_range_at(&str, idx: ByteIndex) -> CharRange { ... }

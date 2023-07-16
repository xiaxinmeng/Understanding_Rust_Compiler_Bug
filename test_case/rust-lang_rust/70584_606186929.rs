rust
pub struct Slice<T: 'static>(&'static [T]);

static CONTENT: &'static [u8] = b"";

pub static CONTENT_MAP: Slice<&'static [u8]> = {
    Slice(&[
        CONTENT,
    ])
};

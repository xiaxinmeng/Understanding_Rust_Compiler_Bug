rust
use std::ops::Deref;

fn assert_TextSized<T: TextSized>() {}

/// Text-like structures that have a text size.
pub trait TextSized: Copy {
    /// The size of this text-alike.
    fn text_size(self) -> usize;
}

impl TextSized for &'_ str {
    fn text_size(self) -> usize {
        self.len()
    }
}

impl<D: Deref> TextSized for &'_ D
where for<'a> &'a D::Target: TextSized
{
    fn text_size(self) -> usize {
        self.deref().text_size()
    }
}

const _: fn() = || {
    assert_TextSized::<&()>();
};

 rust
// this is a tuple struct because clients only construct these and
// we like the tuple struct construction form
pub struct StringSeq<'a>(&'a mut String);

impl<'a> StringSeq<'a> {
    fn inner_mut(&mut self) -> &mut String {
        let StringSeq(&ref mut s) = *self;
        s
    }

    fn inner(&self) -> &String {
        let StringSeq(&ref s) = *self;
        s
    }
}

impl<'a> MutableSeq<char> for StringSeq<'a> {
    #[inline]
    fn push(&mut self, value: char) {
        self.inner_mut().push_char(value)
    }

    #[inline]
    fn pop(&mut self) -> Option<char> {
        self.inner_mut().pop_char()
    }
}

impl<'a> Mutable for StringSeq<'a> {
    #[inline]
    fn clear(&mut self) {
        self.inner_mut().clear()
    }
}

impl<'a> Collection for StringSeq<'a> {
    #[inline]
    fn len(&self) -> uint {
        self.inner().len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.inner().is_empty()
    }
}

fn replace_last<T, S: MutableSeq<T>>(seq: &mut S, val: T) {
    if seq.pop().is_some() {
        seq.push(val)
    }
}

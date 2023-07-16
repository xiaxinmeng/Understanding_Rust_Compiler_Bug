rust
struct CharSearcherInner<'a> {
    haystack: &'a str,
    needle: [u8; 4],
    needle_len: usize,
    range: ::ops::Range<usize>
}

unsafe impl<'a> Searcher<'a> for CharSearcherInner<'a> {
    fn next(&mut self) -> SearchStep {
        unsafe {
            let haystack = self.haystack.get_unchecked(self.range.clone()).as_bytes();
            let needle = self.needle.get_unchecked(..self.needle_len);
            if haystack.is_empty() {
                SearchStep::Done
            } else {
                let start = self.range.start;
                if haystack.starts_with(needle) {
                    self.range.start += self.needle_len;
                    SearchStep::Match(start, self.range.start)
                } else {
                    let leading_ones = (!haystack[0]).leading_zeros();
                    if leading_ones == 0 {
                        self.range.start += 1;
                    } else {
                        self.range.start += leading_ones as usize;
                    }
                    SearchStep::Reject(start, self.range.start)
                }
            }
        }
    }
}

rust
type SizeHint = (usize, Option<usize>);
struct OverrideSizeHint<I>(I, SizeHint);
extension_trait! {
    <I: Iterator> SizeHintExt<I> for I {
        fn override_size_hint(self, hint: SizeHint) -> OverrideSizeHint<I> {
            OverrideSizeHint(self, hint)
        }
    }
}
impl<I: Iterator> Iterator for OverrideSizeHint<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> SizeHint {
        self.1
    }
}

fn collect_hint(ratio: u8, chars: &[char]) -> Vec<char> {
    chars.iter().enumerate().filter_map(|(i, c)| {
        if filter(ratio, i) {
            Some(*c)
        } else {
            None
        }
    }).override_size_hint((chars.len(), Some(chars.len()))).collect()
}

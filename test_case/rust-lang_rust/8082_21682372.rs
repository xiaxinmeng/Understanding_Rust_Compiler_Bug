 rust
type StrCharIterator<'self> = MapIterator<'self, (uint, char), char, StrCharOffsetIterator<'self>>;

fn iter() -> StrCharIterator<'self> {
    self.char_offset_iter().transform(|(_, c)| c)
}

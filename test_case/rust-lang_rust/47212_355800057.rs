rust
fn to_words<'a>() -> Option<Box<Iterator<Item = &'a str> + 'a>> {
    xxx().map(|x| Box::new(x.split(' ')) as Box<Iterator<Item = _>>)
}

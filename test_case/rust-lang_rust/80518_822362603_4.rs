rust
pub fn munge<'b, Data>(&'b mut self, data: Data) -> impl Iterator<Item=u8> + 'b
where
    Data: IntoIterator,
    Data::Item: Borrow<u8>,
    Data::IntoIter: 'b,
{
    let key = &mut self.key;
    let index = &mut self.index;
    data.into_iter().map(move |b| *b.borrow() ^ next_key(&*key, index))
}

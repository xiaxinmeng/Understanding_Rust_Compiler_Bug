
pub trait Iterator {
    type Item;
    
    // Required method
    fn next(&mut self) -> Option<Self::Item>;

    // Provided methods
    fn next_chunk<const N: usize>(
        &mut self
    ) -> Result<[Self::Item; N], IntoIter<Self::Item, N>>
    fn size_hint(&self) -> (usize, Option<usize>)
    fn count(self) -> usize
    fn last(self) -> Option<Self::Item>
    fn advance_by(&mut self, n: usize) -> Result<(), usize>
    ...
}

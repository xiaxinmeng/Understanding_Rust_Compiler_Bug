 rust
struct ChunkIter<T>
    where T: Iterator,
{
    iter: T,
}

struct Chunks<T>
    where T: Iterator,
{
    chunk_iter: ChunkIter<T>
}

impl<T> Chunks<T>
    where T: Iterator,
{
    fn new(iter: T) -> Chunks<T> { unimplemented!() }
}

struct Map<T, F>
    where T: Iterator,
{
    iter: Chunks<T>,
    f: F
}

impl<T> Chunks<T>
    where T: Iterator,
{
    fn map<B, F>(self, f: F) -> Map<T, F>
        where F: FnMut(&mut ChunkIter<T>) -> B,
    { unimplemented!() }
}

impl<T> Iterator for ChunkIter<T>
    where T: Iterator,
{
    type Item = T::Item;
    fn next(&mut self) -> Option<Self::Item> { unimplemented!() }
}

fn main() {
    let vin = vec![1];
    let c = Chunks::new(vin.iter());
    let v = c.map(|i/*: &mut ChunkIter<std::slice::Iter<_>>*/| {
        i.count()
    });
}

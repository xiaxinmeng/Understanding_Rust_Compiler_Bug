rust
impl InfiniteIterator for Cycle<T: NonEmptyIterator>

impl EmptyIterator for Cycle<T: EmptyIterator>

impl EmptyIterator for Chain<A: EmptyIterator, B: EmptyIterator>

impl InfiniteIterator for Chain<A: Iterator, B: InfiniteIterator>

impl InfiniteIterator for Chain<A: InfiniteIterator, B: Iterator>

impl InfiniteIterator for Zip<A: InfiniteIterator, B: InfiniteIterator>

impl EmptyIterator for Zip<A: EmptyIterator, B: EmptyIterator> 

impl InfiniteIterator for Flatten<I: IntoIterator> where I::IntoIter: InfiniteIterator

impl InfiniteIterator for Flatten<I: IntoIterator<Item = T>, T> where T: InfiniteIterator, I::IntoIter: NonEmptyIterator

impl EmptyIterator for Flatten<I: IntoIterator> where I::IntoIter: EmptyIterator

impl EmptyIterator for Flatten<I: IntoIterator> where I::Item: EmptyIterator

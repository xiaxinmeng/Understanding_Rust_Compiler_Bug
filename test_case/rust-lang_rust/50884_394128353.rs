
let mut iter = some_iter();
iter.next().map(|first| first + iter.sum()); // requires U: Add<T>

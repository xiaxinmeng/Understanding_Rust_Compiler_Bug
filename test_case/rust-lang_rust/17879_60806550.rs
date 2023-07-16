
error: the trait `core::iter::FromIterator<(int,int)>` is not implemented for the type `smallintmap::SmallIntMap<int>`

let mut map: SmallIntMap<int> = xs.iter().map(|&x| x).collect();

rust
    use std::ops::Bound::*;
    let mut iter = map.range::<[_], _>((Included(key), Unbounded));
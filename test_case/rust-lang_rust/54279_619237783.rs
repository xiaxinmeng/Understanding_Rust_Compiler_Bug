rust
> let mut slice = [1, 1, 2, 3];
> let new = slice.dedup_in_place();
> //  one may use slice, ignore new and refer to “removed” elements
> 
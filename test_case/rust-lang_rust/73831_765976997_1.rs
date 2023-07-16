rust
> slice.partition_point(pred)
>
> // equivalent to
> slice.binary_search_by(|x| if pred(x) { Less } else { Greater }).unwrap_err()
> 
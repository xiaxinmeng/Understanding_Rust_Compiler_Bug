rust
> #![feature(slice_partition_dedup)]
>
> let mut slice = ["foo", "Foo", "BAZ", "Bar", "bar", "baz", "BAZ"];
>
> let (dedup, duplicates) = slice.partition_dedup_by(|a, b| a.eq_ignore_ascii_case(b));
>
> assert_eq!(dedup, ["foo", "BAZ", "Bar", "baz"]);
> assert_eq!(duplicates, ["bar", "Foo", "BAZ"]);
> 
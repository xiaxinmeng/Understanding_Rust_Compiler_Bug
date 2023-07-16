
> strings.into_iter().for_each(|string| {
>     let value = map.entry(string).or_insert(0);
>     *value += 1;
> });
> 
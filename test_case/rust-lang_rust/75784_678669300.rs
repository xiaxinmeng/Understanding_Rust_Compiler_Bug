
> let range = 0..100;
> let a = repeat(3);
> range.zip(a).flat_map(|(i, j)| {
>     once(i).chain(once(j))
> });
> 
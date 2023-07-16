
strings.into_iter().for_each(|string| {
    *map.entry(string).or_insert(0) += 1;
});

 rust
// Two types
let col: Collection<_> = my_iter_of_results.collect::<Result<_, _>>()?;
// or lots of angle brackets smashed together.
let col = my_iter_of_results.collect::<Result<Collection<_>, _>>()?;

// versus
let col: Collection<_> = my_iter_of_results.collect()?;

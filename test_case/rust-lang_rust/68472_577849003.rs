rust
fn consume(iter: &mut impl Iterator<Item = i32>) {
    // Calls the optimized version
    iter.for_each(|x| println!("{}", x));

    // Calls the default
    (&mut iter).for_each(|x| println!("{}", x));

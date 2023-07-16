
let mut generator = Box::pin(move || {
    // println!("{:?}", stats);
    stats.total_items += 1;
    yield 2;
});

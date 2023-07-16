rust
fn print_items<I>(mut iter: I)
where
	I: LendingIterator,
	for<'a where I: 'a> I::Item<'a>: Debug,

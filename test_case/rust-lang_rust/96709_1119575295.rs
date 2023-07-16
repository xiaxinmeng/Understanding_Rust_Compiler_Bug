rust
pub trait LendingIteratorLifetime<'this>
where
	Self: 'this,
{
	type Item;
}

pub trait LendingIterator: for<'this> LendingIteratorLifetime<'this> {
	fn next(&mut self) -> Option<<Self as LendingIteratorLifetime<'_>>::Item>;
}

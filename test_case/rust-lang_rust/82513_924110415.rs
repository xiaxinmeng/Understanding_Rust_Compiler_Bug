
impl<T: Trait> A<T>
where
	(T::AccountId, T::AccountId): Default,
{
	fn foo() -> Self {
		A::_C(Default::default())
	}
}
